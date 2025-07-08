use kdl::{KdlDocument, KdlNode};
use miette::{bail, miette, IntoDiagnostic, Result};
use xdg::BaseDirectories;

use std::fs;

#[tracing::instrument]
pub fn config_file_path(file_name_candidate: Option<String>) -> Result<String> {
    match file_name_candidate {
        Some(file_name) => Ok(file_name),
        None => {
            let config_file =
                match BaseDirectories::with_prefix("vaultpipe").get_config_file("config.kdl") {
                    Some(config_file) => config_file,
                    None => bail!("cannot get config file path"),
                };

            Ok(config_file.into_os_string().into_string().unwrap())
        }
    }
}

#[derive(Debug)]
pub struct Config {}

pub fn read_config(file_name: Option<String>) -> Result<Config> {
    let file_name = config_file_path(file_name)?;
    tracing::debug!(target: "detected config file", config_file = file_name);

    let contents = fs::read_to_string(&file_name).map_err(|err| {
        miette!(
            code = "config::read_config",
            help = "Validate the config path and check that the file exists or run with --setup.",
            "Config file: {file_name}\n{err}"
        )
    })?;

    let nodes = contents.parse::<KdlDocument>().into_diagnostic()?;

    Ok(Config {})
}
