use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::digest as ring_digest;
use sha1::Sha1;
use md5::Md5;
use sha2::Digest;

#[napi]
pub struct Hash {
  algorithm: String,
  data: Vec<u8>,
}

#[napi]
impl Hash {
  #[napi(constructor)]
  pub fn new(algorithm: String) -> Result<Self> {
    Ok(Self {
      algorithm: algorithm.to_lowercase(),
      data: Vec::new(),
    })
  }

  #[napi]
  pub fn update(&mut self, data: Either<Buffer, String>, encoding: Option<String>) -> Result<&Self> {
    match data {
      Either::A(buf) => self.data.extend_from_slice(&buf),
      Either::B(s) => {
        let enc = encoding.unwrap_or_else(|| "utf8".to_string());
        if enc == "hex" {
          if let Ok(bytes) = hex::decode(&s) {
            self.data.extend_from_slice(&bytes);
          } else {
            self.data.extend_from_slice(s.as_bytes());
          }
        } else if enc == "base64" {
          if let Ok(bytes) = base64_decode(&s) {
            self.data.extend_from_slice(&bytes);
          } else {
            self.data.extend_from_slice(s.as_bytes());
          }
        } else {
          self.data.extend_from_slice(s.as_bytes());
        }
      }
    }
    Ok(self)
  }

  #[napi]
  pub fn digest(&mut self, output_encoding: Option<String>) -> Result<Either<Buffer, String>> {
    let raw = compute_hash(&self.algorithm, &self.data)?;
    self.data.clear();

    match output_encoding.as_deref() {
      Some("hex") => Ok(Either::B(hex::encode(&raw))),
      Some("base64") => Ok(Either::B(base64_encode(&raw))),
      Some("latin1") | Some("binary") => {
        let s: String = raw.iter().map(|&b| b as char).collect();
        Ok(Either::B(s))
      }
      _ => Ok(Either::A(Buffer::from(raw))),
    }
  }
}

#[napi]
pub fn create_hash(algorithm: String) -> Result<Hash> {
  Hash::new(algorithm)
}

#[napi]
pub fn hash(algorithm: String, data: Either<Buffer, String>, output_encoding: Option<String>) -> Result<Either<Buffer, String>> {
  let mut h = Hash::new(algorithm)?;
  h.update(data, None)?;
  h.digest(output_encoding)
}

#[napi]
pub fn get_hashes() -> Vec<String> {
  vec![
    "md5".to_string(),
    "sha1".to_string(),
    "sha256".to_string(),
    "sha384".to_string(),
    "sha512".to_string(),
    "sha512_256".to_string(),
  ]
}

pub(crate) fn compute_hash(algo: &str, data: &[u8]) -> Result<Vec<u8>> {
  match algo {
    "sha256" => Ok(ring_digest::digest(&ring_digest::SHA256, data).as_ref().to_vec()),
    "sha384" => Ok(ring_digest::digest(&ring_digest::SHA384, data).as_ref().to_vec()),
    "sha512" => Ok(ring_digest::digest(&ring_digest::SHA512, data).as_ref().to_vec()),
    "sha512_256" => Ok(ring_digest::digest(&ring_digest::SHA512_256, data).as_ref().to_vec()),
    "sha1" => {
      let mut hasher = Sha1::new();
      hasher.update(data);
      Ok(hasher.finalize().to_vec())
    }
    "md5" => {
      let mut hasher = Md5::new();
      hasher.update(data);
      Ok(hasher.finalize().to_vec())
    }
    _ => Err(Error::new(Status::InvalidArg, format!("Unsupported hash algorithm: {}", algo))),
  }
}

pub(crate) fn base64_encode(data: &[u8]) -> String {
  use base64::Engine;
  base64::engine::general_purpose::STANDARD.encode(data)
}

pub(crate) fn base64_decode(data: &str) -> Result<Vec<u8>> {
  use base64::Engine;
  base64::engine::general_purpose::STANDARD.decode(data).map_err(|e| Error::new(Status::InvalidArg, e.to_string()))
}
