use miette::Result;
use tracing_subscriber::EnvFilter;

use keepassxc_browser_protocol::client::Client;

pub fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let mut client = Client::new()?;
    client.handshake()?;

    if client.test_association().is_err() {
        client.associate()?;
    };

    // TODO: implement unlock request if db is locked

    client.get_logins("vp://test".to_owned())?;

    Ok(())
}
