#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;
use rsa::pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey, LineEnding};
use rsa::rand_core::OsRng;
use rsa::sha2::Sha256;
use rsa::{Oaep, RsaPrivateKey, RsaPublicKey};

#[napi(object)]
pub struct KeyPairResult {
  pub public_key: String,
  pub private_key: String,
}

#[napi(object)]
#[derive(Clone)]
pub struct GenerateKeyPairOptions {
  pub modulus_length: Option<u32>,
  pub public_exponent: Option<u32>,
}

#[napi]
pub fn generate_key_pair_sync(
  key_type: String,
  options: Option<GenerateKeyPairOptions>,
) -> Result<KeyPairResult> {
  if key_type.to_lowercase() != "rsa" {
    return Err(Error::new(
      Status::InvalidArg,
      format!("Unsupported key type: {key_type}"),
    ));
  }

  let bits = options
    .as_ref()
    .and_then(|o| o.modulus_length)
    .unwrap_or(2048) as usize;

  let priv_key = RsaPrivateKey::new(&mut OsRng, bits)
    .map_err(|e| Error::new(Status::GenericFailure, format!("RSA gen failed: {e}")))?;
  let pub_key = RsaPublicKey::from(&priv_key);

  let priv_pem = priv_key
    .to_pkcs8_pem(LineEnding::LF)
    .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?
    .to_string();

  let pub_pem = pub_key
    .to_public_key_pem(LineEnding::LF)
    .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

  Ok(KeyPairResult {
    public_key: pub_pem,
    private_key: priv_pem,
  })
}

pub struct KeyPairTask {
  key_type: String,
  options: Option<GenerateKeyPairOptions>,
}

#[napi]
impl Task for KeyPairTask {
  type Output = KeyPairResult;
  type JsValue = KeyPairResult;

  fn compute(&mut self) -> Result<Self::Output> {
    generate_key_pair_sync(self.key_type.clone(), self.options.clone())
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi]
pub fn generate_key_pair(
  key_type: String,
  options: Option<GenerateKeyPairOptions>,
) -> AsyncTask<KeyPairTask> {
  AsyncTask::new(KeyPairTask { key_type, options })
}

#[napi]
pub fn public_encrypt(key_pem: String, buffer: Either<String, &[u8]>) -> Result<Buffer> {
  let pub_key = RsaPublicKey::from_public_key_pem(&key_pem)
    .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid public key PEM: {e}")))?;

  let data = match buffer {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.to_vec(),
  };

  let padding = Oaep::new::<Sha256>();
  let enc = pub_key
    .encrypt(&mut OsRng, padding, &data)
    .map_err(|e| Error::new(Status::GenericFailure, format!("Encryption failed: {e}")))?;

  Ok(Buffer::from(enc))
}

#[napi]
pub fn private_decrypt(key_pem: String, buffer: Either<String, &[u8]>) -> Result<Buffer> {
  let priv_key = RsaPrivateKey::from_pkcs8_pem(&key_pem)
    .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid private key PEM: {e}")))?;

  let data = match buffer {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.to_vec(),
  };

  let padding = Oaep::new::<Sha256>();
  let dec = priv_key
    .decrypt(padding, &data)
    .map_err(|e| Error::new(Status::GenericFailure, format!("Decryption failed: {e}")))?;

  Ok(Buffer::from(dec))
}

#[napi]
pub fn public_decrypt(key_pem: String, buffer: Either<String, &[u8]>) -> Result<Buffer> {
  let pub_key = RsaPublicKey::from_public_key_pem(&key_pem)
    .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid public key PEM: {e}")))?;

  let data = match buffer {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.to_vec(),
  };

  let padding = Oaep::new::<Sha256>();
  let enc = pub_key
    .encrypt(&mut OsRng, padding, &data)
    .map_err(|e| Error::new(Status::GenericFailure, format!("Public decrypt failed: {e}")))?;

  Ok(Buffer::from(enc))
}

#[napi]
pub fn private_encrypt(key_pem: String, buffer: Either<String, &[u8]>) -> Result<Buffer> {
  let priv_key = RsaPrivateKey::from_pkcs8_pem(&key_pem)
    .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid private key PEM: {e}")))?;

  let data = match buffer {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.to_vec(),
  };

  let padding = Oaep::new::<Sha256>();
  let dec = priv_key
    .decrypt(padding, &data)
    .map_err(|e| Error::new(Status::GenericFailure, format!("Private encrypt failed: {e}")))?;

  Ok(Buffer::from(dec))
}
