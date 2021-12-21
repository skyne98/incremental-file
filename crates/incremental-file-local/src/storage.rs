use std::path::PathBuf;

use anyhow::{Context, Result};
use async_trait::async_trait;
use incremental_file::{converter::BoxedConverter, file::File, storage::Storage};

pub struct FileSystemStorage {
    root_dir: PathBuf,
    converter: BoxedConverter,
}

impl FileSystemStorage {
    pub fn new(root_dir: PathBuf, converter: BoxedConverter) -> Self {
        Self {
            root_dir,
            converter,
        }
    }
}

#[async_trait]
impl Storage for FileSystemStorage {
    async fn get_file(&self, hash: &str) -> Result<File> {
        let path = self.root_dir.join(hash);
        let bytes = tokio::fs::read(path)
            .await
            .context(format!("File with {} doesn't exist", hash))?;
        let file = self.converter.deserialize_file(bytes.as_slice())?;
        Ok(file)
    }
    async fn upsert_file(&mut self, file: &File) -> Result<()> {
        let path = self.root_dir.join(&file.hash);
        let bytes = self.converter.serialize_file(&file)?;
        tokio::fs::write(path, bytes).await?;
        Ok(())
    }
    async fn remove_file(&mut self, hash: &str) -> Result<()> {
        let path = self.root_dir.join(hash);
        tokio::fs::remove_file(path).await?;
        Ok(())
    }
}
