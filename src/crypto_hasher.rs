#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::digest;

#[napi]
pub struct Hash {
  ctx: Option<digest::Context>,
}

#[napi]
impl Hash {
  #[napi(constructor)]
  pub fn new(algorithm: String) -> Result<Self> {
    let algo = match algorithm.to_lowercase().as_str() {
      "sha1" | "sha-1" => &digest::SHA1_FOR_LEGACY_USE_ONLY,
      "sha256" | "sha-256" => &digest::SHA256,
      "sha384" | "sha-384" => &digest::SHA384,
      "sha512" | "sha-512" => &digest::SHA512,
      "sha512-256" | "sha512_256" => &digest::SHA512_256,
      _ => {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Digest method not supported: {algorithm}"),
        ));
      }
    };

    Ok(Self {
      ctx: Some(digest::Context::new(algo)),
    })
  }

  #[napi]
  pub fn update(&mut self, data: Either<String, Uint8Array>) -> Result<&Self> {
    let ctx = self.ctx.as_mut().ok_or_else(|| {
      Error::new(
        Status::GenericFailure,
        "Hash object cannot be updated after digest() has been called",
      )
    })?;

    let bytes = match data {
      Either::A(s) => s.into_bytes(),
      Either::B(b) => b.to_vec(),
    };

    ctx.update(&bytes);
    Ok(self)
  }

  #[napi]
  pub fn digest(&mut self, encoding: Option<String>) -> Result<Either<String, Buffer>> {
    let ctx = self.ctx.take().ok_or_else(|| {
      Error::new(
        Status::GenericFailure,
        "Digest already called on this Hash instance",
      )
    })?;

    let val = ctx.finish();
    let bytes = val.as_ref();

    let encoding_str = encoding.as_deref().unwrap_or("buffer");
    match encoding_str {
      "hex" => Ok(Either::A(hex_encode(bytes))),
      "base64" => Ok(Either::A(base64_encode(bytes))),
      "buffer" | "" => Ok(Either::B(bytes.to_vec().into())),
      _ => Err(Error::new(
        Status::InvalidArg,
        format!("Unknown encoding: {encoding_str}"),
      )),
    }
  }
}

#[napi]
pub fn create_hash(algorithm: String) -> Result<Hash> {
  Hash::new(algorithm)
}

#[napi]
pub fn hash(
  algorithm: String,
  data: Either<String, Uint8Array>,
  encoding: Option<String>,
) -> Result<Either<String, Buffer>> {
  let mut h = Hash::new(algorithm)?;
  h.update(data)?;
  h.digest(encoding)
}

#[napi]
pub fn get_hashes() -> Vec<String> {
  vec![
    "sha1".to_string(),
    "sha256".to_string(),
    "sha384".to_string(),
    "sha512".to_string(),
    "sha512-256".to_string(),
  ]
}

fn hex_encode(data: &[u8]) -> String {
  let mut s = String::with_capacity(data.len() * 2);
  for &b in data {
    s.push_str(&format!("{b:02x}"));
  }
  s
}

fn base64_encode(data: &[u8]) -> String {
  const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
  let mut result = String::new();
  let mut i = 0;
  while i < data.len() {
    let b0 = data[i] as u32;
    let b1 = if i + 1 < data.len() { data[i + 1] as u32 } else { 0 };
    let b2 = if i + 2 < data.len() { data[i + 2] as u32 } else { 0 };

    let triplet = (b0 << 16) | (b1 << 8) | b2;

    result.push(ALPHABET[((triplet >> 18) & 63) as usize] as char);
    result.push(ALPHABET[((triplet >> 12) & 63) as usize] as char);

    if i + 1 < data.len() {
      result.push(ALPHABET[((triplet >> 6) & 63) as usize] as char);
    } else {
      result.push('=');
    }

    if i + 2 < data.len() {
      result.push(ALPHABET[(triplet & 63) as usize] as char);
    } else {
      result.push('=');
    }

    i += 3;
  }
  result
}
