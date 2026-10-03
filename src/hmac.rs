#![deny(clippy::all)]

use hmac::KeyInit;
use napi::bindgen_prelude::*;
use napi_derive::napi;

enum HmacState {
  Ring(ring::hmac::Context),
  Md5(hmac::Hmac<md5::Md5>),
}

#[napi]
pub struct Hmac {
  state: Option<HmacState>,
}

#[napi]
impl Hmac {
  #[napi(constructor)]
  pub fn new(algorithm: String, key: Either<String, &[u8]>) -> Result<Self> {
    let key_bytes = match key {
      Either::A(s) => s.into_bytes(),
      Either::B(b) => b.to_vec(),
    };

    let alg_lower = algorithm.to_lowercase();
    let state = match alg_lower.as_str() {
      "sha1" => {
        let key = ring::hmac::Key::new(ring::hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY, &key_bytes);
        HmacState::Ring(ring::hmac::Context::with_key(&key))
      }
      "sha256" => {
        let key = ring::hmac::Key::new(ring::hmac::HMAC_SHA256, &key_bytes);
        HmacState::Ring(ring::hmac::Context::with_key(&key))
      }
      "sha384" => {
        let key = ring::hmac::Key::new(ring::hmac::HMAC_SHA384, &key_bytes);
        HmacState::Ring(ring::hmac::Context::with_key(&key))
      }
      "sha512" => {
        let key = ring::hmac::Key::new(ring::hmac::HMAC_SHA512, &key_bytes);
        HmacState::Ring(ring::hmac::Context::with_key(&key))
      }
      "md5" => {
        use hmac::Mac;
        let mac = hmac::Hmac::<md5::Md5>::new_from_slice(&key_bytes)
          .map_err(|e: hmac::digest::InvalidLength| Error::new(Status::InvalidArg, e.to_string()))?;
        HmacState::Md5(mac)
      }
      _ => {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Unsupported HMAC algorithm: {algorithm}"),
        ));
      }
    };

    Ok(Hmac { state: Some(state) })
  }

  #[napi]
  pub fn update(&mut self, data: Either<String, &[u8]>, encoding: Option<String>) -> Result<()> {
    let bytes = match data {
      Either::A(s) => match encoding.as_deref() {
        Some("hex") => hex::decode(&s).map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?,
        Some("base64") => {
          use base64::Engine;
          base64::engine::general_purpose::STANDARD
            .decode(&s)
            .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?
        }
        _ => s.into_bytes(),
      },
      Either::B(b) => b.to_vec(),
    };

    match self.state.as_mut() {
      Some(HmacState::Ring(ctx)) => {
        ctx.update(&bytes);
        Ok(())
      }
      Some(HmacState::Md5(mac)) => {
        use hmac::Mac;
        mac.update(&bytes);
        Ok(())
      }
      None => Err(Error::new(
        Status::GenericFailure,
        "Hmac object already finalized",
      )),
    }
  }

  #[napi]
  pub fn digest(&mut self, encoding: Option<String>) -> Result<Either<String, Buffer>> {
    let state = self
      .state
      .take()
      .ok_or_else(|| Error::new(Status::GenericFailure, "Hmac object already finalized"))?;

    let tag_bytes: Vec<u8> = match state {
      HmacState::Ring(ctx) => ctx.sign().as_ref().to_vec(),
      HmacState::Md5(mac) => {
        use hmac::Mac;
        mac.finalize().into_bytes().to_vec()
      }
    };

    match encoding.as_deref() {
      Some("hex") => Ok(Either::A(hex::encode(tag_bytes))),
      Some("base64") => {
        use base64::Engine;
        Ok(Either::A(
          base64::engine::general_purpose::STANDARD.encode(tag_bytes),
        ))
      }
      _ => Ok(Either::B(Buffer::from(tag_bytes))),
    }
  }
}

#[napi]
pub fn create_hmac(algorithm: String, key: Either<String, &[u8]>) -> Result<Hmac> {
  Hmac::new(algorithm, key)
}
