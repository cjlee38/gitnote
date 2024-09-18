// use std::fmt::Display;
// use anyhow::anyhow;
// use encoding_rs::Encoding;
// use crate::cli::argument::ConfigGetArgs;
// use crate::config::{Charset, Config, PersistenceType};
//
// pub trait ConfigOptions {
//     fn set<K, V>(&self, key: K, value: V) -> anyhow::Result<Self>
//     where
//         K: AsRef<str> + Display,
//         V: AsRef<str> + Display;
//     fn get<K>(&self, key: K) -> anyhow::Result<&Self>
//     where
//         K: AsRef<str>;
// }
//
// impl ConfigOptions for Config {
//     fn set<K, V>(&self, key: K, value: V) -> anyhow::Result<Self>
//     where
//         K: AsRef<str> + Display,
//         V: AsRef<str> + Display,
//     {
//         todo!()
//     }
//
//     fn get<K>(&self, key: K) -> anyhow::Result<&Self>
//     where
//         K: AsRef<str>
//     {
//         todo!()
//     }
// }
//
//
// impl ConfigOptions for PersistenceType {
//     fn set<K, V>(&self, _key: K, value: V) -> anyhow::Result<Self>
//     where
//         K: AsRef<str> + Display,
//         V: AsRef<str> + Display,
//     {
//         let persistence_type = match value.to_string().to_lowercase().as_str() {
//             "ephemeral" => PersistenceType::Ephemeral,
//             "closet" => PersistenceType::Closet,
//             "latest" => PersistenceType::Latest,
//             _ => return Err(anyhow!("Invalid persistence type : `{}`", value)),
//         };
//         Ok(persistence_type)
//     }
//
//     fn get<K>(&self, _key: K) -> anyhow::Result<&Self>
//     where
//         K: AsRef<str>
//     {
//         Ok(&self)
//     }
// }
//
// impl ConfigOptions for Charset {
//     fn set<K, V>(&self, _key: K, value: V) -> anyhow::Result<Self>
//     where
//         K: AsRef<str>,
//         V: AsRef<str> + Display,
//     {
//         Encoding::for_label(value.to_string().as_bytes())
//             .map(|encoding| Charset { encoding })
//             .ok_or(anyhow!("Invalid charset : `{}`", value))
//     }
//
//     fn get<K>(&self, key: K) -> anyhow::Result<&Self>
//     where
//         K: AsRef<str>
//     {
//         Ok(&self)
//     }
// }