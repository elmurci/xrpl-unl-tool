use anstream::println;
use anyhow::{anyhow, Result};
use base64::prelude::*;
use chrono::{Duration, Utc};
use clap::Parser;
use color_eyre::owo_colors::OwoColorize;
use crypto::{sign, verify_signature};
use enums::{Commands, SecretProvider, Version};
use manifest::{decode_manifest, serialize_manifest_data};
use secret::get_secret;
use std::collections::HashSet;
use structs::{Cli, DecodedBlob, Unl, Validator};
use time::{convert_to_human_time, convert_to_ripple_time, convert_to_unix_time};
use util::{
    base58_decode, base58_to_hex, decode_unl, generate_unl_file, get_manifests, get_tick_or_cross,
    get_unl, hex_to_base58,
};

mod secret;
mod crypto;
mod enums;
mod manifest;
mod structs;
mod time;
mod util;

#[tokio::main]
async fn main() -> Result<()> {

    let cli = Cli::parse();

    match &cli.command {
        Commands::Load { arg } => {
            let Some(url_or_file) = arg else {
                return Err(anyhow!("No URL or file was passed"));
            };

            let unl = get_unl(url_or_file).await?;
            let decoded_unl = decode_unl(unl.clone())?;

            // Let's panic if the blob is not decodable
            let mut decoded_blob = decoded_unl.decoded_blob.expect("Could not decode blob");

            // Or the manifest is incorrect
            let unl_decoded_manifest = decoded_unl
                .decoded_manifest
                .expect("Could not decode manifest");

            let manifest_signin_key = hex::encode(
                base58_decode(
                    enums::Version::NodePublic,
                    &unl_decoded_manifest.signing_public_key,
                )?,
            )
            .to_uppercase();
    
            
            let manifest_verification = verify_signature(
                &manifest_signin_key,
                &serialize_manifest_data(&unl_decoded_manifest)?,
                &unl_decoded_manifest.signature,
            );
            
            let unl_verification = verify_signature(
                &manifest_signin_key,
                &BASE64_STANDARD.decode(&unl.blob)?,
                &unl.signature,
            );
            let expiration_unix_timestamp = convert_to_unix_time(decoded_blob.expiration);
            println!("\nThere are {} validators in this UNL. Sequence is: {} | Manifest: {} | UNL: {} | Expires: {} \n", decoded_blob.validators.len().green(), decoded_blob.sequence.green(), get_tick_or_cross(manifest_verification), get_tick_or_cross(unl_verification), convert_to_human_time(expiration_unix_timestamp));

            for validator in decoded_blob.validators.iter_mut() {
                if let Some(validator_manifest) = &validator.decoded_manifest {
                    let payload = serialize_manifest_data(validator_manifest)?;

                    let manifest_master_validation = verify_signature(
                        &hex::encode(
                            &base58_decode(Version::NodePublic, &validator_manifest.master_public_key)?,
                        )
                        .to_uppercase(),
                        &payload,
                        &validator_manifest.master_signature,
                    );

                    let manifest_signing_validation = verify_signature(
                        &hex::encode(
                            &base58_decode(Version::NodePublic, &validator_manifest.signing_public_key)?,
                        )
                        .to_uppercase(),
                        &payload,
                        &validator_manifest.signature,
                    );

                    println!(
                        "Validator: {} ({}) | Master: {}, Signing: {} | {}",
                        &validator.validation_public_key,
                        hex_to_base58(&validator.validation_public_key)?,
                        get_tick_or_cross(manifest_master_validation),
                        get_tick_or_cross(manifest_signing_validation),
                        validator_manifest.clone().domain.unwrap_or("".to_string())
                    );
                } else {

                }
            }
        }
        Commands::Compare { arg } => {
            let Some(urls_or_files) = arg else {
                return Err(anyhow!("No URL or file was passed"));
            };
            if urls_or_files.len() != 2 {
                return Err(anyhow!("Two URLs or files must be passed"));
            }

            let unl_1_id = &urls_or_files[0];
            let unl_1 = get_unl(unl_1_id).await?;
            let decoded_unl_1 = decode_unl(unl_1.clone())?;

            let unl_2_id = &urls_or_files[1];
            let unl_2 = get_unl(unl_2_id).await?;
            let decoded_unl_2 = decode_unl(unl_2.clone())?;

            let decoded_unl_1_blob = decoded_unl_1.decoded_blob.ok_or_else(|| anyhow!("Could not decode blob 1"))?;
            let decoded_unl_1_validators = decoded_unl_1_blob.validators;
            let decoded_unl_2_blob = decoded_unl_2.decoded_blob.ok_or_else(|| anyhow!("Could not decode blob 2"))?;
            let decoded_unl_2_validators = decoded_unl_2_blob.validators;
            let validators_manifests_1: Vec<String> = decoded_unl_1_validators
                .iter()
                .map(|c| c.manifest.clone())
                .collect();
            
            let validators_manifests_2: Vec<String> = decoded_unl_2_validators
                .iter()
                .map(|c| c.manifest.clone())
                .collect();
            let validators_manifests_1_len = validators_manifests_1.len();
            let validators_manifests_2_len = validators_manifests_2.len();
            let a: HashSet<_> = validators_manifests_1.into_iter().collect();
            let b: HashSet<_> = validators_manifests_2.into_iter().collect();
            let mut a_but_not_b = vec![];
            let mut b_but_not_a = vec![];

            // TODO: think of a more effective way of comparing
            for validator in a.difference(&b) {
                let decoded_manifest =
                    decode_manifest(validator)?;
                a_but_not_b.push(format!(
                    "{} {}",
                    decoded_manifest.master_public_key,
                    decoded_manifest.domain.unwrap_or("".to_string())
                ));
            }

            for validator in b.difference(&a) {
                let decoded_manifest =
                    decode_manifest(validator)?;
                b_but_not_a.push(format!(
                    "{} {}",
                    decoded_manifest.master_public_key,
                    decoded_manifest.domain.unwrap_or("".to_string())
                ));
            }

            if a_but_not_b.is_empty() && b_but_not_a.is_empty() {
                println!(
                    "{} {}",
                    "Both UNLs have the same validators".green(),
                    validators_manifests_1_len.bright_magenta()
                );
            } else {
                println!(
                    "\n {} ({})",
                    unl_1_id.blue(),
                    validators_manifests_1_len.bright_magenta()
                );
                a_but_not_b
                    .iter()
                    .for_each(|c| println!("{}{}", "+".green(), c.green()));
                b_but_not_a
                    .iter()
                    .for_each(|c| println!("{}{}", "-".red(), c.red()));

                println!(
                    "\n {} ({})",
                    unl_2_id.blue(),
                    validators_manifests_2_len.bright_magenta()
                );
                b_but_not_a
                    .iter()
                    .for_each(|c| println!("{}{}", "+".green(), c.green()));
                a_but_not_b
                    .iter()
                    .for_each(|c| println!("{}{}", "-".red(), c.red()));
            }
        }
        Commands::Sign { arg } => {
            let Some(params) = arg else {
                return Err(anyhow!("No URL or file was passed"));
            };

            if params.len() != 6 {
                return Err(anyhow!("Parameters missing: manifest, manifests, sequence, expiration_in_days, secret_provider and secret_id must be passed"));
            }

            let manifest = params[0].clone();
            let manifests = params[1].clone();
            let sequence = params[2].parse::<u32>()?;
            let expiration_in_days = params[3].parse::<u16>()?;
            let secret_provider = SecretProvider::from_str(&params[4].clone());
            let secret_name = params[5].clone();
            let secret = get_secret(secret_provider?, &secret_name).await?;

            if secret.is_none() {
                return Err(anyhow!("No secret was found"));
            }

            let keypair = secret.unwrap();

            let mut unl = Unl::default();
 
            unl.manifest = manifest;
            unl.public_key = keypair.public_key.clone();

            let manifests = get_manifests(&manifests)?;
            let mut validators: Vec<Validator> = vec![];

            for manifest in manifests {
                let decoded_manifest = decode_manifest(&manifest)?;
                let validator = Validator {
                    validation_public_key: base58_to_hex(&decoded_manifest.master_public_key)
                        .to_uppercase(),
                    manifest,
                    decoded_manifest: None,
                };
                validators.push(validator);
            }

            let decoded_blob = DecodedBlob {
                sequence,
                expiration: convert_to_ripple_time(Some(
                    (Utc::now() + Duration::days(expiration_in_days as i64)).timestamp(),
                )),
                validators,
            };

            let decoded_blob_payload = serde_json::to_string(&decoded_blob)?;

            let signature = sign(
                &keypair.public_key,
                &keypair.private_key,
                &decoded_blob_payload,
            );

            unl.signature = signature.clone();
            unl.blob = BASE64_STANDARD.encode(decoded_blob_payload.clone());

            let unl_content = &serde_json::to_string(&unl)?;
            let file = generate_unl_file(unl_content).is_ok();
            println!("{} {}", "UNL file generated", get_tick_or_cross(file));
        }
    }

    Ok(())
}
