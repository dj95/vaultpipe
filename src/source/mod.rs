use miette::Result;

use std::sync::Arc;

pub trait Source {
    fn get_value(&self, identifier: &str) -> Result<String>;
}

pub fn initialize_source_by_name(name: &str) -> Option<Arc<dyn Source>> {
    // TODO: register secret sources by name

    None
}

pub fn get_secret_source_from_uri(uri: &str) -> Option<String> {
    if !uri.starts_with("vp://") {
        return None;
    }

    uri[5..].split_once("/").map(|res| res.0.to_owned())
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("somerandomstring", None)]
    #[case("vp://keepass/db/value", Some("keepass".to_owned()))]
    #[case("vp://test", None)]
    #[case("vp://test/foo", Some("test".to_owned()))]
    #[test_log::test]
    fn test_get_secret_source_from_uri(#[case] uri: &str, #[case] expected: Option<String>) {
        let res = get_secret_source_from_uri(uri);

        assert_eq!(res, expected);
    }
}
