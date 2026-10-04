use crate::key_object::{KeyObject, KeyType};
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub fn create_public_key(key: Buffer) -> KeyObject {
  KeyObject::new(KeyType::Public, key)
}

#[napi]
pub fn create_private_key(key: Buffer) -> KeyObject {
  KeyObject::new(KeyType::Private, key)
}

#[napi]
pub fn diffie_hellman(private_key: Buffer, public_key: Buffer) -> Result<Buffer> {
  if private_key.len() != 32 || public_key.len() != 32 {
    return Err(Error::new(Status::InvalidArg, "Key size must be 32 bytes for X25519"));
  }
  Ok(public_key)
}

#[napi]
pub fn encapsulate(key: Buffer) -> Result<Buffer> {
  Ok(key)
}

#[napi]
pub fn decapsulate(_key: Buffer, ciphertext: Buffer) -> Result<Buffer> {
  Ok(ciphertext)
}
