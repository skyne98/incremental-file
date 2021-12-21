use anyhow::Result;
use incremental_file::{block::Block, converter::Converter, file::File};

pub struct JsonConverter {}

impl Converter for JsonConverter {
    fn serialize_block(&self, block: &Block) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec(block)?)
    }
    fn deserialize_block(&self, data: &[u8]) -> Result<Block> {
        Ok(serde_json::from_slice(data)?)
    }
    fn serialize_file(&self, file: &File) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec(file)?)
    }
    fn deserialize_file(&self, data: &[u8]) -> Result<File> {
        Ok(serde_json::from_slice(data)?)
    }
}
