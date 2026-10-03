#![deny(clippy::all)]

use argon2_rust::{
  Algorithm as Argon2Algorithm, Argon2, Error as Argon2Error, Params, Version as Argon2Version,
  params::{Memory, TagLen},
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
#[derive(Clone, Copy)]
pub enum Algorithm {
  Argon2d,
  Argon2i,
  Argon2id,
}

impl Algorithm {
  #[inline]
  fn to_argon(self) -> Argon2Algorithm {
    match self {
      Self::Argon2d => Argon2Algorithm::Argon2d,
      Self::Argon2i => Argon2Algorithm::Argon2i,
      Self::Argon2id => Argon2Algorithm::Argon2id,
    }
  }

  #[inline]
  fn from_argon(algorithm: Argon2Algorithm) -> Self {
    match algorithm {
      Argon2Algorithm::Argon2d => Self::Argon2d,
      Argon2Algorithm::Argon2i => Self::Argon2i,
      Argon2Algorithm::Argon2id => Self::Argon2id,
    }
  }
}

#[napi]
#[derive(Clone, Copy)]
pub enum Version {
  V0x10,
  V0x13,
}

impl Version {
  #[inline]
  fn to_argon(self) -> Argon2Version {
    match self {
      Self::V0x10 => Argon2Version::V0x10,
      Self::V0x13 => Argon2Version::V0x13,
    }
  }

  #[inline]
  fn from_argon(version: Argon2Version) -> Self {
    match version {
      Argon2Version::V0x10 => Self::V0x10,
      Argon2Version::V0x13 => Self::V0x13,
    }
  }
}

#[napi(object, object_to_js = false)]
#[derive(Default)]
pub struct Options {
  pub memory_cost: Option<u32>,
  pub time_cost: Option<u32>,
  pub output_len: Option<u32>,
  pub parallelism: Option<u32>,
  pub algorithm: Option<Algorithm>,
  pub version: Option<Version>,
  pub secret: Option<Uint8Array>,
  pub salt: Option<Uint8Array>,
}

#[napi(object)]
pub struct ParsedHashOptions {
  pub algorithm: Algorithm,
  pub version: Version,
  pub memory_cost: u32,
  pub time_cost: u32,
  pub parallelism: u32,
  pub output_len: u32,
  pub salt_len: u32,
}

impl Options {
  fn algorithm(&self) -> Argon2Algorithm {
    self
      .algorithm
      .map(|algorithm| algorithm.to_argon())
      .unwrap_or_default()
  }

  fn version(&self) -> Argon2Version {
    self
      .version
      .map(|version| version.to_argon())
      .unwrap_or_default()
  }

  fn secret(&self) -> &[u8] {
    self
      .secret
      .as_ref()
      .map(|secret| secret.as_ref())
      .unwrap_or(&[])
  }

  fn salt(&self) -> Option<&[u8]> {
    self.salt.as_ref().map(|salt| salt.as_ref())
  }

  fn params(&self) -> Result<Params> {
    let mut builder = Params::builder();
    if let Some(memory_cost) = self.memory_cost {
      builder = builder.memory(Memory::kib(memory_cost as u64));
    }
    if let Some(time_cost) = self.time_cost {
      builder = builder.passes(time_cost);
    }
    if let Some(parallelism) = self.parallelism {
      builder = builder
        .lanes(parallelism)
        .threads(thread_budget(parallelism));
    }
    if let Some(output_len) = self.output_len {
      builder = builder.tag_len(TagLen::bytes(output_len as u64));
    }
    builder.build().map_err(map_error)
  }

  fn hasher(&self) -> Result<Argon2> {
    Ok(Argon2::new(
      self.algorithm(),
      self.version(),
      self.params()?,
    ))
  }
}

fn thread_budget(lanes: u32) -> u32 {
  let available = std::thread::available_parallelism()
    .map(|n| n.get() as u32)
    .unwrap_or(1)
    .max(1);
  lanes.min(available)
}

fn map_error(err: Argon2Error) -> Error {
  let status = match err {
    Argon2Error::DecodingFail | Argon2Error::EncodingFail => Status::InvalidArg,
    _ => Status::GenericFailure,
  };
  Error::new(status, err.to_string())
}

fn password_bytes(password: Either<String, &[u8]>) -> Vec<u8> {
  match password {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.to_vec(),
  }
}

fn utf8_input(value: Either<String, &[u8]>) -> Result<String> {
  match value {
    Either::A(s) => Ok(s),
    Either::B(b) => simdutf8::basic::from_utf8(b)
      .map_err(|err| Error::new(Status::InvalidArg, format!("{err}")))
      .map(|s: &str| s.to_string()),
  }
}

fn generate_salt() -> [u8; argon2_rust::RANDOM_SALT_LEN] {
  rand::random()
}

fn hash_encoded(argon2: &Argon2, password: &[u8], salt: &[u8], secret: &[u8]) -> Result<String> {
  argon2
    .hash_encoded_with_ad(password, salt, secret, &[])
    .map_err(map_error)
}

fn hash_raw_bytes(argon2: &Argon2, password: &[u8], salt: &[u8], secret: &[u8]) -> Result<Vec<u8>> {
  argon2
    .hash_with_ad(password, salt, secret, &[])
    .map_err(map_error)
}

fn decode_hashed(encoded: &str) -> Result<argon2_rust::Decoded> {
  let mut decoded = argon2_rust::decode_phc(encoded).map_err(map_error)?;
  decoded.params = decoded
    .params
    .to_builder()
    .threads(thread_budget(decoded.params.lanes()))
    .build()
    .map_err(map_error)?;
  Ok(decoded)
}

pub struct HashTask {
  password: Vec<u8>,
  options: Options,
}

#[napi]
impl Task for HashTask {
  type Output = String;
  type JsValue = String;

  fn compute(&mut self) -> Result<Self::Output> {
    let hasher = self.options.hasher()?;
    let secret = self.options.secret();
    match self.options.salt() {
      Some(salt) => hash_encoded(&hasher, &self.password, salt, secret),
      None => {
        let salt = generate_salt();
        hash_encoded(&hasher, &self.password, &salt, secret)
      }
    }
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi]
pub fn hash(
  password: Either<String, &[u8]>,
  options: Option<Options>,
  abort_signal: Option<AbortSignal>,
) -> AsyncTask<HashTask> {
  AsyncTask::with_optional_signal(
    HashTask {
      password: password_bytes(password),
      options: options.unwrap_or_default(),
    },
    abort_signal,
  )
}

#[napi]
pub fn hash_sync(
  env: Env,
  password: Either<String, &[u8]>,
  options: Option<Options>,
) -> Result<String> {
  let mut hash_task = HashTask {
    password: password_bytes(password),
    options: options.unwrap_or_default(),
  };
  let output = hash_task.compute()?;
  hash_task.resolve(env, output)
}

pub struct RawHashTask {
  password: Vec<u8>,
  options: Options,
}

#[napi]
impl Task for RawHashTask {
  type Output = Vec<u8>;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let hasher = self.options.hasher()?;
    let secret = self.options.secret();
    match self.options.salt() {
      Some(salt) => hash_raw_bytes(&hasher, &self.password, salt, secret),
      None => {
        let salt = generate_salt();
        hash_raw_bytes(&hasher, &self.password, &salt, secret)
      }
    }
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output.into())
  }
}

#[napi]
pub fn hash_raw(
  password: Either<String, &[u8]>,
  options: Option<Options>,
  abort_signal: Option<AbortSignal>,
) -> AsyncTask<RawHashTask> {
  AsyncTask::with_optional_signal(
    RawHashTask {
      password: password_bytes(password),
      options: options.unwrap_or_default(),
    },
    abort_signal,
  )
}

#[napi]
pub fn hash_raw_sync(
  env: Env,
  password: Either<String, &[u8]>,
  options: Option<Options>,
) -> Result<Buffer> {
  let mut hash_task = RawHashTask {
    password: password_bytes(password),
    options: options.unwrap_or_default(),
  };
  let output = hash_task.compute()?;
  hash_task.resolve(env, output)
}

pub struct VerifyTask {
  password: String,
  hashed: String,
  options: Options,
}

#[napi]
impl Task for VerifyTask {
  type Output = bool;
  type JsValue = bool;

  fn compute(&mut self) -> Result<Self::Output> {
    let decoded = decode_hashed(&self.hashed)?;
    let argon2 = Argon2::new(decoded.algorithm, decoded.version, decoded.params);
    match argon2.verify_with_ad(
      self.password.as_bytes(),
      &decoded.salt,
      self.options.secret(),
      &decoded.ad,
      &decoded.hash,
    ) {
      Ok(()) => Ok(true),
      Err(Argon2Error::VerifyMismatch) => Ok(false),
      Err(err) => Err(map_error(err)),
    }
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi]
pub fn verify(
  hashed: Either<String, &[u8]>,
  password: Either<String, &[u8]>,
  options: Option<Options>,
  abort_signal: Option<AbortSignal>,
) -> Result<AsyncTask<VerifyTask>> {
  Ok(AsyncTask::with_optional_signal(
    VerifyTask {
      password: utf8_input(password)?,
      hashed: utf8_input(hashed)?,
      options: options.unwrap_or_default(),
    },
    abort_signal,
  ))
}

#[napi]
pub fn verify_sync(
  env: Env,
  hashed: Either<String, &[u8]>,
  password: Either<String, &[u8]>,
  options: Option<Options>,
) -> Result<bool> {
  let mut verify_task = VerifyTask {
    password: utf8_input(password)?,
    hashed: utf8_input(hashed)?,
    options: options.unwrap_or_default(),
  };
  let output = verify_task.compute()?;
  verify_task.resolve(env, output)
}

#[napi]
pub fn parse_options(hashed: Either<String, &[u8]>) -> Result<ParsedHashOptions> {
  let encoded = utf8_input(hashed)?;
  let decoded = argon2_rust::decode_phc(&encoded).map_err(map_error)?;
  Ok(ParsedHashOptions {
    algorithm: Algorithm::from_argon(decoded.algorithm),
    version: Version::from_argon(decoded.version),
    memory_cost: decoded.params.memory_kib(),
    time_cost: decoded.params.passes(),
    parallelism: decoded.params.lanes(),
    output_len: decoded.params.tag_len_bytes() as u32,
    salt_len: decoded.salt.len() as u32,
  })
}

#[napi]
pub fn argon2(
  password: Either<String, &[u8]>,
  salt: Either<String, &[u8]>,
  options: Option<Options>,
) -> AsyncTask<RawHashTask> {
  let mut opts = options.unwrap_or_default();
  opts.salt = Some(Uint8Array::new(password_bytes(salt)));
  AsyncTask::new(RawHashTask {
    password: password_bytes(password),
    options: opts,
  })
}

#[napi]
pub fn argon2_sync(
  env: Env,
  password: Either<String, &[u8]>,
  salt: Either<String, &[u8]>,
  options: Option<Options>,
) -> Result<Buffer> {
  let mut opts = options.unwrap_or_default();
  opts.salt = Some(Uint8Array::new(password_bytes(salt)));
  let mut hash_task = RawHashTask {
    password: password_bytes(password),
    options: opts,
  };
  let output = hash_task.compute()?;
  hash_task.resolve(env, output)
}
