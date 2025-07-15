use miette::IntoDiagnostic;
use serde::{self, Deserialize, Serialize};

use super::Action;

#[derive(Deserialize, Serialize)]
pub struct ChangePublicKey {
    #[serde(rename = "publicKey")]
    pub public_key: String,
}

impl Action for ChangePublicKey {
    fn action(&self) -> String {
        "change-public-keys".to_owned()
    }

    fn payload(&self) -> miette::Result<String> {
        serde_json::to_string(self).into_diagnostic()
    }

    fn needs_encryption(&self) -> bool {
        false
    }
}
