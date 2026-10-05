#![deny(clippy::all)]

use argon2_rust::{
  Algorithm as Argon2Algorithm, Argon2, Error as Argon2Error, Params, Version as Argon2Version,
  params::{Memory, TagLen},
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi(object)]
pub struct Argon2Parameters {
  pub message: Either<String, Buffer>,
  pub nonce: Either<String, Buffer>,
  pub parallelism: u32,
  pub tag_length: u32,
  pub memory: u32,
  pub passes: u32,
  pub secret: Option<Either<String, Buffer>>,
  pub associated_data: Option<Either<String, Buffer>>,
}

fn to_bytes(input: Either<String, Buffer>) -> Vec<u8> {
  match input {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.to_vec(),
  }
}

fn parse_algorithm(alg: &str) -> Result<Argon2Algorithm> {
  match alg.to_lowercase().as_str() {
    "argon2d" => Ok(Argon2Algorithm::Argon2d),
    "argon2i" => Ok(Argon2Algorithm::Argon2i),
    "argon2id" => Ok(Argon2Algorithm::Argon2id),
    _ => Err(Error::new(
      Status::InvalidArg,
      format!("Invalid Argon2 algorithm: {}", alg),
    )),
  }
}

fn map_error(err: Argon2Error) -> Error {
  Error::new(Status::GenericFailure, err.to_string())
}

fn compute_argon2(algorithm: &str, params: Argon2Parameters) -> Result<Vec<u8>> {
  let alg = parse_algorithm(algorithm)?;
  let message_bytes = to_bytes(params.message);
  let nonce_bytes = to_bytes(params.nonce);

  if nonce_bytes.len() < 8 {
    return Err(Error::new(
      Status::InvalidArg,
      "nonce must be at least 8 bytes long",
    ));
  }

  let secret_bytes = params.secret.map(to_bytes).unwrap_or_default();
  let ad_bytes = params.associated_data.map(to_bytes).unwrap_or_default();

  let argon_params = Params::builder()
    .memory(Memory::kib(params.memory as u64))
    .passes(params.passes)
    .lanes(params.parallelism)
    .threads(params.parallelism)
    .tag_len(TagLen::bytes(params.tag_length as u64))
    .build()
    .map_err(map_error)?;

  let argon2 = Argon2::new(alg, Argon2Version::V0x13, argon_params);

  let derived = argon2
    .hash_with_ad(&message_bytes, &nonce_bytes, &secret_bytes, &ad_bytes)
    .map_err(map_error)?;

  Ok(derived)
}

#[napi(js_name = "argon2Sync")]
pub fn argon2_sync(
  algorithm: String,
  parameters: Argon2Parameters,
) -> Result<Buffer> {
  let derived = compute_argon2(&algorithm, parameters)?;
  Ok(derived.into())
}

pub struct Argon2Task {
  algorithm: String,
  parameters: Argon2Parameters,
}

#[napi]
impl Task for Argon2Task {
  type Output = Vec<u8>;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    compute_argon2(&self.algorithm, std::mem::replace(&mut self.parameters, Argon2Parameters {
      message: Either::A(String::new()),
      nonce: Either::A(String::new()),
      parallelism: 1,
      tag_length: 4,
      memory: 8,
      passes: 1,
      secret: None,
      associated_data: None,
    }))
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output.into())
  }
}

#[napi(js_name = "argon2")]
pub fn argon2_async(
  algorithm: String,
  parameters: Argon2Parameters,
  signal: Option<AbortSignal>,
) -> AsyncTask<Argon2Task> {
  AsyncTask::with_optional_signal(
    Argon2Task {
      algorithm,
      parameters,
    },
    signal,
  )
}
