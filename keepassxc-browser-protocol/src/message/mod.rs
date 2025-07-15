use miette::{IntoDiagnostic, Result};
use serde::{Deserialize, Serialize};

pub mod action;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "publicKey", skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    pub nonce: String,
    #[serde(rename = "clientID")]
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

impl Message {
    pub fn json(&self) -> Result<String> {
        serde_json::to_string(self).into_diagnostic()
    }
}
