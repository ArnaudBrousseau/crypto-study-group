use anyhow::Result;
use ed25519_dalek::{Signer, Verifier};
use rand_core::OsRng;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Ed25519Error {
    #[error("Error while signing or verifying with ed25519_dalek")]
    SignatureError(#[from] ed25519_dalek::SignatureError),
}

// States
// Transaction empty
// Transaction constructed
// Transaction w/ signer attached
// Transaction signed
// Transaction broadcast

fn ed25519_sign(msg: &str) -> Result<Vec<u8>> {
    let msg_bytes = msg.as_bytes();
    let signing_key = ed25519_dalek::Keypair::generate(&mut OsRng);
    let signature = signing_key.sign(msg_bytes);

    let verify_key = signing_key.public;
    verify_key
        .verify(msg_bytes, &signature)
        .map_err(Ed25519Error::from)?;

    Ok(signature.to_bytes().to_vec())
}

fn main() -> Result<()> {
    let signature = ed25519_sign("Hello, World!")?;
    let hex_signature = hex::encode(signature);
    println!("ed25519 signature: {hex_signature}");
    Ok(())
}
