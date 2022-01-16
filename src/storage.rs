use std::collections::HashMap;

use crate::{block::Block, file::File};
use anyhow::Result;
use async_trait::async_trait;

pub type BoxedStorage = Box<dyn Storage>;
/// A storage is a collection of files addressed by their hashes, and block data. Files may be in any state of completeness or validity.
#[async_trait]
pub trait Storage {
    async fn get_file(&self, hash: &str) -> Result<Option<File>>;
    async fn file_exists(&self, hash: &str) -> Result<bool>;
    async fn upsert_file(&mut self, file: &File) -> Result<()>;
    async fn remove_file(&mut self, hash: &str) -> Result<()>;
    async fn get_block_data(&self, block: &Block) -> Result<Option<Vec<u8>>>;
    async fn block_exists(&self, block: &Block) -> Result<bool>;
    async fn upsert_block_data<D: AsRef<[u8]> + Send>(
        &mut self,
        block: &Block,
        data: D,
    ) -> Result<()>;
    async fn remove_block_data(&mut self, block: &Block) -> Result<()>;
}

pub struct MemoryStorage {
    files: HashMap<String, File>,
    blocks: HashMap<String, Vec<u8>>,
}
impl MemoryStorage {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
            blocks: HashMap::new(),
        }
    }
}
#[async_trait]
impl Storage for MemoryStorage {
    async fn get_file(&self, hash: &str) -> Result<Option<File>> {
        let file = self.files.get(hash).map(|f| f.clone());
        Ok(file)
    }

    async fn file_exists(&self, hash: &str) -> Result<bool> {
        Ok(self.files.contains_key(hash))
    }

    async fn upsert_file(&mut self, file: &File) -> Result<()> {
        self.files.insert(file.hash.clone(), file.clone());
        Ok(())
    }

    async fn remove_file(&mut self, hash: &str) -> Result<()> {
        self.files.remove(hash);
        Ok(())
    }

    async fn get_block_data(&self, block: &Block) -> Result<Option<Vec<u8>>> {
        let block = self.blocks.get(&block.hash).map(|b| b.clone());
        Ok(block)
    }

    async fn block_exists(&self, block: &Block) -> Result<bool> {
        Ok(self.blocks.contains_key(&block.hash))
    }

    async fn upsert_block_data<D: AsRef<[u8]> + Send>(
        &mut self,
        block: &Block,
        data: D,
    ) -> Result<()> {
        self.blocks
            .insert(block.hash.clone(), data.as_ref().to_vec());
        Ok(())
    }

    async fn remove_block_data(&mut self, block: &Block) -> Result<()> {
        self.blocks.remove(&block.hash);
        Ok(())
    }
}
