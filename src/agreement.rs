use napi::bindgen_prelude::*;
use napi_derive::napi;
use crate::key_object::KeyObject;
use x25519_dalek::{EphemeralSecret, StaticSecret, PublicKey as X25519PublicKey};

#[napi]
pub fn create_secret_key(key: Either<Buffer, String>, _encoding: Option<String>) -> Result<KeyObject> {
  let raw = match key {
    Either::A(b) => b.to_vec(),
    Either::B(s) => s.into_bytes(),
  };
  Ok(KeyObject {
    key_type: "secret".to_string(),
    asymmetric_key_type_val: None,
    raw_data: raw,
  })
}

#[napi]
pub fn create_public_key(key: Either<Buffer, String>) -> Result<KeyObject> {
  let raw = match key {
    Either::A(b) => b.to_vec(),
    Either::B(s) => s.into_bytes(),
  };
  Ok(KeyObject {
    key_type: "public".to_string(),
    asymmetric_key_type_val: Some("rsa".to_string()),
    raw_data: raw,
  })
}

#[napi]
pub fn create_private_key(key: Either<Buffer, String>) -> Result<KeyObject> {
  let raw = match key {
    Either::A(b) => b.to_vec(),
    Either::B(s) => s.into_bytes(),
  };
  Ok(KeyObject {
    key_type: "private".to_string(),
    asymmetric_key_type_val: Some("rsa".to_string()),
    raw_data: raw,
  })
}

#[napi(object)]
pub struct EncapsulateResult {
  pub shared_key: Buffer,
  pub ciphertext: Buffer,
}

#[napi]
pub fn encapsulate(public_key: Either<Buffer, String>) -> Result<EncapsulateResult> {
  let raw = match public_key {
    Either::A(b) => b.to_vec(),
    Either::B(s) => hex::decode(&s).unwrap_or_else(|_| s.into_bytes()),
  };

  if raw.len() != 32 {
    return Err(Error::new(
      Status::InvalidArg,
      "Public key for X25519 encapsulation must be 32 bytes",
    ));
  }

  let mut pk_bytes = [0u8; 32];
  pk_bytes.copy_from_slice(&raw);
  let peer_public = X25519PublicKey::from(pk_bytes);

  let mut rng = rand::rng();
  let ephemeral_secret = EphemeralSecret::random_from_rng(&mut rng);
  let ephemeral_public = X25519PublicKey::from(&ephemeral_secret);
  let shared_secret = ephemeral_secret.diffie_hellman(&peer_public);

  Ok(EncapsulateResult {
    shared_key: Buffer::from(shared_secret.as_bytes().to_vec()),
    ciphertext: Buffer::from(ephemeral_public.as_bytes().to_vec()),
  })
}

#[napi]
pub fn decapsulate(
  private_key: Either<Buffer, String>,
  ciphertext: Buffer,
) -> Result<Buffer> {
  let priv_raw = match private_key {
    Either::A(b) => b.to_vec(),
    Either::B(s) => hex::decode(&s).unwrap_or_else(|_| s.into_bytes()),
  };

  if priv_raw.len() != 32 || ciphertext.len() != 32 {
    return Err(Error::new(
      Status::InvalidArg,
      "Private key and ciphertext for X25519 decapsulation must be 32 bytes",
    ));
  }

  let mut sk_bytes = [0u8; 32];
  sk_bytes.copy_from_slice(&priv_raw);
  let static_secret = StaticSecret::from(sk_bytes);

  let mut ct_bytes = [0u8; 32];
  ct_bytes.copy_from_slice(&ciphertext);
  let peer_public = X25519PublicKey::from(ct_bytes);

  let shared_secret = static_secret.diffie_hellman(&peer_public);
  Ok(Buffer::from(shared_secret.as_bytes().to_vec()))
}

#[napi]
pub fn create_mac(algorithm: String, key: Either<Buffer, String>) -> Result<crate::hmac::Hmac> {
  crate::hmac::Hmac::new(algorithm, key)
}

#[napi]
pub struct Verify {
  algorithm: String,
  data: Vec<u8>,
}

#[napi]
impl Verify {
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
  pub fn verify(
    &mut self,
    key: Either<Buffer, String>,
    signature: Buffer,
  ) -> Result<bool> {
    let key_bytes = match key {
      Either::A(b) => b.to_vec(),
      Either::B(s) => s.into_bytes(),
    };
    let is_valid = crate::signature::verify_internal(
      Some(&self.algorithm),
      &self.data,
      &key_bytes,
      &signature,
    )?;
    self.data.clear();
    Ok(is_valid)
  }
}

#[napi]
pub fn create_verify(algorithm: String) -> Verify {
  Verify::new(algorithm)
}
