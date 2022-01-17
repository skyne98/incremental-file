# incremental-file
This tiny library allows a program to incrementally receive a file split into verifiable blocks (or chunks). Both the storage of the current state of the file and the transport used to request and receive the blocks are generic. By default, the library only provides a `MemoryStorage` storage implementation.

Inside of the `crates/` directory, you can find the following additional libraries and implementations:
- `crates/incremental-file-local`: A storage implementation using the local file system
- `crates/incremental-file-http`: An implementation of the `Acquirer` that receives chunks from an HTTP server
- `crates/incremental-file-converter-*`: Serialization of files and blocks using `bincode`, `json` or `toml`

### Example
#### Creating a new file
```rust
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
```
#### Creating a cryptographically signed file
```rust
let data = (0..100).collect::<Vec<u8>>();
let keypair = generate_keypair()?;
let public_key = get_public_key(&keypair)?;
let public_key = parse_public_key(&public_key);
let signature = keypair.sign(&data);
public_key.verify(&data, signature.as_ref()).unwrap();
```