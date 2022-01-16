use std::marker::Send;
use std::path::PathBuf;

use anyhow::{Context, Result};
use async_trait::async_trait;
use incremental_file::{block::Block, converter::Converter, file::File, storage::Storage};

pub struct FileSystemStorage<C: Converter> {
    root_dir: PathBuf,
    file_dir: PathBuf,
    block_dir: PathBuf,
    converter: C,
}

impl<C: Converter> FileSystemStorage<C> {
    pub fn new(root_dir: PathBuf, converter: C) -> Self {
        Self {
            root_dir: root_dir.clone(),
            file_dir: root_dir.join("files"),
            block_dir: root_dir.join("blocks"),
            converter,
        }
    }
    pub async fn ensure_dirs(&self) -> Result<()> {
        tokio::fs::create_dir_all(&self.file_dir).await?;
        tokio::fs::create_dir_all(&self.block_dir).await?;
        Ok(())
    }
}

#[async_trait]
impl<C: Converter> Storage for FileSystemStorage<C> {
    // Files
    async fn get_file(&self, hash: &str) -> Result<Option<File>> {
        self.ensure_dirs().await?;
        let path = self.file_dir.join(hash);
        if path.exists() {
            let bytes = tokio::fs::read(path)
                .await
                .context(format!("There was an issue reading file {}", hash))?;
            let file = self.converter.deserialize_file(bytes.as_slice())?;
            Ok(Some(file))
        } else {
            Ok(None)
        }
    }
    async fn file_exists(&self, hash: &str) -> Result<bool> {
        self.ensure_dirs().await?;
        let path = self.file_dir.join(hash);
        let exists = tokio::fs::metadata(path).await.is_ok();
        Ok(exists)
    }
    async fn upsert_file(&mut self, file: &File) -> Result<()> {
        self.ensure_dirs().await?;
        let path = self.file_dir.join(&file.hash);
        let bytes = self.converter.serialize_file(&file)?;
        tokio::fs::write(path, bytes).await?;
        Ok(())
    }
    async fn remove_file(&mut self, hash: &str) -> Result<()> {
        self.ensure_dirs().await?;
        let path = self.file_dir.join(hash);
        tokio::fs::remove_file(path).await?;
        Ok(())
    }

    // Blocks
    async fn get_block_data(&self, block: &Block) -> Result<Option<Vec<u8>>> {
        self.ensure_dirs().await?;
        let path = self.block_dir.join(&block.hash);
        let exists = tokio::fs::metadata(&path).await.is_ok();
        if exists {
            let bytes = tokio::fs::read(&path).await?;
            // Validate
            if bytes.len() as u64 != block.length {
                return Err(anyhow::anyhow!(
                    "Block with hash {} has invalid length",
                    block.hash
                ));
            }
            let hash = blake3::hash(bytes.as_slice());
            if format!("{}", hash) != block.hash {
                return Err(anyhow::anyhow!(
                    "Data read for block with hash {} doesn't match its hash",
                    block.hash
                ));
            }
            Ok(Some(bytes))
        } else {
            Ok(None)
        }
    }
    async fn block_exists(&self, block: &Block) -> Result<bool> {
        self.ensure_dirs().await?;
        let path = self.block_dir.join(&block.hash);
        let exists = tokio::fs::metadata(path).await.is_ok();
        Ok(exists)
    }
    async fn upsert_block_data<D: AsRef<[u8]> + Send>(
        &mut self,
        block: &Block,
        data: D,
    ) -> Result<()> {
        self.ensure_dirs().await?;
        let hash = &block.hash;
        let path = self.block_dir.join(hash);
        let bytes = data.as_ref();
        tokio::fs::write(path, bytes).await?;
        Ok(())
    }
    async fn remove_block_data(&mut self, block: &Block) -> Result<()> {
        self.ensure_dirs().await?;
        let path = self.block_dir.join(&block.hash);
        tokio::fs::remove_file(path).await?;
        Ok(())
    }
}
