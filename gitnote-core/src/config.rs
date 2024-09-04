use std::path::Path;

use encoding_rs::Encoding;
use serde::{Deserialize, Serialize};

use crate::config::PersistenceType::Ephemeral;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    persistence_type: PersistenceType,
    charset: Charset,
}

impl Config {
    pub fn resolve<P>(p: P) -> anyhow::Result<Config>
    where
        P: AsRef<Path>,
    {
        let s = std::fs::read_to_string(p)?;
        Self::resolve_from_str(s.as_str())
    }

    fn resolve_from_str(s: &str) -> anyhow::Result<Config> {
        Ok(serde_yaml_ng::from_str::<Self>(s)?)
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

#[derive(Debug)]
struct Charset {
    encoding: &'static Encoding,
}

impl<'de> serde::Deserialize<'de> for Charset {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let encoding = Encoding::for_label(s.as_bytes())
            .expect(format!("`{}` is Unknown charset", s).as_str());
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