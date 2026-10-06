use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::pbkdf2;
use std::num::NonZeroU32;

fn parse_digest_algorithm(digest: &str) -> Result<pbkdf2::Algorithm> {
  match digest.to_lowercase().replace('-', "").as_str() {
    "sha1" => Ok(pbkdf2::PBKDF2_HMAC_SHA1),
    "sha256" => Ok(pbkdf2::PBKDF2_HMAC_SHA256),
    "sha384" => Ok(pbkdf2::PBKDF2_HMAC_SHA384),
    "sha512" => Ok(pbkdf2::PBKDF2_HMAC_SHA512),
    _ => Err(Error::new(
      Status::InvalidArg,
      format!("Unsupported PBKDF2 digest algorithm: {digest}"),
    )),
  }
}

fn input_to_vec(data: Either<String, Uint8Array>) -> Vec<u8> {
  match data {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.as_ref().to_vec(),
  }
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
    let alg = parse_digest_algorithm(&self.digest)?;
    let iterations = NonZeroU32::new(self.iterations).ok_or_else(|| {
      Error::new(Status::InvalidArg, "Iterations must be greater than 0")
    })?;
    let mut out = vec![0u8; self.keylen];
    pbkdf2::derive(alg, iterations, &self.salt, &self.password, &mut out);
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
) -> AsyncTask<Pbkdf2Task> {
  AsyncTask::new(Pbkdf2Task {
    password: input_to_vec(password),
    salt: input_to_vec(salt),
    iterations,
    keylen: keylen as usize,
    digest,
  })
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
    password: input_to_vec(password),
    salt: input_to_vec(salt),
    iterations,
    keylen: keylen as usize,
    digest,
  };
  let output = task.compute()?;
  task.resolve(env, output)
}
