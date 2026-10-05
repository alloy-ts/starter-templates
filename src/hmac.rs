use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::hmac::{Context, Key, HMAC_SHA1_FOR_LEGACY_USE_ONLY, HMAC_SHA256, HMAC_SHA384, HMAC_SHA512};

#[napi]
pub struct Hmac {
  ctx: Option<Context>,
}

#[napi]
impl Hmac {
  #[napi(constructor)]
  pub fn new(algorithm: String, key: Buffer) -> Result<Self> {
    let algo = match algorithm.to_lowercase().as_str() {
      "sha1" => HMAC_SHA1_FOR_LEGACY_USE_ONLY,
      "sha256" => HMAC_SHA256,
      "sha384" => HMAC_SHA384,
      "sha512" => HMAC_SHA512,
      _ => return Err(Error::new(Status::InvalidArg, format!("Unsupported HMAC algorithm: {}", algorithm))),
    };

    let s_key = Key::new(algo, &key);
    let ctx = Context::with_key(&s_key);

    Ok(Hmac { ctx: Some(ctx) })
  }

  #[napi]
  pub fn update(&mut self, data: Buffer) -> Result<()> {
    if let Some(ctx) = &mut self.ctx {
      ctx.update(&data);
      Ok(())
    } else {
      Err(Error::new(Status::GenericFailure, "HMAC digest already finalized"))
    }
  }

  #[napi]
  pub fn digest(&mut self, encoding: Option<String>) -> Result<Either<String, Buffer>> {
    let ctx = self
      .ctx
      .take()
      .ok_or_else(|| Error::new(Status::GenericFailure, "HMAC digest already finalized"))?;

    let tag = ctx.sign();
    let bytes = tag.as_ref();

    if let Some(enc) = encoding {
      match enc.to_lowercase().as_str() {
        "hex" => Ok(Either::A(hex::encode(bytes))),
        _ => Ok(Either::B(Buffer::from(bytes))),
      }
    } else {
      Ok(Either::B(Buffer::from(bytes)))
    }
  }
}

#[napi]
pub fn create_hmac(algorithm: String, key: Buffer) -> Result<Hmac> {
  Hmac::new(algorithm, key)
}
