use napi::bindgen_prelude::*;
use napi_derive::napi;
use hkdf::Hkdf;
use sha2::{Sha256, Sha384, Sha512};
use sha1::Sha1;

pub fn hkdf_derive_internal(
  digest: &str,
  ikm: &[u8],
  salt: &[u8],
  info: &[u8],
  keylen: usize,
) -> Result<Vec<u8>> {
  let mut okm = vec![0u8; keylen];

  match digest.to_lowercase().as_str() {
    "sha256" => {
      let hk = Hkdf::<Sha256>::new(Some(salt), ikm);
      hk.expand(info, &mut okm)
        .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
    }
    "sha384" => {
      let hk = Hkdf::<Sha384>::new(Some(salt), ikm);
      hk.expand(info, &mut okm)
        .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
    }
    "sha512" => {
      let hk = Hkdf::<Sha512>::new(Some(salt), ikm);
      hk.expand(info, &mut okm)
        .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
    }
    "sha1" => {
      let hk = Hkdf::<Sha1>::new(Some(salt), ikm);
      hk.expand(info, &mut okm)
        .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
    }
    other => {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Unsupported HKDF digest: {}", other),
      ));
    }
  }

  Ok(okm)
}

#[napi]
pub fn hkdf_sync(
  digest: String,
  ikm: Either<Buffer, String>,
  salt: Either<Buffer, String>,
  info: Either<Buffer, String>,
  keylen: u32,
) -> Result<Buffer> {
  let ikm_bytes = match ikm {
    Either::A(b) => b.to_vec(),
    Either::B(s) => s.into_bytes(),
  };
  let salt_bytes = match salt {
    Either::A(b) => b.to_vec(),
    Either::B(s) => s.into_bytes(),
  };
  let info_bytes = match info {
    Either::A(b) => b.to_vec(),
    Either::B(s) => s.into_bytes(),
  };

  let derived = hkdf_derive_internal(&digest, &ikm_bytes, &salt_bytes, &info_bytes, keylen as usize)?;
  Ok(Buffer::from(derived))
}

pub struct HkdfTask {
  digest: String,
  ikm: Vec<u8>,
  salt: Vec<u8>,
  info: Vec<u8>,
  keylen: usize,
}

#[napi]
impl Task for HkdfTask {
  type Output = Vec<u8>;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    hkdf_derive_internal(&self.digest, &self.ikm, &self.salt, &self.info, self.keylen)
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(Buffer::from(output))
  }
}

#[napi]
pub fn hkdf(
  digest: String,
  ikm: Either<Buffer, String>,
  salt: Either<Buffer, String>,
  info: Either<Buffer, String>,
  keylen: u32,
) -> AsyncTask<HkdfTask> {
  let ikm_bytes = match ikm {
    Either::A(b) => b.to_vec(),
    Either::B(s) => s.into_bytes(),
  };
  let salt_bytes = match salt {
    Either::A(b) => b.to_vec(),
    Either::B(s) => s.into_bytes(),
  };
  let info_bytes = match info {
    Either::A(b) => b.to_vec(),
    Either::B(s) => s.into_bytes(),
  };

  AsyncTask::new(HkdfTask {
    digest,
    ikm: ikm_bytes,
    salt: salt_bytes,
    info: info_bytes,
    keylen: keylen as usize,
  })
}
