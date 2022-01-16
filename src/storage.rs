use crate::{block::Block, file::File};
use anyhow::Result;
use async_trait::async_trait;

pub type BoxedStorage = Box<dyn Storage>;
/// A storage is a collection of files addressed by their hashes, and block data. Files may be in any state of completeness or validity.
#[async_trait]
pub trait Storage {
    async fn get_file(&self, hash: &str) -> Result<File>;
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
