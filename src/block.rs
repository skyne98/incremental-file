use anyhow::Result;
use blake3::Hash;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub length: u64,
    pub data: Option<Vec<u8>>,
    pub hash: String,
}

impl Block {
    pub fn new(length: u64, hash: String) -> Block {
        Block {
            length,
            data: None,
            hash,
        }
    }
    pub fn from_data(data: Vec<u8>) -> Self {
        let hash = format!("{}", blake3::hash(data.as_slice()));
        Block {
            length: data.len() as u64,
            data: Some(data),
            hash,
        }
    }
    pub fn hash(&self) -> Result<Hash> {
        let hash = self.hash.parse()?;
        Ok(hash)
    }
    pub fn update(&mut self, data: Vec<u8>) {
        self.length = data.len() as u64;
        self.hash = format!("{}", blake3::hash(data.as_slice()));
        self.data = Some(data);
    }
    pub fn validate(&self) -> Result<()> {
        if let Some(data) = &self.data {
            let hash = self.hash()?;
            let hash_data = blake3::hash(data.as_slice());
            if hash != hash_data {
                Err(anyhow::anyhow!("Invalid hash"))
            } else {
                Ok(())
            }
        } else {
            return Err(anyhow::anyhow!(
                "Block data is empty, probably it wasn't received yet"
            ));
        }
    }
}
