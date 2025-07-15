use miette::IntoDiagnostic;
use serde::{self, Deserialize, Serialize};

use super::Action;

#[derive(Deserialize, Serialize)]
pub struct Associate {
    pub action: String,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "idKey")]
    pub id_key: String,
}

impl Associate {
    pub fn new(key: String, id_key: String) -> Self {
        Self {
            action: "associate".to_owned(),
            key,
            id_key,
        }
    }
}

impl Action for Associate {
    fn action(&self) -> String {
        "associate".to_owned()
    }

    fn payload(&self) -> miette::Result<String> {
        serde_json::to_string(self).into_diagnostic()
    }

    fn needs_encryption(&self) -> bool {
        true
    }
}
