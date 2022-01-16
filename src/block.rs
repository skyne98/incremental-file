use anyhow::Result;
use blake3::Hash;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub length: u64,
    pub hash: String,
}

impl Block {
    pub fn new(length: u64, hash: String) -> Block {
        Block { length, hash }
    }
    pub fn from_data<D: AsRef<[u8]>>(data: D) -> Self {
        let data = data.as_ref();
        let hash = format!("{}", blake3::hash(data));
        Block {
            length: data.len() as u64,
            hash,
        }
    }
    pub fn hash(&self) -> Result<Hash> {
        let hash = self.hash.parse()?;
        Ok(hash)
    }
    pub fn update<D: AsRef<[u8]>>(&mut self, data: D) {
        let data = data.as_ref();
        self.length = data.len() as u64;
        self.hash = format!("{}", blake3::hash(data));
    }
    pub fn validate<D: AsRef<[u8]>>(&self, data: D) -> Result<()> {
        let data = data.as_ref();
        let hash = self.hash()?;
        let hash_data = blake3::hash(data);
        if hash != hash_data {
            Err(anyhow::anyhow!("Invalid hash"))
        } else {
            Ok(())
        }
    }
}
