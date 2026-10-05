#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;
use md5::Md5;
use sha1::Sha1;
use sha2::{Digest, Sha256, Sha384, Sha512};

fn to_input_bytes(data: Either<String, Buffer>, encoding: Option<String>) -> Result<Vec<u8>> {
  match data {
    Either::A(s) => {
      let enc = encoding.unwrap_or_else(|| "utf8".to_string());
      match enc.to_lowercase().as_str() {
        "hex" => hex::decode(&s).map_err(|e: hex::FromHexError| Error::new(Status::InvalidArg, e.to_string())),
        "base64" => {
          use base64::Engine;
          base64::engine::general_purpose::STANDARD
            .decode(&s)
            .map_err(|e: base64::DecodeError| Error::new(Status::InvalidArg, e.to_string()))
        }
        _ => Ok(s.into_bytes()),
      }
    }
    Either::B(b) => Ok(b.to_vec()),
  }
}

fn encode_output(bytes: &[u8], encoding: Option<String>) -> Result<Either<String, Buffer>> {
  match encoding {
    Some(enc) => match enc.to_lowercase().as_str() {
      "hex" => Ok(Either::A(hex::encode(bytes))),
      "base64" => {
        use base64::Engine;
        Ok(Either::A(
          base64::engine::general_purpose::STANDARD.encode(bytes),
        ))
      }
      "utf8" | "utf-8" => Ok(Either::A(String::from_utf8_lossy(bytes).to_string())),
      "buffer" | "" => Ok(Either::B(Buffer::from(bytes))),
      _ => Ok(Either::A(String::from_utf8_lossy(bytes).to_string())),
    },
    None => Ok(Either::B(Buffer::from(bytes))),
  }
}

enum HasherImpl {
  Md5(Md5),
  Sha1(Sha1),
  Sha256(Sha256),
  Sha384(Sha384),
  Sha512(Sha512),
}

impl HasherImpl {
  fn new(alg: &str) -> Result<Self> {
    match alg.to_lowercase().replace("-", "").as_str() {
      "md5" => Ok(HasherImpl::Md5(Md5::new())),
      "sha1" => Ok(HasherImpl::Sha1(Sha1::new())),
      "sha256" => Ok(HasherImpl::Sha256(Sha256::new())),
      "sha384" => Ok(HasherImpl::Sha384(Sha384::new())),
      "sha512" => Ok(HasherImpl::Sha512(Sha512::new())),
      _ => Err(Error::new(
        Status::InvalidArg,
        format!("Unsupported hash algorithm: {}", alg),
      )),
    }
  }

  fn update(&mut self, data: &[u8]) {
    match self {
      HasherImpl::Md5(h) => h.update(data),
      HasherImpl::Sha1(h) => h.update(data),
      HasherImpl::Sha256(h) => h.update(data),
      HasherImpl::Sha384(h) => h.update(data),
      HasherImpl::Sha512(h) => h.update(data),
    }
  }

  fn finalize(self) -> Vec<u8> {
    match self {
      HasherImpl::Md5(h) => h.finalize().to_vec(),
      HasherImpl::Sha1(h) => h.finalize().to_vec(),
      HasherImpl::Sha256(h) => h.finalize().to_vec(),
      HasherImpl::Sha384(h) => h.finalize().to_vec(),
      HasherImpl::Sha512(h) => h.finalize().to_vec(),
    }
  }
}

#[napi]
pub struct Hash {
  algorithm: String,
  hasher: Option<HasherImpl>,
  finalized: bool,
}

#[napi]
impl Hash {
  #[napi(constructor)]
  pub fn new(algorithm: String, _options: Option<serde_json::Value>) -> Result<Self> {
    let hasher = HasherImpl::new(&algorithm)?;
    Ok(Self {
      algorithm,
      hasher: Some(hasher),
      finalized: false,
    })
  }

  #[napi]
  pub fn update(
    &mut self,
    data: Either<String, Buffer>,
    input_encoding: Option<String>,
  ) -> Result<&Self> {
    if self.finalized {
      return Err(Error::new(
        Status::GenericFailure,
        "Digest already called",
      ));
    }
    let bytes = to_input_bytes(data, input_encoding)?;
    if let Some(ref mut h) = self.hasher {
      h.update(&bytes);
    }
    Ok(self)
  }

  #[napi]
  pub fn digest(&mut self, encoding: Option<String>) -> Result<Either<String, Buffer>> {
    if self.finalized {
      return Err(Error::new(
        Status::GenericFailure,
        "Digest already called",
      ));
    }
    self.finalized = true;
    let hasher = self.hasher.take().ok_or_else(|| {
      Error::new(Status::GenericFailure, "Hasher context unavailable")
    })?;
    let res = hasher.finalize();
    encode_output(&res, encoding)
  }
}

#[napi(js_name = "createHash")]
pub fn create_hash(
  algorithm: String,
  options: Option<serde_json::Value>,
) -> Result<Hash> {
  Hash::new(algorithm, options)
}

#[napi(js_name = "hash")]
pub fn hash_one_shot(
  algorithm: String,
  data: Either<String, Buffer>,
  options: Option<String>,
) -> Result<Either<String, Buffer>> {
  let mut h = Hash::new(algorithm, None)?;
  h.update(data, None)?;
  h.digest(options)
}
