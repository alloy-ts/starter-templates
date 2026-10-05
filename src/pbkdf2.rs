use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::pbkdf2;
use std::num::NonZeroU32;

pub fn pbkdf2_derive_internal(
  password: &[u8],
  salt: &[u8],
  iterations: u32,
  keylen: usize,
  digest_algo: &str,
) -> Result<Vec<u8>> {
  let algo = match digest_algo.to_lowercase().as_str() {
    "sha256" => pbkdf2::PBKDF2_HMAC_SHA256,
    "sha384" => pbkdf2::PBKDF2_HMAC_SHA384,
    "sha512" => pbkdf2::PBKDF2_HMAC_SHA512,
    "sha1" => pbkdf2::PBKDF2_HMAC_SHA1,
    other => {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Unsupported PBKDF2 digest algorithm: {}", other),
      ))
    }
  };

  let iter = NonZeroU32::new(iterations).ok_or_else(|| {
    Error::new(Status::InvalidArg, "Iterations must be greater than 0")
  })?;

  let mut out = vec![0u8; keylen];
  pbkdf2::derive(algo, iter, salt, password, &mut out);

  Ok(out)
}

#[napi]
pub fn pbkdf2_sync(
  password: Either<Buffer, String>,
  salt: Either<Buffer, String>,
  iterations: u32,
  keylen: u32,
  digest: String,
) -> Result<Buffer> {
  let pass_bytes = match &password {
    Either::A(b) => b.as_ref(),
    Either::B(s) => s.as_bytes(),
  };
  let salt_bytes = match &salt {
    Either::A(b) => b.as_ref(),
    Either::B(s) => s.as_bytes(),
  };

  let res = pbkdf2_derive_internal(
    pass_bytes,
    salt_bytes,
    iterations,
    keylen as usize,
    &digest,
  )?;
  Ok(Buffer::from(res))
}

pub struct Pbkdf2Task {
  password: Vec<u8>,
  salt: Vec<u8>,
  iterations: u32,
  keylen: usize,
  digest: String,
}

#[napi]
impl Task for Pbkdf2Task {
  type Output = Vec<u8>;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    pbkdf2_derive_internal(
      &self.password,
      &self.salt,
      self.iterations,
      self.keylen,
      &self.digest,
    )
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(Buffer::from(output))
  }
}

#[napi]
pub fn pbkdf2(
  password: Either<Buffer, String>,
  salt: Either<Buffer, String>,
  iterations: u32,
  keylen: u32,
  digest: String,
) -> AsyncTask<Pbkdf2Task> {
  let pass_bytes = match password {
    Either::A(b) => b.to_vec(),
    Either::B(s) => s.into_bytes(),
  };
  let salt_bytes = match salt {
    Either::A(b) => b.to_vec(),
    Either::B(s) => s.into_bytes(),
  };

  AsyncTask::new(Pbkdf2Task {
    password: pass_bytes,
    salt: salt_bytes,
    iterations,
    keylen: keylen as usize,
    digest,
  })
}
