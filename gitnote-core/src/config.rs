use std::fs;
use std::io::Cursor;
use std::path::Path;

use anyhow::Context;
use serde::{Deserialize, Serialize};

use encoding_rs::Encoding;

use crate::config::PersistenceType::Ephemeral;
use crate::utils::Writeable;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    persistence_type: PersistenceType,
    #[serde(default)]
    charset: Charset,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            persistence_type: PersistenceType::default(),
            charset: Charset::default(),
        }
    }
}

impl Config {
    pub fn resolve<P>(p: P) -> anyhow::Result<Config>
    where
        P: AsRef<Path>,
    {
        let s = fs::read_to_string(p)?;
        if s.is_empty() {
            return Ok(Config::default());
        }
        Self::resolve_from_str(s.as_str())
    }

    fn resolve_from_str(s: &str) -> anyhow::Result<Config> {
        serde_yaml_ng::from_str::<Self>(s)
            .context("Failed to parse config")
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum PersistenceType {
    Ephemeral,
    Closet,
    Latest,
}

impl Default for PersistenceType {
    fn default() -> Self {
        Ephemeral
    }
}

#[derive(Debug, PartialEq)]
struct Charset {
    encoding: &'static Encoding,
}

impl Default for Charset {
    fn default() -> Self {
        Charset {
            encoding: Encoding::for_label(b"utf-8").unwrap(),
        }
    }
}

impl<'de> serde::Deserialize<'de> for Charset {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.is_empty() {
            return Ok(Charset::default());
        }
        let encoding = Encoding::for_label(s.as_bytes())
            .expect(format!("`{}` is Unknown charset", s).as_str()); // TODO : may fail. need to handle error
        Ok(Charset { encoding })
    }
}

impl serde::Serialize for Charset {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.encoding.name().serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve() {
        let text = r#"
persistence_type: ephemeral
charset: utf-8
        "#;
        let config = Config::resolve_from_str(text).unwrap();
        assert_eq!(config.persistence_type, Ephemeral);
        assert_eq!(config.charset.encoding.name(), "UTF-8");
    }

    #[test]
    fn empty() {
        let text = r#""#;
        let config = Config::resolve_from_str(text).unwrap();
        assert_eq!(config.persistence_type, Ephemeral);
        assert_eq!(config.charset.encoding.name(), "UTF-8");
    }

    #[test]
    #[should_panic]
    fn charset_unknown() {
        let text = r#"
persistence_type: ephemeral
charset: unknown
        "#;
        let config = Config::resolve_from_str(text).unwrap();
    }

    #[test]
    #[should_panic]
    fn persistence_type_unknown() {
        let text = r#"
persistence_type: unknown
charset: utf-8
        "#;
        let config = Config::resolve_from_str(text).unwrap();
    }

    mod charsets {
        #[test]
        fn charset() {
            let bytes = vec![0x68, 0x65, 0x6c, 0x6c, 0x6f, 0xB4, 0xC2, 0x20, 0xBE, 0xC8, 0xB3, 0xE7];
            let encoding = encoding_rs::Encoding::for_label(b"euc-kr").unwrap();
            let (decoded, _encoding, error) = encoding.decode(&bytes);
            assert_eq!(decoded, "hello는 안녕");
            assert!(!error)
        }

        #[test]
        fn unknown_charset() {
            let encoding = encoding_rs::Encoding::for_label(b"unknown");
            assert_eq!(encoding, None);
        }
    }
}
