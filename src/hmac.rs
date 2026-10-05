use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::hmac;

#[napi]
pub struct Hmac {
  ctx: hmac::Context,
}

#[napi]
impl Hmac {
  #[napi(constructor)]
  pub fn new(algorithm: String, key: Either<Buffer, String>) -> Result<Self> {
    let key_bytes = match key {
      Either::A(b) => b.to_vec(),
      Either::B(s) => s.into_bytes(),
    };

    let algo = match algorithm.to_lowercase().as_str() {
      "sha256" => hmac::HMAC_SHA256,
      "sha384" => hmac::HMAC_SHA384,
      "sha512" => hmac::HMAC_SHA512,
      "sha1" => hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY,
      other => {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Unsupported HMAC algorithm: {}", other),
        ))
      }
    };

    let hmac_key = hmac::Key::new(algo, &key_bytes);
    let ctx = hmac::Context::with_key(&hmac_key);

    Ok(Self { ctx })
  }

  #[napi]
  pub fn update(&mut self, data: Either<Buffer, String>, encoding: Option<String>) -> Result<&Self> {
    let bytes = match data {
      Either::A(b) => b.to_vec(),
      Either::B(s) => {
        let enc = encoding.unwrap_or_else(|| "utf8".to_string());
        if enc == "hex" {
          hex::decode(&s).unwrap_or_else(|_| s.as_bytes().to_vec())
        } else if enc == "base64" {
          crate::crypto_hasher::base64_decode(&s).unwrap_or_else(|_| s.as_bytes().to_vec())
        } else {
          s.into_bytes()
        }
      }
    };

    self.ctx.update(&bytes);
    Ok(self)
  }

  #[napi]
  pub fn digest(&mut self, output_encoding: Option<String>) -> Result<Either<Buffer, String>> {
    let tag = self.ctx.clone().sign();
    let raw = tag.as_ref();

    match output_encoding.as_deref() {
      Some("hex") => Ok(Either::B(hex::encode(raw))),
      Some("base64") => Ok(Either::B(crate::crypto_hasher::base64_encode(raw))),
      Some("latin1") | Some("binary") => {
        let s: String = raw.iter().map(|&b| b as char).collect();
        Ok(Either::B(s))
      }
      _ => Ok(Either::A(Buffer::from(raw))),
    }
  }
}

#[napi]
pub fn create_hmac(algorithm: String, key: Either<Buffer, String>) -> Result<Hmac> {
  Hmac::new(algorithm, key)
}
