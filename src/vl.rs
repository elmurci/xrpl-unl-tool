use std::fs;

use base64::{prelude::BASE64_STANDARD, Engine};
use anyhow::Result;
use chrono::{Duration, NaiveDateTime, Utc};
use url::Url;
use anyhow::anyhow;

use crate::{crypto::{sign, verify_signature}, enums::{self, SecretProvider}, manifest::{decode_manifest, serialize_manifest_data}, secret::get_secret, structs::{BlobV2, DecodedBlob, DecodedVl, Validator, Vl}, time::convert_to_ripple_time, util::{base58_decode, base58_to_hex, get_manifests, verify_blob, verify_manifest}};

pub async fn get_vl(url_or_file: &str) -> Result<Vl> {
    
    let url = Url::parse(url_or_file);

    let vl: Vl = if url.is_err() {
        serde_json::from_str(&fs::read_to_string(url_or_file)?)?
    } else {
        reqwest::get(url_or_file).await?.json::<Vl>().await?
    };
    Ok(vl)
}

pub async fn load_vl(url_or_file: &str) -> Result<DecodedVl> {
    let vl = get_vl(url_or_file).await?;

    // What version is this VL?
    let parsed_vl = match vl.version {
        1 => {
            decode_vl_v1(&vl)?
        },
        2 => {
            decode_vl_v2(&vl)?
        },
        _ => return Err(anyhow!("Unsupported version")),
    };

    Ok(parsed_vl)  
}

pub fn verify_vl(mut vl: DecodedVl) -> Result<DecodedVl> {

    // Manifest Verification
    let manifest_signin_key = hex::encode(
        base58_decode(
            enums::Version::NodePublic,
            &vl.manifest.signing_public_key,
        )?,
    )
    .to_uppercase();

    let manifest_verification = verify_signature(
        &manifest_signin_key,
        &serialize_manifest_data(&vl.manifest)?,
        &vl.manifest.signature,
    );

    vl.manifest.verification =  manifest_verification;

    // With version 1 there is only one blob
    if vl.version == 1 {
        let mut decoded_blob = vl.decoded_blob.clone().expect("Could not get decoded blob");
        // Blob verification verifies each validator manifest
        for validator in decoded_blob.validators.iter_mut() {
            let verified_validator = verify_manifest(validator.decoded_manifest.clone().expect("Could not get decoded manifest"))?;
            validator.decoded_manifest = Some(verified_validator);
        }
        vl.decoded_blob = Some(decoded_blob);
        let verify_blob = verify_blob(
            vl.blob.clone().expect("msg"),
            base58_to_hex(&vl.manifest.signing_public_key.clone()).to_uppercase(),
            vl.signature.clone().expect("msg")
        );
        vl.blob_verification = Some(verify_blob?);
    } else {
        // With version 2, there might be multiple blobs
        let decoded_blobs_v2 = vl.decoded_blobs_v2.as_mut().expect("Could not get decoded blobs v2");
        for blob_v2 in decoded_blobs_v2.iter_mut() {
            let mut decoded_blob = blob_v2.clone().decoded_blob.expect("Could not get decoded blob");
            // TODO: move to a function
            for validator in decoded_blob.validators.iter_mut() {
                let verified_validator = verify_manifest(validator.decoded_manifest.clone().expect("Could not get decoded manifest"))?;
                validator.decoded_manifest = Some(verified_validator);
            }
            blob_v2.blob_verification = Some(true);
            let verify_blob = verify_blob(
                vl.blob.clone().expect("msg"),
                base58_to_hex(&vl.manifest.signing_public_key.clone()).to_uppercase(),
                vl.signature.clone().expect("msg")
            );
            blob_v2.blob_verification = Some(verify_blob?);
            blob_v2.decoded_blob = Some(decoded_blob);
        }
    }

    Ok(vl)

}

pub async fn sign_vl(
    version: u8,
    manifest: String,
    manifests_file: String,
    sequence: u32,
    expiration_in_days: u16,
    secret_provider: SecretProvider,
    secret_name: String,
    effective: Option<String>,
    v2_vl_file: Option<String>,
) -> Result<Vl> {
    let secret = get_secret(secret_provider, &secret_name).await?;
    if secret.is_none() {
        return Err(anyhow!("No secret was found"));
    }

    let keypair = secret.unwrap();
    
    let mut vl = if v2_vl_file.is_some() {
        get_vl(&v2_vl_file.clone().unwrap()).await?
    } else {
        Vl::default()
    };
    
    let manifests = get_manifests(&manifests_file)?;
    let mut validators: Vec<Validator> = vec![];

    for manifest in manifests {
        let decoded_manifest = decode_manifest(&manifest)?;
        let validator = Validator {
            validation_public_key: base58_to_hex(&decoded_manifest.master_public_key)
                .to_uppercase(),
            manifest: Some(manifest),
            decoded_manifest: None,
        };
        validators.push(validator);
    }

    let expiration_ripple_timestamp =  convert_to_ripple_time(Some(
        (Utc::now() + Duration::days(expiration_in_days as i64)).timestamp(),
    ));

    let effective_ripple_timestamp = if version == 2 {
        let calculated = convert_to_ripple_time(Some(NaiveDateTime::parse_from_str(&effective.expect("Could not get the effective date"), "%Y-%m-%d %H:%M").expect("Could not parse effective timestamp, format is %Y-%m-%d %H:%M").and_utc().timestamp()));
        if expiration_ripple_timestamp > calculated {
            Some(calculated)
        } else {
            return Err(anyhow!("Effective date must be before expiration date"));
        }

        // TODO: can't be in the past or in the same time as another unl in the array
    } else {
        None
    };

    let decoded_blob = DecodedBlob {
        sequence,
        expiration: expiration_ripple_timestamp,
        validators,
        effective: effective_ripple_timestamp
    };

    let decoded_blob_payload = serde_json::to_string(&decoded_blob)?;
    let signature = sign(
        &keypair.public_key,
        &keypair.private_key,
        &decoded_blob_payload,
    );

    // For version 1, we will generate a brand new file each time
    // For version 2, we can do the same but also start from an existing file.
    // The latter approach will add a new item to the blobs_v2 array 

    if version == 1 {
        vl.signature = Some(signature.clone());
        vl.blob = Some(BASE64_STANDARD.encode(decoded_blob_payload.clone()));
    } else {
        if v2_vl_file.is_none() {
            vl.manifest = manifest;
            vl.public_key = keypair.public_key.clone();
            vl.blobs_v2 = Some(vec![]);
        }

        vl.blobs_v2.as_mut().expect("Could not get blobs_v2").push(
            BlobV2 {
                signature,
                manifest: None,
                blob: Some(BASE64_STANDARD.encode(decoded_blob_payload.clone())),
                decoded_blob: None,
                blob_verification: None,
            },
        );
        vl.version = 2;
    }

    Ok(vl)

}

pub fn decode_vl_v1(vl: &Vl) -> Result<DecodedVl> {
    let decoded = BASE64_STANDARD.decode(&vl.blob.clone().expect("Could not decode VL (v1) Blob"))?;
    let mut decoded_blob: DecodedBlob = serde_json::from_str(&String::from_utf8(decoded)?)?;
    for validator in decoded_blob.validators.iter_mut() {
        if validator.manifest.is_some() {
            let manifest = decode_manifest(&validator.manifest.clone().expect("Could not decode manifest"))?;
            validator.decoded_manifest = Some(manifest.clone());
        }
    }
    let decoded_publisher_manifest = Some(decode_manifest(&vl.manifest)?).expect("Could not decode the publisher manifest");
    Ok(
        DecodedVl {
            public_key: vl.public_key.clone(),
            manifest: decoded_publisher_manifest,
            blob: vl.blob.clone(),
            blobs_v2: vl.blobs_v2.clone(),
            decoded_blob: Some(decoded_blob),
            decoded_blobs_v2: None,
            signature: vl.signature.clone(),
            version: vl.version,
            blob_verification: None,
        }
    )
}

pub fn decode_vl_v2(vl: &Vl) -> Result<DecodedVl> {
    let decoded_publisher_manifest = Some(decode_manifest(&vl.manifest)?).expect("Could not decode the publisher manifest");
    let mut blobsv2: Vec<BlobV2> = vec![];
    // For each blob v2
    for blobv2 in vl.blobs_v2.clone().expect("Could not decode VL (v2) Blobs") {
        let decoded = BASE64_STANDARD.decode(&blobv2.blob.expect("Could not decode blob v2"))?;
        let mut decoded_blob: DecodedBlob = serde_json::from_str(&String::from_utf8(decoded)?)?;
        for validator in decoded_blob.validators.iter_mut() {
            if validator.manifest.is_some() {
                let manifest = decode_manifest(&validator.manifest.clone().expect("Could not decode manifest"))?;
                validator.decoded_manifest = Some(manifest.clone());
            }
        }
        blobsv2.push(
            BlobV2 { 
                signature: blobv2.signature,
                manifest: None,
                blob: None,
                decoded_blob: Some(decoded_blob),
                blob_verification: None,
            }
        );
    }
    Ok(
        DecodedVl {
            public_key: vl.public_key.clone(),
            manifest: decoded_publisher_manifest,
            blob: None,
            blobs_v2: vl.blobs_v2.clone(),
            decoded_blob: None,
            decoded_blobs_v2: Some(blobsv2),
            signature: vl.signature.clone(),
            version: vl.version,
            blob_verification: None,
        }
    )
}