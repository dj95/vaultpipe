use miette::IntoDiagnostic;
use serde::{self, Deserialize, Serialize};

use super::Action;

#[derive(Deserialize, Serialize)]
pub struct TestAssociation {
    pub action: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "key")]
    pub key: String,
}

impl TestAssociation {
    pub fn new(id: String, key: String) -> Self {
        Self {
            action: "test-associate".to_owned(),
            id,
            key,
        }
    }
}

impl Action for TestAssociation {
    fn action(&self) -> String {
        "test-associate".to_owned()
    }

    fn payload(&self) -> miette::Result<String> {
        serde_json::to_string(self).into_diagnostic()
    }

    fn needs_encryption(&self) -> bool {
        true
    }
}
