use clap::Subcommand;
use anyhow::{anyhow, Result};

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Loads and verifies a given Validation List. It accepts a file path or a url. Example: `./xrpl-vl-tool load {url_or_file_path}`
    Load { arg: Option<String> },
    /// Produces and signs a Validation List
    Sign { arg: Option<Vec<String>> },
    /// Encodes a manifest
    EncodeManifest { arg: Option<Vec<String>> },
    /// Decodes a manifest
    DecodeManifest { arg: Option<String> },
}

#[derive(Debug)]
pub enum SecretProvider {
    Aws,
    Vault,
}

impl SecretProvider {
    pub fn as_str(&self) -> &'static str {
        match self {
            SecretProvider::Aws => "aws",
            SecretProvider::Vault => "vault"
        }
    }
    pub fn from_str(input: &str) -> Result<SecretProvider> {
        match input {
            "aws"  => Ok(SecretProvider::Aws),
            "vault"  => Ok(SecretProvider::Vault),
            _      => Err(anyhow!("Could not parse secret provider value")),
        }
    }
}

#[derive(Debug)]
pub enum Version {
    // None,
    NodePublic,
    // NodePrivate,
    // AccountID,
    // AccountPublic,
    // AccountSecret,
    // // FamilyGenerator,
    // FamilySeed,
}

impl Version {
    pub fn value(&self) -> u8 {
        match *self {
            Version::NodePublic => 28,
            // Version::NodePrivate => 32,
            // Version::AccountID => 0,
            // Version::AccountPublic => 35,
            // Version::AccountSecret => 34,
            // Version::FamilySeed => 33,
        }
    }
}

#[repr(u16)]
#[derive(Debug)]
pub enum ManifestField {
    Sequence = 0x24,
    MasterPublicKey = 0x71,
    SigningPublicKey = 0x73,
    Signature = 0x76,
    Domain = 0x77,
    MasterSignature = 0x7012,
}

impl ManifestField {
    pub fn from_value(input: &u16) -> Result<ManifestField> {
        match input {
            0x24  => Ok(ManifestField::Sequence),
            0x71  => Ok(ManifestField::MasterPublicKey),
            0x73  => Ok(ManifestField::SigningPublicKey),
            0x76  => Ok(ManifestField::Signature),
            0x77  => Ok(ManifestField::Domain),
            0x7012  => Ok(ManifestField::MasterSignature),
            _      => Err(anyhow!("Could not parse manifest field")),
        }
    }
}