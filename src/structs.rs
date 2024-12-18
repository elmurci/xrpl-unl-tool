use clap::Parser;
use ed25519_dalek::{Signer, Verifier};
use serde::{Deserialize, Serialize};

use crate::enums::Commands;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vl {
    pub public_key: String,
    pub manifest: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob: Option<String>, // Only for v1
    #[serde(alias = "blobs-v2", skip_serializing_if = "Option::is_none")]
    pub blobs_v2: Option<Vec<BlobV2>>, // Only for v2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>, // Only for v1
    pub version: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DecodedVl {
    pub public_key: String,
    pub manifest: DecodedManifest,
    pub blob: Option<String>, // Only for v1
    pub blobs_v2: Option<Vec<BlobV2>>, // Only for v2
    pub decoded_blob: Option<DecodedBlob>, // Only for v1
    pub decoded_blobs_v2: Option<Vec<BlobV2>>, // Only for v2
    pub signature: Option<String>, // Only for v1
    pub version: u8,
    pub blob_verification: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DecodedBlob {
    pub sequence: u32,
    pub expiration: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective: Option<i64>,
    pub validators: Vec<Validator>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlobV2 {
    pub signature: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decoded_blob: Option<DecodedBlob>,
    pub blob_verification: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Validator {
    pub validation_public_key: String,
    pub manifest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decoded_manifest: Option<DecodedManifest>,
}

impl Default for Vl {
    fn default() -> Vl {
        Vl {
            public_key: "".to_string(),
            manifest: "".to_string(),
            blob: None,
            blobs_v2: None,
            signature: None,
            version: 1,
        }
    }
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DecodedManifest {
    pub sequence: u32,
    pub master_public_key: String,
    pub signature: String,
    pub signing_public_key: String,
    pub master_signature: String,
    pub domain: Option<String>,
    pub verification: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Secret {
    pub public_key: String,
    pub private_key: String,
}

pub struct Ed25519Signer<S>
where
    S: Signer<ed25519::Signature>,
{
    pub signing_key: S,
}

impl<S> Ed25519Signer<S>
where
    S: Signer<ed25519::Signature>,
{
    pub fn sign(&self, str: &str) -> ed25519::Signature {
        self.signing_key.sign(str.as_bytes())
    }
}

pub struct Ed25519Verifier<V> {
    pub verifying_key: V,
}

impl<V> Ed25519Verifier<V>
where
    V: Verifier<ed25519::Signature>,
{
    pub fn verify(
        &self,
        payload: &[u8],
        signature: &ed25519::Signature,
    ) -> Result<(), ed25519::Error> {
        self.verifying_key.verify(payload, signature)
    }
}