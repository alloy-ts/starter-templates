use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::digest;

enum DigestAlgo {
  Sha1,
  Sha256,
  Sha384,
  Sha512,
  Sha512_256,
}

fn parse_algorithm(algorithm: &str) -> Result<DigestAlgo> {
  match algorithm.to_lowercase().replace('-', "").as_str() {
    "sha1" => Ok(DigestAlgo::Sha1),
    "sha256" => Ok(DigestAlgo::Sha256),
    "sha384" => Ok(DigestAlgo::Sha384),
    "sha512" => Ok(DigestAlgo::Sha512),
    "sha512256" => Ok(DigestAlgo::Sha512_256),
    _ => Err(Error::new(
      Status::InvalidArg,
      format!("Unsupported hash algorithm: {algorithm}"),
    )),
  }
}

enum ContextWrapper {
  Ring(digest::Context),
}

fn input_to_bytes(data: Either<String, Uint8Array>) -> Vec<u8> {
  match data {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.as_ref().to_vec(),
  }
}

fn encode_digest(bytes: &[u8], encoding: Option<String>) -> Result<Either<String, Buffer>> {
  match encoding.as_deref() {
    Some("hex") => Ok(Either::A(hex::encode(bytes))),
    Some("base64") => Ok(Either::A(base64_encode(bytes))),
    Some("latin1") | Some("binary") => Ok(Either::A(
      bytes.iter().map(|&b| b as char).collect::<String>(),
    )),
    Some("utf8") | Some("utf-8") => {
      let s = std::str::from_utf8(bytes)
        .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid UTF-8: {e}")))?;
      Ok(Either::A(s.to_string()))
    }
    None | Some("buffer") => Ok(Either::B(Buffer::from(bytes))),
    Some(other) => Err(Error::new(
      Status::InvalidArg,
      format!("Unknown or unsupported output encoding: {other}"),
    )),
  }
}

fn base64_encode(input: &[u8]) -> String {
  const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
  let mut result = String::with_capacity((input.len() + 2) / 3 * 4);
  for chunk in input.chunks(3) {
    let b0 = chunk[0] as u32;
    let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
    let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };

    let triple = (b0 << 16) | (b1 << 8) | b2;

    result.push(CHARS[((triple >> 18) & 63) as usize] as char);
    result.push(CHARS[((triple >> 12) & 63) as usize] as char);

    if chunk.len() > 1 {
      result.push(CHARS[((triple >> 6) & 63) as usize] as char);
    } else {
      result.push('=');
    }

    if chunk.len() > 2 {
      result.push(CHARS[(triple & 63) as usize] as char);
    } else {
      result.push('=');
    }
  }
  result
}

#[napi]
pub struct Hash {
  context: ContextWrapper,
}

#[napi]
impl Hash {
  #[napi(constructor)]
  pub fn new(algorithm: String) -> Result<Self> {
    let algo = parse_algorithm(&algorithm)?;
    let ctx = match algo {
      DigestAlgo::Sha1 => digest::Context::new(&digest::SHA1_FOR_LEGACY_USE_ONLY),
      DigestAlgo::Sha256 => digest::Context::new(&digest::SHA256),
      DigestAlgo::Sha384 => digest::Context::new(&digest::SHA384),
      DigestAlgo::Sha512 => digest::Context::new(&digest::SHA512),
      DigestAlgo::Sha512_256 => digest::Context::new(&digest::SHA512_256),
    };
    Ok(Self {
      context: ContextWrapper::Ring(ctx),
    })
  }

  #[napi]
  pub fn update(&mut self, data: Either<String, Uint8Array>) -> &Self {
    let bytes = input_to_bytes(data);
    match &mut self.context {
      ContextWrapper::Ring(ctx) => ctx.update(&bytes),
    }
    self
  }

  #[napi]
  pub fn digest(&mut self, encoding: Option<String>) -> Result<Either<String, Buffer>> {
    match &self.context {
      ContextWrapper::Ring(ctx) => {
        let digest_val = ctx.clone().finish();
        encode_digest(digest_val.as_ref(), encoding)
      }
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
  output_encoding: Option<String>,
) -> Result<Either<String, Buffer>> {
  let mut hasher = Hash::new(algorithm)?;
  hasher.update(data);
  hasher.digest(output_encoding)
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
