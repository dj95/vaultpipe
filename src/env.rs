use dotenvy::from_path_iter;
use miette::{IntoDiagnostic, Result};

use std::collections::BTreeMap;

pub fn parse_file(filename: &str) -> Result<BTreeMap<String, String>> {
    let mut env = BTreeMap::new();

    for item in from_path_iter(filename).into_diagnostic()? {
        let (key, value) = item.unwrap();
        env.insert(key, value);
    }

    Ok(env)
}
