#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;

fn parse_hmac_algorithm(alg: &str) -> Result<ring::hmac::Algorithm> {
  match alg.to_lowercase().replace("-", "").as_str() {
    "sha1" => Ok(ring::hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY),
    "sha256" => Ok(ring::hmac::HMAC_SHA256),
    "sha384" => Ok(ring::hmac::HMAC_SHA384),
    "sha512" => Ok(ring::hmac::HMAC_SHA512),
    _ => Err(Error::new(
      Status::InvalidArg,
      format!("Unsupported HMAC algorithm: {}", alg),
    )),
  }
}

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

#[napi]
pub struct Hmac {
  ctx: Option<ring::hmac::Context>,
  finalized: bool,
}

#[napi]
impl Hmac {
  #[napi(constructor)]
  pub fn new(
    algorithm: String,
    key: Either<String, Buffer>,
    _options: Option<serde_json::Value>,
  ) -> Result<Self> {
    let alg = parse_hmac_algorithm(&algorithm)?;
    let key_bytes = to_input_bytes(key, None)?;
    let s_key = ring::hmac::Key::new(alg, &key_bytes);
    let ctx = ring::hmac::Context::with_key(&s_key);
    Ok(Self {
      ctx: Some(ctx),
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
    if let Some(ref mut ctx) = self.ctx {
      ctx.update(&bytes);
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
    let ctx = self.ctx.take().ok_or_else(|| {
      Error::new(Status::GenericFailure, "Hmac context unavailable")
    })?;
    let tag = ctx.sign();
    encode_output(tag.as_ref(), encoding)
  }
}

#[napi(js_name = "createHmac")]
pub fn create_hmac(
  algorithm: String,
  key: Either<String, Buffer>,
  options: Option<serde_json::Value>,
) -> Result<Hmac> {
  Hmac::new(algorithm, key, options)
}

#[napi]
pub struct Mac {
  hmac: Hmac,
}

#[napi]
impl Mac {
  #[napi(constructor)]
  pub fn new(
    algorithm: String,
    key: Either<String, Buffer>,
    options: Option<serde_json::Value>,
  ) -> Result<Self> {
    let hmac = Hmac::new(algorithm, key, options)?;
    Ok(Self { hmac })
  }

  #[napi]
  pub fn update(
    &mut self,
    data: Either<String, Buffer>,
    input_encoding: Option<String>,
  ) -> Result<&Self> {
    self.hmac.update(data, input_encoding)?;
    Ok(self)
  }

  #[napi(js_name = "final")]
  pub fn final_mac(&mut self, output_encoding: Option<String>) -> Result<Either<String, Buffer>> {
    self.hmac.digest(output_encoding)
  }
}

#[napi(js_name = "createMac")]
pub fn create_mac(
  algorithm: String,
  key: Either<String, Buffer>,
  options: Option<serde_json::Value>,
) -> Result<Mac> {
  Mac::new(algorithm, key, options)
}
