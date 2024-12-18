macro_rules! test_data {($fname:expr) => (
    concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/", $fname) // assumes Linux ('/')!
  )}

mod test {
    use xrpl_vl_tool::vl::{load_vl, verify_vl};

    // VL's that should verify (v1 and v2)

    #[tokio::test]
    async fn should_load_v1_valid_vl() {
        let vl = load_vl(test_data!("vl_v1_1.json")).await.unwrap();
        let verified_vl = verify_vl(vl.clone()).unwrap();
        assert!(verified_vl.version == 1);
        assert!(verified_vl.blobs_v2.is_none());
        for validator in verified_vl.decoded_blob.clone().unwrap().validators {
            assert!(validator.decoded_manifest.unwrap().verification == true);
        }
        assert!(verified_vl.manifest.verification == true);
    }

    #[tokio::test]
    async fn should_load_v2_valid_vl_with_1_unl() {
        let vl = load_vl(test_data!("vl_v2_1.json")).await.unwrap();
        let verified_vl = verify_vl(vl.clone()).unwrap();
        assert!(verified_vl.version == 2);
        assert!(verified_vl.blobs_v2.clone().unwrap().len() == 1);
        assert!(verified_vl.blob.is_none());
        for blob_v2 in verified_vl.blobs_v2.unwrap() {
            assert!(blob_v2.blob_verification.unwrap() == true);
            for validator in blob_v2.decoded_blob.unwrap().validators {
                assert!(validator.decoded_manifest.unwrap().verification == true);
            }
        }
         assert!(verified_vl.manifest.verification == true);
    }

    #[tokio::test]
    async fn should_load_v2_valid_vl_with_2_unl() {
        let vl = load_vl(test_data!("vl_v2_2.json")).await.unwrap();
        let verified_vl = verify_vl(vl.clone()).unwrap();
        assert!(verified_vl.version == 2);
        assert!(verified_vl.blobs_v2.clone().unwrap().len() == 2);
        assert!(verified_vl.blob.is_none());
        for blob_v2 in verified_vl.blobs_v2.unwrap() {
            assert!(blob_v2.blob_verification.unwrap() == true);
            for validator in blob_v2.decoded_blob.unwrap().validators {
                assert!(validator.decoded_manifest.unwrap().verification == true);
            }
        }
        assert!(verified_vl.manifest.verification == true);
    }

    // VL wrong format

    #[tokio::test]
    async fn should_error_v1_invalid_format() {
        assert!(load_vl(test_data!("vl_v1_wrong_format.json")).await.is_err() == true);
    }

    #[tokio::test]
    async fn should_error_v2_invalid_format() {
        assert!(load_vl(test_data!("vl_v2_wrong_format.json")).await.is_err() == true);
    }

    #[tokio::test]
    async fn should_error_v1_invalid_master_manifest() {
        assert!(load_vl(test_data!("vl_v1_wrong_manifest_1.json")).await.is_err() == true);
    }

    #[tokio::test]
    async fn should_error_v1_invalid_validator_manifest() {
        assert!(load_vl(test_data!("vl_v1_wrong_manifest_2.json")).await.is_err() == true);
    }

    #[tokio::test]
    async fn should_error_v1_invalid_blob_format() {
        assert!(load_vl(test_data!("vl_v1_wrong_blob.json")).await.is_err() == true);
    }

    #[tokio::test]
    async fn should_error_v2_invalid_master_manifest() {
        assert!(load_vl(test_data!("vl_v2_wrong_manifest_1.json")).await.is_err() == true);
    }

    #[tokio::test]
    async fn should_error_v2_invalid_validator_manifest() {
        assert!(load_vl(test_data!("vl_v2_wrong_manifest_2.json")).await.is_err() == true);
    }

    #[tokio::test]
    async fn should_error_v2_invalid_blob_format() {
        assert!(load_vl(test_data!("vl_v2_wrong_blob.json")).await.is_err() == true);
    }

    // VL's that shouldn't verify (v1 and v2)

    #[tokio::test]
    async fn should_not_verify_master_signature_v1() {
        let vl = load_vl(test_data!("vl_v1_wrong_master_signature.json")).await.unwrap();
        let verified_vl = verify_vl(vl.clone()).unwrap();
        assert!(verified_vl.manifest.verification == false);
    }

    // Encode / Decode manifest

    // One bad validator 

    // The value is equal to the `master_public_key` in the publisher's manifest.

    // Effective dates

    // Expiration dates
    // If the expiration of a blob is not greater than effective, the blob will be considered malformed.

    // If the manifest is not present in a blobs-v2 array entry, then the top-level manifest will be used when checking the signature. 

    // Optional signatures (!!!)

    // Sign

}