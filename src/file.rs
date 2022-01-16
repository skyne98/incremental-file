use crate::{
    block::Block,
    crypto::{parse_public_key, KeyPair, PublicKey},
    storage::Storage,
};
use anyhow::{Context, Result};
use blake3::Hash;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct File {
    pub blocks: Vec<Block>,
    pub hash: String,
    pub signature: Option<String>,
}

impl File {
    pub fn new(blocks: Vec<Block>, hash: String) -> File {
        File {
            blocks,
            hash,
            signature: None,
        }
    }
    pub async fn from_data<D: AsRef<[u8]>, S: Storage>(
        data: D,
        block_size: u64,
        storage: &mut S,
    ) -> Result<Self> {
        let data = data.as_ref();
        let mut blocks = Vec::new();
        let mut data_iter = data.iter();
        let mut block_data = Vec::new();
        let mut block_length = 0;
        while let Some(byte) = data_iter.next() {
            block_data.push(*byte);
            block_length += 1;
            if block_length == block_size {
                let block = Block::from_data(&block_data);
                blocks.push(block.clone());
                storage.upsert_block_data(&block, block_data).await?;
                block_data = Vec::new();
                block_length = 0;
            }
        }
        if block_data.len() > 0 {
            let block = Block::from_data(&block_data);
            blocks.push(block.clone());
            storage.upsert_block_data(&block, &block_data).await?;
        }
        let hash = format!("{}", blake3::hash(data));
        Ok(File {
            blocks,
            hash,
            signature: None,
        })
    }
    pub fn sign(&mut self, keypair: &KeyPair) -> Result<()> {
        let hash = self.hash()?;
        let signature = keypair.sign(hash.as_bytes());
        self.signature = Some(hex::encode(signature));
        Ok(())
    }
    pub fn hash(&self) -> Result<Hash> {
        let hash = self.hash.parse()?;
        Ok(hash)
    }
    pub async fn data<S: Storage>(&self, storage: &S) -> Result<Vec<u8>> {
        let mut data: Vec<u8> = Vec::new();
        for block in &self.blocks {
            data.extend(
                storage
                    .get_block_data(&block)
                    .await?
                    .expect("Block data is empty"),
            );
        }
        Ok(data)
    }
    pub async fn validate<S: Storage>(&self, storage: &S) -> Result<()> {
        let hash = self.hash()?;
        let data = self.data(storage).await?;
        let hash_data = blake3::hash(data.as_slice());
        // Validate hash
        if hash != hash_data {
            Err(anyhow::anyhow!("Invalid hash"))
        } else {
            Ok(())
        }
    }
    pub async fn validate_and_verify<S: Storage>(
        &self,
        storage: &S,
        public_key: &PublicKey,
    ) -> Result<()> {
        let hash = self.hash()?;
        let data = self.data(storage).await?;
        let hash_data = blake3::hash(data.as_slice());
        // Validate hash
        if hash != hash_data {
            Err(anyhow::anyhow!("Invalid hash"))
        } else {
            // Validate signature
            if let Some(signature) = &self.signature {
                let signature = hex::decode(signature)?;
                if let Err(verify_err) = public_key.verify(hash.as_bytes(), &signature) {
                    Err(anyhow::anyhow!(format!(
                        "Signature {:?} is invalid: {}",
                        signature, verify_err
                    )))
                } else {
                    Ok(())
                }
            } else {
                Err(anyhow::anyhow!("No signature"))
            }
        }
    }
    pub async fn unfinished_blocks<S: Storage>(&self, storage: &S) -> Result<Vec<Block>> {
        let mut unfinished_blocks = Vec::new();
        for block in &self.blocks {
            if storage.block_exists(&block).await? {
                unfinished_blocks.push(block.clone());
            }
        }
        Ok(unfinished_blocks)
    }
    pub async fn progress<S: Storage>(&self, storage: &S) -> Result<f64> {
        let unfinished_blocks = self.unfinished_blocks(storage).await?;
        let progress = unfinished_blocks.len() as f64 / self.blocks.len() as f64;
        Ok(progress)
    }
}
