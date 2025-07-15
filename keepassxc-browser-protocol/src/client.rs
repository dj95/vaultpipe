use crypto_box::{
    KEY_SIZE, Nonce, PublicKey, SalsaBox, SecretKey,
    aead::{Aead, AeadCore, OsRng, Payload},
};
use miette::{IntoDiagnostic, Result, bail, miette};

use base64::prelude::*;
use serde_json::Value;

use std::{
    io::{Read, Write},
    os::unix::net::{self, UnixStream},
    sync::Arc,
};

use crate::{
    keystore::{get_client_id, get_public_key, set_client_id, set_public_key},
    message::{self, action},
    socket,
};

pub struct Client {
    socket: net::UnixStream,
    client_id: String,
    public_key: Option<PublicKey>,
    peer_public_key: Option<PublicKey>,
    crypto_box: Option<Arc<SalsaBox>>,
    id_public_key: PublicKey,
    identity: Option<String>,
}

impl Client {
    pub fn new() -> Result<Self> {
        // TODO: write a connection wrapper to also support windows
        let socket = UnixStream::connect(&socket::path()?).into_diagnostic()?;

        let id_public_key = match get_public_key() {
            Ok(id_pk) => id_pk,
            Err(_) => {
                let secret_key = SecretKey::generate(&mut OsRng);
                set_public_key(secret_key.public_key())?;
                secret_key.public_key()
            }
        };
        let identity = get_client_id()?;

        Ok(Self {
            socket,
            client_id: "foo".to_owned(),
            crypto_box: None,
            public_key: None,
            peer_public_key: None,
            id_public_key,
            identity,
        })
    }

    pub fn handshake(&mut self) -> Result<()> {
        let secret_key = SecretKey::generate(&mut OsRng);
        self.public_key = Some(secret_key.public_key());

        let action = action::change_public_key::ChangePublicKey {
            public_key: BASE64_STANDARD.encode(secret_key.public_key().as_bytes()),
        };

        let res = self.send_message(action)?;
        tracing::debug!("{:?}", res);

        if res["success"] != "true" {
            bail!("received invalid response from keepass")
        }

        let decoded_key = BASE64_STANDARD
            .decode(res["publicKey"].as_str().unwrap())
            .into_diagnostic()?;
        if decoded_key.len() != KEY_SIZE {
            bail!("received invalid key");
        }

        self.peer_public_key = Some(PublicKey::from_slice(&decoded_key).into_diagnostic()?);

        let crypto_box = SalsaBox::new(&self.peer_public_key.clone().unwrap(), &secret_key);
        self.crypto_box = Some(Arc::new(crypto_box));

        Ok(())
    }

    pub fn associate(&mut self) -> Result<()> {
        let id_key = SecretKey::generate(&mut OsRng);

        let action = action::associate::Associate::new(
            BASE64_STANDARD.encode(self.public_key.clone().unwrap().as_bytes()),
            BASE64_STANDARD.encode(id_key.public_key().as_bytes()),
        );

        let res = self.send_message(action)?;
        tracing::debug!("{:?}", res);

        self.identity = Some(res["id"].as_str().unwrap().to_owned());

        // save identity in keystore
        set_client_id(self.identity.clone().unwrap())?;
        set_public_key(id_key.public_key())?;

        Ok(())
    }

    pub fn get_logins(&mut self, url: String) -> Result<()> {
        let identity = match self.identity.clone() {
            Some(id) => id,
            None => bail!("no identity"),
        };

        let action = action::get_logins::GetLogins::new(
            url,
            identity,
            BASE64_STANDARD.encode(self.id_public_key.as_bytes()),
        );

        let res = self.send_message(action)?;
        tracing::debug!("{:?}", res);
        
        // TODO: return result

        Ok(())
    }

    pub fn test_association(&mut self) -> Result<()> {
        let identity = match self.identity.clone() {
            Some(id) => id,
            None => bail!("no identity"),
        };

        let action = action::test_association::TestAssociation::new(
            identity,
            BASE64_STANDARD.encode(self.id_public_key.as_bytes()),
        );

        let res = self.send_message(action)?;
        tracing::debug!("{:?}", res);

        Ok(())
    }

    fn send_message(&mut self, payload: impl action::Action) -> Result<Value> {
        let nonce = SalsaBox::generate_nonce(&mut OsRng);
        let msg = match payload.needs_encryption() {
            true => self
                .crypto_box
                .clone()
                .unwrap()
                .encrypt(&nonce, payload.payload()?.as_bytes())
                .map(|res| BASE64_STANDARD.encode(res))
                .map_err(|e| miette!(e))?,
            false => payload.payload()?,
        };

        let mut message = message::Message {
            action: payload.action(),
            message: Some(msg),
            public_key: None,
            nonce: BASE64_STANDARD.encode(nonce),
            client_id: self.client_id.clone(),
            request_id: None,
        };

        tracing::debug!("{:?}", payload.payload());

        if payload.action() == "change-public-keys" {
            message.message = None;
            message.public_key = Some(BASE64_STANDARD.encode(self.public_key.clone().unwrap()));
        }

        tracing::debug!("{:?}", message.json()?);

        // send message
        self.socket
            .write(message.json()?.as_bytes())
            .into_diagnostic()?;

        // receive response
        let mut buf = [0; 4096];
        self.socket.read(&mut buf[..]).into_diagnostic()?;
        let raw_msg = str::from_utf8(&buf[..])
            .into_diagnostic()?
            .trim_matches(char::from(0));

        tracing::debug!("{:?}", raw_msg);

        let v: Value = serde_json::from_str(raw_msg).into_diagnostic()?;

        if let Some(error) = v.get("error") {
            bail!(error.clone());
        }

        if payload.needs_encryption() {
            return self.decrypt_message(v);
        }

        Ok(v)
    }

    fn decrypt_message(&self, v: Value) -> Result<Value> {
        let decoded_nonce = BASE64_STANDARD
            .decode(v["nonce"].as_str().unwrap())
            .into_diagnostic()?;
        let decoded_ciphertext = BASE64_STANDARD
            .decode(v["message"].as_str().unwrap())
            .into_diagnostic()?;

        let raw_result = self
            .crypto_box
            .clone()
            .unwrap()
            .decrypt(
                Nonce::from_slice(&decoded_nonce),
                Payload::from(decoded_ciphertext.as_slice()),
            )
            .map_err(|e| miette!(e))?;

        let v: Value = serde_json::from_str(str::from_utf8(&raw_result).into_diagnostic()?)
            .into_diagnostic()?;

        Ok(v)
    }
}
