use std::env;
use std::fs;

use miette::{IntoDiagnostic, Result, bail};

#[cfg(target_os = "macos")]
pub fn path() -> Result<String> {
    let tmp_dir = env::var("TMPDIR").into_diagnostic()?;
    let socket_path = tmp_dir + "org.keepassxc.KeePassXC.BrowserServer";

    if !fs::exists(&socket_path).into_diagnostic()? {
        bail!("socket does not exist");
    }

    Ok(socket_path)
}

#[cfg(target_os = "linux")]
pub fn path() -> Result<String> {
    Ok("".to_owned())
}
