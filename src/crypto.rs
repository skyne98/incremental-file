use anyhow::{anyhow, Context, Result};
use ring::{
    rand,
    signature::{Ed25519KeyPair, KeyPair as RingKeyPair, UnparsedPublicKey, ED25519},
};

pub type PublicKey = UnparsedPublicKey<Vec<u8>>;
pub type KeyPair = Ed25519KeyPair;

pub fn generate_keypair() -> Result<KeyPair> {
    let rng = rand::SystemRandom::new();
    let keypair =
        Ed25519KeyPair::generate_pkcs8(&rng).map_err(|err| anyhow!("Cannot generate keys"))?;
    Ok(Ed25519KeyPair::from_pkcs8(keypair.as_ref()).map_err(|err| anyhow!("Cannot parse keys"))?)
}
pub fn get_public_key(keypair: &KeyPair) -> Result<Vec<u8>> {
    Ok(keypair.public_key().as_ref().to_vec())
}
pub fn parse_public_key(data: &[u8]) -> PublicKey {
    UnparsedPublicKey::new(&ED25519, data.to_vec())
}
pub fn sign(keypair: &KeyPair, data: &[u8]) -> Vec<u8> {
    keypair.sign(data).as_ref().to_vec()
}
pub fn verify(public_key: &PublicKey, data: &[u8], signature: &[u8]) -> Result<()> {
    public_key
        .verify(data, signature)
        .map_err(|err| anyhow!("Verification failed"))?;
    Ok(())
}
