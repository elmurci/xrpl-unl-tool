use crate::crypto::verify_signature;
use crate::manifest::serialize_manifest_data;
use crate::structs::{DecodedManifest, Validator};
use crate::time::get_timestamp;
use crate::enums::Version;
use anyhow::{anyhow, Result};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use color_eyre::owo_colors::OwoColorize;
use sha2::{Digest, Sha256, Sha512};
use std::fs;
use std::fs::File;
use std::io::prelude::*;

pub fn generate_vl_file(content: &str, version: u8) -> Result<()> {
    let mut file = File::create(format!("generated_vl_v{}.json.{}", version, get_timestamp()))?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn sha512_first_half(message: &[u8]) -> Result<Vec<u8>> {
    let mut hasher = Sha512::new();
    hasher.update(message);
    let result = hasher.finalize();
    Ok(result[..32].to_vec())
}

pub fn double_sha256(hex_str: &str) -> Vec<u8> {
    let bin = hex::decode(hex_str).expect("Invalid hex string");
    let hash = Sha256::digest(&bin);
    let hash2 = Sha256::digest(hash);
    hash2.to_vec()
}

pub fn hex_to_base58(hex_string: &str) -> Result<String> {
    let payload_str = format!("1C{}", hex_string);
    let payload_unhex = hex::decode(&payload_str).expect("Invalid hex string");
    let checksum = &double_sha256(&payload_str)[..4];
    let mut payload_with_checksum = payload_unhex.clone();
    payload_with_checksum.extend_from_slice(checksum);
    Ok(base58_encode(payload_with_checksum))
}

pub fn get_manifests(file_path: &str) -> Result<Vec<String>> {
    let contents = fs::read_to_string(file_path)?;
    let lines: Vec<String> = contents.split("\n").map(|s: &str| s.to_string()).collect();
    Ok(lines)
}

pub fn base58_to_hex(bae58_string: &str) -> String {
    let decb58 = base58_decode(Version::NodePublic, bae58_string).expect("Invalid base58 string");
    hex::encode(decb58)
}

pub fn bytes_to_base58(bytes: &[u8]) -> Result<String> {
    hex_to_base58(&hex::encode(bytes))
}

pub fn base58_encode<I: AsRef<[u8]>>(input: I) -> String {
    bs58::encode(input)
        .with_alphabet(bs58::Alphabet::RIPPLE)
        .into_string()
}

pub fn base58_decode<I: AsRef<[u8]>>(version: Version, input: I) -> bs58::decode::Result<Vec<u8>> {
    bs58::decode(input)
        .with_alphabet(bs58::Alphabet::RIPPLE)
        .with_check(Some(version.value()))
        .into_vec()
        .map(|mut vec| {
            let _ = vec.remove(0);
            vec
        })
}

pub fn get_tick_or_cross(is_valid: bool) -> String {
    if is_valid {
        "✓".green().to_string()
    } else {
        "x".red().to_string()
    }
}

pub fn get_key_bytes(key: &str) -> Result<Vec<u8>> {
    if key.len() >= 64 {
        Ok(base58_decode(
            Version::NodePublic,
            hex_to_base58(key).unwrap().as_str(),
        )?)
    } else if key.len() != 33 {
        Ok(base58_decode(Version::NodePublic, key)?)
    } else {
        Ok(key.as_bytes().to_vec())
    }
}

pub fn verify_blob(blob: String, public_key: String, signature: String) -> Result<bool> {
    Ok(
        verify_signature(
            &public_key,
            BASE64_STANDARD.decode(blob)?.as_slice(),
            &signature,
        )
    )
}

pub fn verify_manifest(validator_manifest: DecodedManifest) -> Result<DecodedManifest> {
    let mut manifest = validator_manifest.clone();
    let payload = serialize_manifest_data(&manifest)?;

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

    if !manifest_master_validation || !manifest_signing_validation {
        return Err(anyhow!("Could not verify manifest, either manifest_master_validation or manifest_signing_validation failed."));
    }

    manifest.verification = true;

    Ok(manifest)
}

pub fn print_validators_summary(mut validators: Vec<Validator>) -> Result<()> {
    for validator in validators.iter_mut() {
        if let Some(validator_manifest) = &validator.decoded_manifest {
            
            let validator_validation = verify_manifest(validator_manifest.clone())?;
            
            println!(
                "Validator: {} ({}) | Verification: {} | {}",
                &validator.validation_public_key,
                hex_to_base58(&validator.validation_public_key)?,
                get_tick_or_cross(validator_validation.verification),
                validator_manifest.clone().domain.unwrap_or("".to_string())
            );
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_tick_or_cross_valid() {
        let result = get_tick_or_cross(true);
        let expected = "✓".green().to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_tick_or_cross_invalid() {
        let result = get_tick_or_cross(false);
        let expected = "x".red().to_string();
        assert_eq!(result, expected);
    }
}