use crate::block::Block;
use anyhow::{Context, Result};
use blake3::Hash;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    pub blocks: Vec<Block>,
    pub hash: String,
    // TODO: Add signature
}

impl File {
    pub fn new(blocks: Vec<Block>, hash: String) -> File {
        File { blocks, hash }
    }
    pub fn from_data(data: Vec<u8>, block_size: u64) -> Self {
        let mut blocks = Vec::new();
        let mut data_iter = data.iter();
        let mut block_data = Vec::new();
        let mut block_length = 0;
        while let Some(byte) = data_iter.next() {
            block_data.push(*byte);
            block_length += 1;
            if block_length == block_size {
                let block = Block::from_data(block_data);
                blocks.push(block);
                block_data = Vec::new();
                block_length = 0;
            }
        }
        if block_data.len() > 0 {
            let block = Block::from_data(block_data);
            blocks.push(block);
        }
        let hash = format!("{}", blake3::hash(data.as_slice()));
        File { blocks, hash }
    }
    pub fn hash(&self) -> Result<Hash> {
        let hash = self.hash.parse()?;
        Ok(hash)
    }
    pub fn update_block(&mut self, block: Block) -> Result<()> {
        let hash = block.hash()?;
        let hash_str = format!("{}", hash);
        let index = self
            .blocks
            .iter()
            .position(|b| b.hash == hash_str)
            .context("File doesn't contain this block")?;
        self.blocks[index] = block;

        Ok(())
    }
    pub fn recalculate_hash(&mut self) -> Result<()> {
        let mut data = Vec::new();
        for block in &self.blocks {
            data.extend(block.data.as_ref().expect("Block data is empty"));
        }
        let hash = format!("{}", blake3::hash(data.as_slice()));
        self.hash = hash;
        Ok(())
    }
    pub fn data(&self) -> Result<Vec<u8>> {
        let mut data: Vec<u8> = Vec::new();
        for block in &self.blocks {
            data.extend(block.data.as_ref().expect("Block data is empty"));
        }
        Ok(data)
    }
    pub fn validate(&self) -> Result<()> {
        let hash = self.hash()?;
        let hash_data = blake3::hash(self.data()?.as_slice());
        if hash != hash_data {
            Err(anyhow::anyhow!("Invalid hash"))
        } else {
            Ok(())
        }
    }
    pub fn unfinished_blocks(&self) -> Vec<&Block> {
        let mut unfinished_blocks = Vec::new();
        for block in &self.blocks {
            if block.data.is_none() {
                unfinished_blocks.push(block.clone());
            }
        }
        unfinished_blocks
    }
}
