use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::pbkdf2 as ring_pbkdf2;
use std::num::NonZeroU32;

#[napi]
pub fn pbkdf2_sync(
  password: Buffer,
  salt: Buffer,
  iterations: u32,
  keylen: u32,
  digest: String,
) -> Result<Buffer> {
  let algorithm = match digest.to_lowercase().as_str() {
    "sha1" => ring_pbkdf2::PBKDF2_HMAC_SHA1,
    "sha256" => ring_pbkdf2::PBKDF2_HMAC_SHA256,
    "sha384" => ring_pbkdf2::PBKDF2_HMAC_SHA384,
    "sha512" => ring_pbkdf2::PBKDF2_HMAC_SHA512,
    _ => return Err(Error::new(Status::InvalidArg, format!("Unsupported PBKDF2 digest: {}", digest))),
  };

  let iter = NonZeroU32::new(iterations)
    .ok_or_else(|| Error::new(Status::InvalidArg, "Iterations must be greater than 0"))?;

  let mut out = vec![0u8; keylen as usize];
  ring_pbkdf2::derive(algorithm, iter, &salt, &password, &mut out);

  Ok(Buffer::from(out))
}

#[napi]
pub fn pbkdf2(
  password: Buffer,
  salt: Buffer,
  iterations: u32,
  keylen: u32,
  digest: String,
) -> Result<Buffer> {
  pbkdf2_sync(password, salt, iterations, keylen, digest)
}
