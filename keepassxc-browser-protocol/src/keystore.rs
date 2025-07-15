use crypto_box::PublicKey;
use keyring::Entry;
use miette::{IntoDiagnostic, Result};

pub fn get_public_key() -> Result<PublicKey> {
    let entry = Entry::new("keepassxc_rs", "identity_key").into_diagnostic()?;
    let raw_value = entry.get_secret().into_diagnostic()?;
    PublicKey::from_slice(&raw_value).into_diagnostic()
}

pub fn set_public_key(public_key: PublicKey) -> Result<()> {
    let entry = Entry::new("keepassxc_rs", "identity_key").into_diagnostic()?;
    entry.set_secret(public_key.as_bytes()).into_diagnostic()?;
    Ok(())
}

pub fn get_client_id() -> Result<Option<String>> {
    let entry = Entry::new("keepassxc_rs", "client_id").into_diagnostic()?;

    match entry.get_secret() {
        Ok(value) => Ok(Some(String::from_utf8(value).into_diagnostic()?)),
        Err(_) => Ok(None),
    }
}

pub fn set_client_id(identity: String) -> Result<()> {
    let entry = Entry::new("keepassxc_rs", "client_id").into_diagnostic()?;
    entry.set_secret(identity.as_bytes()).into_diagnostic()?;
    Ok(())
}
