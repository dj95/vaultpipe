use miette::IntoDiagnostic;
use serde::{self, Deserialize, Serialize};

use super::Action;

#[derive(Deserialize, Serialize)]
pub struct GetLogins {
    pub action: String,
    pub url: String,
    pub keys: Vec<Key>,
}

#[derive(Deserialize, Serialize)]
pub struct Key {
    pub id: String,
    pub key: String,
}

impl GetLogins {
    pub fn new(url: String, id: String, key: String) -> Self {
        Self {
            action: "get-logins".to_owned(),
            url,
            keys: vec![Key { id, key }],
        }
    }
}

impl Action for GetLogins {
    fn action(&self) -> String {
        "get-logins".to_owned()
    }

    fn payload(&self) -> miette::Result<String> {
        serde_json::to_string(self).into_diagnostic()
    }

    fn needs_encryption(&self) -> bool {
        true
    }
}
