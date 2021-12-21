use crate::{converter::BoxedConverter, file::File};
use anyhow::Result;
use async_trait::async_trait;

pub type BoxedStorage = Box<dyn Storage>;
/// A storage is a collection of files addressed by their hashes. Files may be in any state of completeness or validity.
#[async_trait]
pub trait Storage {
    async fn get_file(&self, hash: &str) -> Result<File>;
    async fn upsert_file(&mut self, file: &File) -> Result<()>;
    async fn remove_file(&mut self, hash: &str) -> Result<()>;
}
