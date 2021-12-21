use anyhow::Result;
use incremental_file::{block::Block, converter::Converter, file::File};

pub struct BincodeConverter {}

impl Converter for BincodeConverter {
    fn serialize_block(&self, block: &Block) -> Result<Vec<u8>> {
        Ok(bincode::serialize(block)?)
    }
    fn deserialize_block(&self, data: &[u8]) -> Result<Block> {
        Ok(bincode::deserialize(data)?)
    }
    fn serialize_file(&self, file: &File) -> Result<Vec<u8>> {
        Ok(bincode::serialize(file)?)
    }
    fn deserialize_file(&self, data: &[u8]) -> Result<File> {
        Ok(bincode::deserialize(data)?)
    }
}
