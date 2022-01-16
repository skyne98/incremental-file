use crate::block::Block;
use anyhow::Result;
use async_trait::async_trait;

pub type BoxedAcquirer = Box<dyn Acquirer>;
/// Reads or downloads file blocks from the external source.
#[async_trait]
pub trait Acquirer {
    async fn get_block(&self, block: &Block) -> Result<Vec<u8>>;
}
