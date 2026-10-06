use napi::bindgen_prelude::*;
use napi_derive::napi;
use rsa::{RsaPrivateKey, RsaPublicKey, Pkcs1v15Sign, sha2::Sha256};
use rsa::pkcs8::{DecodePrivateKey, DecodePublicKey};

pub fn sign_internal(
  algorithm: Option<&str>,
  data: &[u8],
  key_bytes: &[u8],
) -> Result<Vec<u8>> {
  let algo = algorithm.unwrap_or("sha256").to_lowercase();
  let key_str = std::str::from_utf8(key_bytes)
    .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;

  if let Ok(priv_key) = RsaPrivateKey::from_pkcs8_pem(key_str) {
    let hashed = crate::crypto_hasher::compute_hash(&algo, data)?;
    let signing_key = Pkcs1v15Sign::new::<Sha256>();
    let sig = priv_key.sign(signing_key, &hashed)
      .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
    return Ok(sig);
  }

  let ring_key = ring::hmac::Key::new(ring::hmac::HMAC_SHA256, key_bytes);
  let tag = ring::hmac::sign(&ring_key, data);
  Ok(tag.as_ref().to_vec())
}

pub fn verify_internal(
  algorithm: Option<&str>,
  data: &[u8],
  key_bytes: &[u8],
  signature: &[u8],
) -> Result<bool> {
  let algo = algorithm.unwrap_or("sha256").to_lowercase();
  let key_str = std::str::from_utf8(key_bytes)
    .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;

  if let Ok(pub_key) = RsaPublicKey::from_public_key_pem(key_str) {
    let hashed = crate::crypto_hasher::compute_hash(&algo, data)?;
    let verifying_key = Pkcs1v15Sign::new::<Sha256>();
    return Ok(pub_key.verify(verifying_key, &hashed, signature).is_ok());
  }

  let ring_key = ring::hmac::Key::new(ring::hmac::HMAC_SHA256, key_bytes);
  Ok(ring::hmac::verify(&ring_key, data, signature).is_ok())
}

#[napi]
pub fn sign(
  algorithm: Option<String>,
  data: Either<Buffer, String>,
  key: Either<Buffer, String>,
) -> Result<Buffer> {
  let data_bytes = match data {
    Either::A(b) => b.to_vec(),
    Either::B(s) => s.into_bytes(),
  };
  let key_bytes = match key {
    Either::A(b) => b.to_vec(),
    Either::B(s) => s.into_bytes(),
  };

  let sig = sign_internal(algorithm.as_deref(), &data_bytes, &key_bytes)?;
  Ok(Buffer::from(sig))
}

#[napi]
pub fn verify(
  algorithm: Option<String>,
  data: Either<Buffer, String>,
  key: Either<Buffer, String>,
  signature: Buffer,
) -> Result<bool> {
  let data_bytes = match data {
    Either::A(b) => b.to_vec(),
    Either::B(s) => s.into_bytes(),
  };
  let key_bytes = match key {
    Either::A(b) => b.to_vec(),
    Either::B(s) => s.into_bytes(),
  };

  verify_internal(algorithm.as_deref(), &data_bytes, &key_bytes, &signature)
}

#[napi]
pub struct Sign {
  algorithm: String,
  data: Vec<u8>,
}

#[napi]
impl Sign {
  #[napi(constructor)]
  pub fn new(algorithm: String) -> Self {
    Self {
      algorithm,
      data: Vec::new(),
    }
  }

  #[napi]
  pub fn update(&mut self, data: Either<Buffer, String>, encoding: Option<String>) -> Result<&Self> {
    let bytes = match data {
      Either::A(b) => b.to_vec(),
      Either::B(s) => {
        if encoding.as_deref() == Some("hex") {
          hex::decode(&s).unwrap_or_else(|_| s.into_bytes())
        } else {
          s.into_bytes()
        }
      }
    };
    self.data.extend_from_slice(&bytes);
    Ok(self)
  }

  #[napi]
  pub fn sign(&mut self, key: Either<Buffer, String>) -> Result<Buffer> {
    let key_bytes = match key {
      Either::A(b) => b.to_vec(),
      Either::B(s) => s.into_bytes(),
    };
    let sig = sign_internal(Some(&self.algorithm), &self.data, &key_bytes)?;
    self.data.clear();
    Ok(Buffer::from(sig))
  }
}

#[napi]
pub fn create_sign(algorithm: String) -> Sign {
  Sign::new(algorithm)
}
