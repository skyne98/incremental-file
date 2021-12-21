use anyhow::Result;
use incremental_file::{block::Block, converter::Converter, file::File};

pub struct TomlConverter {}

impl Converter for TomlConverter {
    fn serialize_block(&self, block: &Block) -> Result<Vec<u8>> {
        Ok(toml::to_vec(block)?)
    }
    fn deserialize_block(&self, data: &[u8]) -> Result<Block> {
        Ok(toml::from_slice(data)?)
    }
    fn serialize_file(&self, file: &File) -> Result<Vec<u8>> {
        Ok(toml::to_vec(file)?)
    }
    fn deserialize_file(&self, data: &[u8]) -> Result<File> {
        Ok(toml::from_slice(data)?)
    }
}
