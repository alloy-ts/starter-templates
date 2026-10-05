use argon2::{Algorithm, Argon2, ParamsBuilder, Version};
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi(object)]
pub struct Argon2Parameters {
  pub message: Either<Buffer, String>,
  pub nonce: Either<Buffer, String>,
  pub parallelism: u32,
  pub tag_length: u32,
  pub memory: u32,
  pub passes: u32,
  pub secret: Option<Either<Buffer, String>>,
  pub associated_data: Option<Either<Buffer, String>>,
}

pub fn parse_argon2_algorithm(algo: &str) -> Result<Algorithm> {
  match algo.to_lowercase().as_str() {
    "argon2d" => Ok(Algorithm::Argon2d),
    "argon2i" => Ok(Algorithm::Argon2i),
    "argon2id" => Ok(Algorithm::Argon2id),
    other => Err(Error::new(
      Status::InvalidArg,
      format!("Invalid Argon2 algorithm: {}", other),
    )),
  }
}

pub fn argon2_derive_internal(
  algorithm: &str,
  params: &Argon2Parameters,
) -> Result<Vec<u8>> {
  let algo = parse_argon2_algorithm(algorithm)?;

  let pass_bytes = match &params.message {
    Either::A(b) => b.as_ref(),
    Either::B(s) => s.as_bytes(),
  };
  let nonce_bytes = match &params.nonce {
    Either::A(b) => b.as_ref(),
    Either::B(s) => s.as_bytes(),
  };

  let mut builder = ParamsBuilder::new();
  builder.m_cost(params.memory);
  builder.t_cost(params.passes);
  builder.p_cost(params.parallelism);
  builder.output_len(params.tag_length as usize);

  let argon_params = builder
    .build()
    .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;

  let secret_vec = params.secret.as_ref().map(|s| match s {
    Either::A(b) => b.to_vec(),
    Either::B(str_val) => str_val.as_bytes().to_vec(),
  });

  let argon2 = match &secret_vec {
    Some(sec) => Argon2::new_with_secret(sec.as_slice(), algo, Version::V0x13, argon_params)
      .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?,
    None => Argon2::new(algo, Version::V0x13, argon_params),
  };

  let mut output = vec![0u8; params.tag_length as usize];
  argon2
    .hash_password_into(pass_bytes, nonce_bytes, &mut output)
    .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

  Ok(output)
}

#[napi]
pub fn argon2_sync(
  algorithm: String,
  parameters: Argon2Parameters,
) -> Result<Buffer> {
  let res = argon2_derive_internal(&algorithm, &parameters)?;
  Ok(Buffer::from(res))
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
    argon2_derive_internal(&self.algorithm, &self.parameters)
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(Buffer::from(output))
  }
}

#[napi]
pub fn argon2(
  algorithm: String,
  parameters: Argon2Parameters,
) -> AsyncTask<Argon2Task> {
  AsyncTask::new(Argon2Task {
    algorithm,
    parameters,
  })
}
