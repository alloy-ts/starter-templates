use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::hmac;

fn parse_algorithm(algorithm: &str) -> Result<hmac::Algorithm> {
  match algorithm.to_lowercase().replace('-', "").as_str() {
    "sha1" => Ok(hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY),
    "sha256" => Ok(hmac::HMAC_SHA256),
    "sha384" => Ok(hmac::HMAC_SHA384),
    "sha512" => Ok(hmac::HMAC_SHA512),
    _ => Err(Error::new(
      Status::InvalidArg,
      format!("Unsupported HMAC algorithm: {algorithm}"),
    )),
  }
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
pub struct Hmac {
  context: hmac::Context,
}

#[napi]
impl Hmac {
  #[napi(constructor)]
  pub fn new(algorithm: String, key: Either<String, Uint8Array>) -> Result<Self> {
    let alg = parse_algorithm(&algorithm)?;
    let key_bytes = input_to_bytes(key);
    let s_key = hmac::Key::new(alg, &key_bytes);
    let context = hmac::Context::with_key(&s_key);
    Ok(Self { context })
  }

  #[napi]
  pub fn update(&mut self, data: Either<String, Uint8Array>) -> &Self {
    let bytes = input_to_bytes(data);
    self.context.update(&bytes);
    self
  }

  #[napi]
  pub fn digest(&mut self, encoding: Option<String>) -> Result<Either<String, Buffer>> {
    let tag = self.context.clone().sign();
    encode_digest(tag.as_ref(), encoding)
  }
}

#[napi]
pub fn create_hmac(
  algorithm: String,
  key: Either<String, Uint8Array>,
) -> Result<Hmac> {
  Hmac::new(algorithm, key)
}
