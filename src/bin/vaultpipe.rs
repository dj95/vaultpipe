use clap::Parser;
use miette::Result;
use tracing_subscriber::EnvFilter;

use std::collections::BTreeMap;
use std::sync::Arc;

use vaultpipe::config;
use vaultpipe::env;
use vaultpipe::pty;
use vaultpipe::source::{get_secret_source_from_uri, Source, initialize_source_by_name};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[clap(trailing_var_arg = true)]
struct Args {
    #[arg(short, long, help = "Path to the configuration file")]
    config: Option<String>,

    #[arg(
        long,
        default_value_t = false,
        help = "Remove all environment variables"
    )]
    clear_env: bool,

    #[arg(short, long, default_value = ".env", help = "The env file to process")]
    env_file: String,

    command: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing subscriber with environment configuration
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args = Args::parse();
    tracing::debug!("{:?}", args.command);

    let config = config::read_config(args.config)?;
    let mut env = env::parse_file(&args.env_file)?;
    let mut source_map: BTreeMap<String, Arc<dyn Source>> = BTreeMap::new();

    for (key, value) in &env.clone() {
        if !value.starts_with("vp://") {
            continue;
        }

        let source_name = match get_secret_source_from_uri(value) {
            Some(name) => name,
            None => continue,
        };

        if !source_map.contains_key(key) {
            let src = match initialize_source_by_name(&source_name) {
                Some(s) => s,
                None => continue
            };

            source_map.insert(source_name.clone(), src);
        }

        let src = source_map.get(&source_name).unwrap();
        let new_value = src.get_value(value)?;
        env.insert(key.clone(), new_value);
    }

    pty::run(&args.command, env, args.clear_env)?;

    Ok(())
}
