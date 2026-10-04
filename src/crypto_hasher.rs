use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::digest::{Context, SHA1_FOR_LEGACY_USE_ONLY, SHA256, SHA384, SHA512};

#[napi]
pub struct CryptoHasher {
  ctx: Option<Context>,
}

#[napi]
impl CryptoHasher {
  #[napi(constructor)]
  pub fn new(algorithm: String) -> Result<Self> {
    let algo = match algorithm.to_lowercase().as_str() {
      "sha1" => &SHA1_FOR_LEGACY_USE_ONLY,
      "sha256" => &SHA256,
      "sha384" => &SHA384,
      "sha512" => &SHA512,
      _ => return Err(Error::new(Status::InvalidArg, format!("Unsupported hash algorithm: {}", algorithm))),
    };

    Ok(CryptoHasher {
      ctx: Some(Context::new(algo)),
    })
  }

  #[napi]
  pub fn update(&mut self, data: Buffer) -> Result<()> {
    if let Some(ctx) = &mut self.ctx {
      ctx.update(&data);
      Ok(())
    } else {
      Err(Error::new(Status::GenericFailure, "Hash digest already finalized"))
    }
  }

  #[napi]
  pub fn digest(&mut self, encoding: Option<String>) -> Result<Either<String, Buffer>> {
    let ctx = self
      .ctx
      .take()
      .ok_or_else(|| Error::new(Status::GenericFailure, "Hash digest already finalized"))?;

    let digest = ctx.finish();
    let bytes = digest.as_ref();

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
pub fn create_hash(algorithm: String) -> Result<CryptoHasher> {
  CryptoHasher::new(algorithm)
}

#[napi]
pub fn get_hashes() -> Vec<String> {
  vec![
    "sha1".to_string(),
    "sha256".to_string(),
    "sha384".to_string(),
    "sha512".to_string(),
  ]
}
