use anyhow::Result;

use crate::{block::Block, file::File};

pub type BoxedConverter = Box<dyn Converter>;
pub trait Converter: Send + Sync {
    fn serialize_block(&self, block: &Block) -> Result<Vec<u8>>;
    fn deserialize_block(&self, data: &[u8]) -> Result<Block>;
    fn serialize_file(&self, file: &File) -> Result<Vec<u8>>;
    fn deserialize_file(&self, data: &[u8]) -> Result<File>;
}
