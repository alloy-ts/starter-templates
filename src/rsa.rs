use napi::bindgen_prelude::*;
use napi_derive::napi;
use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey};
use rsa::RsaPrivateKey;

#[napi(object)]
pub struct KeyPairResult {
  pub public_key: String,
  pub private_key: String,
}

#[napi]
pub fn generate_key_pair_sync(key_type: String, modulus_length: u32) -> Result<KeyPairResult> {
  if key_type.to_lowercase() != "rsa" {
    return Err(Error::new(Status::InvalidArg, format!("Unsupported key type: {}", key_type)));
  }

  let mut rng = rand::thread_rng();
  let priv_key = RsaPrivateKey::new(&mut rng, modulus_length as usize)
    .map_err(|e| Error::new(Status::GenericFailure, format!("RSA generation error: {}", e)))?;

  let priv_pem = priv_key
    .to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
    .map_err(|e| Error::new(Status::GenericFailure, format!("PEM export error: {}", e)))?;

  let pub_key = priv_key.to_public_key();
  let pub_pem = pub_key
    .to_public_key_pem(rsa::pkcs8::LineEnding::LF)
    .map_err(|e| Error::new(Status::GenericFailure, format!("PEM export error: {}", e)))?;

  Ok(KeyPairResult {
    public_key: pub_pem.to_string(),
    private_key: priv_pem.to_string(),
  })
}

#[napi]
pub fn generate_key_pair(key_type: String, modulus_length: u32) -> Result<KeyPairResult> {
  generate_key_pair_sync(key_type, modulus_length)
}
