#![deny(clippy::all)]

use std::num::NonZeroU32;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::pbkdf2;

fn get_algorithm(digest: &str) -> Result<pbkdf2::Algorithm> {
  match digest.to_lowercase().as_str() {
    "sha1" | "sha-1" => Ok(pbkdf2::PBKDF2_HMAC_SHA1),
    "sha256" | "sha-256" => Ok(pbkdf2::PBKDF2_HMAC_SHA256),
    "sha384" | "sha-384" => Ok(pbkdf2::PBKDF2_HMAC_SHA384),
    "sha512" | "sha-512" => Ok(pbkdf2::PBKDF2_HMAC_SHA512),
    _ => Err(Error::new(
      Status::InvalidArg,
      format!("Unsupported digest algorithm for PBKDF2: {digest}"),
    )),
  }
}

fn to_bytes(input: Either<String, Uint8Array>) -> Vec<u8> {
  match input {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.to_vec(),
  }
}

pub struct Pbkdf2Task {
  password: Vec<u8>,
  salt: Vec<u8>,
  iterations: u32,
  keylen: u32,
  digest: String,
}

#[napi]
impl Task for Pbkdf2Task {
  type Output = Vec<u8>;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let algo = get_algorithm(&self.digest)?;
    let iterations = NonZeroU32::new(self.iterations).ok_or_else(|| {
      Error::new(Status::InvalidArg, "Iterations must be greater than 0")
    })?;

    let mut out = vec![0u8; self.keylen as usize];
    pbkdf2::derive(algo, iterations, &self.salt, &self.password, &mut out);
    Ok(out)
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output.into())
  }
}

#[napi]
pub fn pbkdf2(
  password: Either<String, Uint8Array>,
  salt: Either<String, Uint8Array>,
  iterations: u32,
  keylen: u32,
  digest: String,
  abort_signal: Option<AbortSignal>,
) -> AsyncTask<Pbkdf2Task> {
  AsyncTask::with_optional_signal(
    Pbkdf2Task {
      password: to_bytes(password),
      salt: to_bytes(salt),
      iterations,
      keylen,
      digest,
    },
    abort_signal,
  )
}

#[napi]
pub fn pbkdf2_sync(
  env: Env,
  password: Either<String, Uint8Array>,
  salt: Either<String, Uint8Array>,
  iterations: u32,
  keylen: u32,
  digest: String,
) -> Result<Buffer> {
  let mut task = Pbkdf2Task {
    password: to_bytes(password),
    salt: to_bytes(salt),
    iterations,
    keylen,
    digest,
  };
  let output = task.compute()?;
  task.resolve(env, output)
}
