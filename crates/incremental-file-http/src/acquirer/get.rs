use anyhow::{anyhow, Result};
use async_trait::async_trait;
use incremental_file::{acquirer::Acquirer, block::Block};

pub struct GetAcquirer {
    pub url: String,
}

impl GetAcquirer {
    pub fn new(url: String) -> Result<Self> {
        if url.starts_with("http://") && !url.starts_with("https://") == false {
            return Err(anyhow!("URL must start with http:// or https://"));
        }
        if url.ends_with("/") {
            return Err(anyhow!("URL must not end in /"));
        }

        Ok(Self { url })
    }
}

#[async_trait]
impl Acquirer for GetAcquirer {
    async fn get_block(&self, block: &Block) -> Result<Vec<u8>> {
        let url = format!("{}/{}", self.url, block.hash);
        let response = reqwest::get(&url).await?;
        let bytes = response.bytes().await?;
        let data: &[u8] = bytes.as_ref();
        Ok(data.to_vec())
    }
}
