use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::hkdf;

#[derive(Clone, Copy)]
struct HKDFKeyLen(usize);

impl hkdf::KeyType for HKDFKeyLen {
  fn len(&self) -> usize {
    self.0
  }
}

#[napi]
pub fn hkdf_sync(
  digest: String,
  ikm: Buffer,
  salt: Buffer,
  info: Buffer,
  keylen: u32,
) -> Result<Buffer> {
  let algo = match digest.to_lowercase().as_str() {
    "sha256" => hkdf::HKDF_SHA256,
    "sha384" => hkdf::HKDF_SHA384,
    "sha512" => hkdf::HKDF_SHA512,
    _ => return Err(Error::new(Status::InvalidArg, format!("Unsupported HKDF digest: {}", digest))),
  };

  let salt_key = hkdf::Salt::new(algo, &salt);
  let prk = salt_key.extract(&ikm);

  let info_slice: &[&[u8]] = &[&info];
  let okm = prk
    .expand(info_slice, HKDFKeyLen(keylen as usize))
    .map_err(|_| Error::new(Status::GenericFailure, "HKDF expand failed"))?;

  let mut out = vec![0u8; keylen as usize];
  okm
    .fill(&mut out)
    .map_err(|_| Error::new(Status::GenericFailure, "HKDF fill failed"))?;

  Ok(Buffer::from(out))
}

#[napi]
pub fn hkdf(
  digest: String,
  ikm: Buffer,
  salt: Buffer,
  info: Buffer,
  keylen: u32,
) -> Result<Buffer> {
  hkdf_sync(digest, ikm, salt, info, keylen)
}
