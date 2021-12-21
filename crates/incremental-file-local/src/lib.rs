use std::path::Path;

use anyhow::Result;
use incremental_file::{converter::BoxedConverter, file::File};

pub mod storage;

pub async fn write_file<P: AsRef<Path>>(
    file: &File,
    path: P,
    converter: &BoxedConverter,
) -> Result<()> {
    let path = path.as_ref();
    let bytes = converter.serialize_file(&file)?;
    tokio::fs::write(path, bytes).await?;
    Ok(())
}
