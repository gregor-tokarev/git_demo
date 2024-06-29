use std::{fmt::Display, io::BufWriter, str::FromStr};

use anyhow::{anyhow, Result};
use sha1::{Digest, Sha1};

#[derive(Debug)]
pub enum TreeRowMode {
    File,
    Tree,
}

impl ToString for TreeRowMode {
    fn to_string(&self) -> String {
        match self {
            Self::File => String::from("100644"),
            Self::Tree => String::from("40000"),
        }
    }
}

impl FromStr for TreeRowMode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "100644" => Ok(Self::File),
            "40000" => Ok(Self::Tree),
            _ => Err(anyhow!("No code")),
        }
    }
}

#[derive(Debug)]
pub struct TreeRow {
    pub object_name: String,
    hash: String,
    mode: TreeRowMode,
}

impl Display for TreeRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.mode.to_string(),
            self.object_name,
            self.hash
        )
    }
}

impl TreeRow {
    pub fn from_bytes(mode: &[u8], object_name: &[u8], hash: &[u8]) -> Result<Self> {
        let inner_object_name = String::from_utf8(object_name.to_vec())?;

        let hash = hex::encode(Sha1::new().chain_update(hash).finalize());

        Ok(Self {
            hash,
            object_name: inner_object_name,
            mode: TreeRowMode::from_str(&String::from_utf8(mode.to_vec())?)?,
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        let mut writer = BufWriter::new(bytes);

        // bytes.clone()
        vec![]
    }
}
