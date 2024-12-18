use anstream::println;
use anyhow::{anyhow, Result};
use clap::Parser;
use color_eyre::owo_colors::OwoColorize;
use xrpl_vl_tool::enums::{Commands, SecretProvider};
use xrpl_vl_tool::manifest::{decode_manifest, encode_manifest};
use xrpl_vl_tool::time::{convert_to_human_time, convert_to_unix_time};
use xrpl_vl_tool::vl::{load_vl, sign_vl, verify_vl};
use xrpl_vl_tool::structs::{Cli, DecodedManifest};
use xrpl_vl_tool::util::{
    generate_vl_file, get_tick_or_cross, print_validators_summary
};

#[tokio::main]
async fn main() -> Result<()> {

    let cli = Cli::parse();

    match &cli.command {
        Commands::Load { arg } => {
            let Some(url_or_file) = arg else {
                return Err(anyhow!("No URL or file was passed"));
            };

            let vl = load_vl(url_or_file).await?;
            let verified_vl = verify_vl(vl)?;
  
            if verified_vl.version == 1 {
                // UNL Summary
                let decoded_blob = verified_vl.decoded_blob.clone().unwrap();
                let expiration_unix_timestamp = convert_to_unix_time(decoded_blob.expiration);
                println!("\nThere are {} validators in this VL. Sequence is: {} | Blob Signature: {} | Manifest Signature: {} | Expires: {} | Version: 1 \n", decoded_blob.validators.len().green(), decoded_blob.sequence.green(), get_tick_or_cross(verified_vl.blob_verification.expect("Could not get blob verification")), get_tick_or_cross(verified_vl.manifest.verification), convert_to_human_time(expiration_unix_timestamp));
                // Validators
                let _ = print_validators_summary(decoded_blob.validators);
            } else {
                let decoded_blobs_v2 = verified_vl.blobs_v2.clone().expect("Could not get decoded blobs v2");
                // Summary
                println!("\nThere are {} UNL's in this Validators List | Version 2 | Manifest Signature: {}\n", decoded_blobs_v2.len(), get_tick_or_cross(verified_vl.manifest.verification));
                for (index, blob_v2) in decoded_blobs_v2.iter().enumerate() {
                    let decoded_blob = blob_v2.clone().decoded_blob.expect("Could not get decoded blob");
                    let expiration_unix_timestamp = convert_to_unix_time(decoded_blob.expiration);
                    let effective_unix_timestamp = convert_to_unix_time(decoded_blob.effective.expect("Could not get effective timestamp"));
                    // Summary
                    println!("\n{}) There are {} validators in this VL. Sequence is: {} | Blob Signature: {} | Effective from: {} | Expires: {} \n", index+1, decoded_blob.validators.len().green(), decoded_blob.sequence.green(), get_tick_or_cross(blob_v2.blob_verification.expect("Could not get blob verification flag")), convert_to_human_time(effective_unix_timestamp), convert_to_human_time(expiration_unix_timestamp));
                    // Validators
                    let _ = print_validators_summary(decoded_blob.validators);
                }
            }
        }
        Commands::Sign { arg } => {
            let Some(params) = arg else {
                return Err(anyhow!("No URL or file was passed"));
            };

            if params.len() < 7 {
                return Err(anyhow!("List of parameters: version, manifest, manifests, sequence, expiration_in_days, secret_provider, secret_id, effective_date (for v2), effective_time (for v2) and v2_vl_file(optional)."));
            }

            let version = params[0].parse::<u8>()?;
            let manifest = params[1].clone();
            let manifests_file = params[2].clone();
            let sequence = params[3].parse::<u32>()?;
            let expiration_in_days = params[4].parse::<u16>()?;
            let secret_provider = SecretProvider::from_str(&params[5].clone())?;
            let secret_name = params[6].clone();
            let effective = if version == 2 {
                if params.len() > 8 {
                    Some(format!("{}{}", params[7].clone(), params[8].clone()))
                } else {
                    return Err(anyhow!("Please specify a valid effective date and time"));
                }
            } else {
                None
            };
            let v2_vl_file = if params.len() > 9 {
                Some(params[9].clone())
            } else {
                None
            };

            let vl = sign_vl(
                version,
                manifest,
                manifests_file,
                sequence,
                expiration_in_days,
                secret_provider,
                secret_name,
                effective,
                v2_vl_file,
            ).await?;

            let vl_content = &serde_json::to_string(&vl)?;
            let file = generate_vl_file(vl_content, version).is_ok();
            println!("Validators List v{} file generated {}", version, get_tick_or_cross(file));
         },
         Commands::EncodeManifest { arg } => {
            let Some(params) = arg else {
                return Err(anyhow!("No parameters passed"));
            };

            if params.len() < 5 {
                return Err(anyhow!("List of parameters: sequence, master public key, signing public key, signature, master signature, domain (optional)."));
            }

            let sequence = params[0].parse::<u32>()?;
            let master_public_key = params[1].clone();
            let signing_public_key = params[2].clone();
            let signature = params[3].clone();
            let master_signature = params[4].clone();
            let domain = if params.len() > 5 {
                Some(params[5].clone())
            } else {
                None
            };

            let encoded_manifest = encode_manifest(
                sequence,
                master_public_key,
                signing_public_key,
                signature,
                master_signature,
                domain
            )?;

            println!("\n Encoded manifest: \n\n {}", encoded_manifest);
         },
         Commands::DecodeManifest { arg } => {
            let Some(manifest) = arg else {
                return Err(anyhow!("No manifest passed"));
            };

            let decoded_manifest = decode_manifest(&manifest)?;
            
            println!("\n Decoded manifest: \n\n Sequence: {} \n Master Public Key: {} \n Signing Public Key: {} \n Signature: {} \n Master Signature: {} \n Domain: {:?} \n",
            decoded_manifest.sequence,
            decoded_manifest.master_public_key,
            decoded_manifest.signing_public_key,
            decoded_manifest.signature.to_uppercase(),
            decoded_manifest.master_signature.to_uppercase(),
            decoded_manifest.domain,
            );
         }
    }

    Ok(())
}