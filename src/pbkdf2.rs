#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::num::NonZeroU32;

fn pbkdf2_derive(
  password: &[u8],
  salt: &[u8],
  iterations: u32,
  keylen: usize,
  digest: &str,
) -> Result<Vec<u8>> {
  let nonzero_iter = NonZeroU32::new(iterations).ok_or_else(|| {
    Error::new(
      Status::InvalidArg,
      "Iterations must be greater than 0".to_string(),
    )
  })?;

  let mut out = vec![0u8; keylen];
  let alg_lower = digest.to_lowercase();

  match alg_lower.as_str() {
    "sha1" => {
      ring::pbkdf2::derive(
        ring::pbkdf2::PBKDF2_HMAC_SHA1,
        nonzero_iter,
        salt,
        password,
        &mut out,
      );
    }
    "sha256" => {
      ring::pbkdf2::derive(
        ring::pbkdf2::PBKDF2_HMAC_SHA256,
        nonzero_iter,
        salt,
        password,
        &mut out,
      );
    }
    "sha384" => {
      ring::pbkdf2::derive(
        ring::pbkdf2::PBKDF2_HMAC_SHA384,
        nonzero_iter,
        salt,
        password,
        &mut out,
      );
    }
    "sha512" => {
      ring::pbkdf2::derive(
        ring::pbkdf2::PBKDF2_HMAC_SHA512,
        nonzero_iter,
        salt,
        password,
        &mut out,
      );
    }
    "md5" => {
      pbkdf2::pbkdf2_hmac::<md5::Md5>(password, salt, iterations, &mut out);
    }
    _ => {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Unsupported PBKDF2 digest: {digest}"),
      ));
    }
  }

  Ok(out)
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
    pbkdf2_derive(
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

fn to_bytes(input: Either<String, &[u8]>) -> Vec<u8> {
  match input {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.to_vec(),
  }
}

#[napi]
pub fn pbkdf2_sync(
  password: Either<String, &[u8]>,
  salt: Either<String, &[u8]>,
  iterations: u32,
  keylen: u32,
  digest: String,
) -> Result<Buffer> {
  let pass_bytes = to_bytes(password);
  let salt_bytes = to_bytes(salt);
  let out = pbkdf2_derive(
    &pass_bytes,
    &salt_bytes,
    iterations,
    keylen as usize,
    &digest,
  )?;
  Ok(Buffer::from(out))
}

#[napi]
pub fn pbkdf2(
  password: Either<String, &[u8]>,
  salt: Either<String, &[u8]>,
  iterations: u32,
  keylen: u32,
  digest: String,
) -> AsyncTask<Pbkdf2Task> {
  AsyncTask::new(Pbkdf2Task {
    password: to_bytes(password),
    salt: to_bytes(salt),
    iterations,
    keylen: keylen as usize,
    digest,
  })
}
