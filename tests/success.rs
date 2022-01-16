use anyhow::{Context, Result};
use incremental_file::{
    file::File,
    storage::{MemoryStorage, Storage},
};

#[test]
fn success() {
    assert!(true);
}

#[tokio::test]
async fn can_create_file() -> Result<()> {
    let mut storage = MemoryStorage::new();
    let data = (0..100).collect::<Vec<u8>>();
    let file = File::from_data(data, 10, &mut storage).await?;

    assert_eq!(file.blocks.len(), 10);
    for block in &file.blocks {
        assert_eq!(
            storage
                .get_block_data(block)
                .await?
                .context("Block doesn't exist")?
                .len(),
            10
        );
    }
    Ok(())
}
#[tokio::test]
async fn can_create_file_with_one_leftover_block() -> Result<()> {
    let mut storage = MemoryStorage::new();
    let data = (0..101).collect::<Vec<u8>>();
    let file = File::from_data(data, 10, &mut storage).await?;

    assert_eq!(file.blocks.len(), 11);
    for block_index in 0..10 {
        let block = &file.blocks[block_index];
        assert_eq!(
            storage
                .get_block_data(block)
                .await?
                .context("Block doesn't exist")?
                .len(),
            10
        );
    }
    let last_block = storage
        .get_block_data(&file.blocks[10])
        .await?
        .context("Block doesn't exist")?;
    assert_eq!(last_block.len(), 1);
    Ok(())
}
#[tokio::test]
async fn can_create_and_reassemble_file() -> Result<()> {
    let mut storage = MemoryStorage::new();
    let data = (0..100).collect::<Vec<u8>>();
    let file = File::from_data(&data, 10, &mut storage).await?;

    let reassembled = file.data(&storage).await?;
    assert_eq!(reassembled.len(), data.len());
    for (index, byte) in data.iter().enumerate() {
        assert_eq!(reassembled[index], *byte);
    }
    Ok(())
}
#[tokio::test]
async fn file_hash_is_correct() -> Result<()> {
    let mut storage = MemoryStorage::new();
    let data = (0..100).collect::<Vec<u8>>();
    let file = File::from_data(&data, 10, &mut storage).await?;
    let hash = file.hash()?;
    let actual_hash = blake3::hash(&data);
    assert!(format!("{}", hash).len() > 0);
    assert!(format!("{}", actual_hash).len() > 0);
    assert_eq!(format!("{}", hash), format!("{}", actual_hash));
    Ok(())
}
#[tokio::test]
async fn file_validation_succeeds() -> Result<()> {
    let mut storage = MemoryStorage::new();
    let data = (0..100).collect::<Vec<u8>>();
    let file = File::from_data(&data, 10, &mut storage).await?;
    assert!(file.validate(&storage).await.is_ok());
    Ok(())
}
