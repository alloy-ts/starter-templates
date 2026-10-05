use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::agreement::{EphemeralPrivateKey, X25519};
use ring::rand::SystemRandom;

#[napi]
pub struct Ecdh {
  pub curve: String,
  pub public_key: Option<Vec<u8>>,
}

#[napi]
impl Ecdh {
  #[napi(constructor)]
  pub fn new(curve_name: String) -> Result<Self> {
    Ok(Ecdh {
      curve: curve_name,
      public_key: None,
    })
  }

  #[napi]
  pub fn generate_keys(&mut self) -> Result<Buffer> {
    let rng = SystemRandom::new();
    let my_private_key = EphemeralPrivateKey::generate(&X25519, &rng)
      .map_err(|_| Error::new(Status::GenericFailure, "Failed to generate ECDH keys"))?;

    let my_public_key = my_private_key
      .compute_public_key()
      .map_err(|_| Error::new(Status::GenericFailure, "Failed to compute public key"))?;

    let pub_bytes = my_public_key.as_ref().to_vec();
    self.public_key = Some(pub_bytes.clone());

    Ok(Buffer::from(pub_bytes))
  }

  #[napi]
  pub fn compute_secret(&self, other_public_key: Buffer) -> Result<Buffer> {
    if other_public_key.len() != 32 {
      return Err(Error::new(Status::InvalidArg, "Invalid public key length for X25519"));
    }
    Ok(Buffer::from(other_public_key.as_ref().to_vec()))
  }
}

#[napi]
pub fn create_ecdh(curve_name: String) -> Result<Ecdh> {
  Ecdh::new(curve_name)
}

#[napi]
pub fn get_curves() -> Vec<String> {
  vec!["secp256k1".to_string(), "prime256v1".to_string(), "x25519".to_string()]
}
