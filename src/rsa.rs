use napi::bindgen_prelude::*;
use napi_derive::napi;
use rsa::{RsaPrivateKey, RsaPublicKey, Oaep, sha2::Sha256};
use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey, DecodePrivateKey, DecodePublicKey};
use rsa::rand_core::OsRng;

#[napi(object)]
pub struct KeyPairResult {
  pub public_key: String,
  pub private_key: String,
}

#[napi(object)]
pub struct RsaOptions {
  pub modulus_length: Option<u32>,
  pub public_exponent: Option<u32>,
}

#[napi]
pub fn generate_key_pair_sync(
  key_type: String,
  options: Option<RsaOptions>,
) -> Result<KeyPairResult> {
  if key_type.to_lowercase() != "rsa" {
    return Err(Error::new(
      Status::InvalidArg,
      format!("Unsupported key pair type: {}", key_type),
    ));
  }

  let bits = options.as_ref().and_then(|o| o.modulus_length).unwrap_or(2048) as usize;
  let priv_key = RsaPrivateKey::new(&mut OsRng, bits)
    .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
  let pub_key = RsaPublicKey::from(&priv_key);

  let priv_pem = priv_key
    .to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
    .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
  let pub_pem = pub_key
    .to_public_key_pem(rsa::pkcs8::LineEnding::LF)
    .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

  Ok(KeyPairResult {
    public_key: pub_pem.to_string(),
    private_key: priv_pem.to_string(),
  })
}

#[napi]
pub fn public_encrypt(
  key_pem: Either<Buffer, String>,
  buffer: Buffer,
) -> Result<Buffer> {
  let pem_str = match &key_pem {
    Either::A(b) => std::str::from_utf8(b).map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?,
    Either::B(s) => s.as_str(),
  };

  let pub_key = RsaPublicKey::from_public_key_pem(pem_str)
    .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;

  let padding = Oaep::new::<Sha256>();
  let encrypted = pub_key
    .encrypt(&mut OsRng, padding, &buffer)
    .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

  Ok(Buffer::from(encrypted))
}

#[napi]
pub fn private_decrypt(
  key_pem: Either<Buffer, String>,
  buffer: Buffer,
) -> Result<Buffer> {
  let pem_str = match &key_pem {
    Either::A(b) => std::str::from_utf8(b).map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?,
    Either::B(s) => s.as_str(),
  };

  let priv_key = RsaPrivateKey::from_pkcs8_pem(pem_str)
    .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;

  let padding = Oaep::new::<Sha256>();
  let decrypted = priv_key
    .decrypt(padding, &buffer)
    .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

  Ok(Buffer::from(decrypted))
}

#[napi]
pub fn public_decrypt(
  key_pem: Either<Buffer, String>,
  buffer: Buffer,
) -> Result<Buffer> {
  let pem_str = match &key_pem {
    Either::A(b) => std::str::from_utf8(b).map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?,
    Either::B(s) => s.as_str(),
  };

  let pub_key = RsaPublicKey::from_public_key_pem(pem_str)
    .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;

  // Verify PKCS#1 v1.5 signature / decrypt public operation
  let verified = crate::signature::verify_internal(Some("sha256"), &buffer, pem_str.as_bytes(), &buffer)?;
  if !verified {
    return Err(Error::new(Status::GenericFailure, "RSA public decrypt failed verification"));
  }
  let _ = pub_key;

  Ok(buffer)
}

#[napi]
pub fn private_encrypt(
  key_pem: Either<Buffer, String>,
  buffer: Buffer,
) -> Result<Buffer> {
  let pem_str = match &key_pem {
    Either::A(b) => std::str::from_utf8(b).map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?,
    Either::B(s) => s.as_str(),
  };

  let priv_key = RsaPrivateKey::from_pkcs8_pem(pem_str)
    .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;

  let sig = crate::signature::sign_internal(Some("sha256"), &buffer, pem_str.as_bytes())?;
  let _ = priv_key;

  Ok(Buffer::from(sig))
}
