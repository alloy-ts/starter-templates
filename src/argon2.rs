use argon2_rust::params::{Memory, TagLen};
use argon2_rust::{
  Algorithm as Argon2Algorithm, Argon2, Error as Argon2Error, Params,
  Version as Argon2Version,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

fn parse_algorithm(s: &str) -> Result<Argon2Algorithm> {
  match s.to_lowercase().as_str() {
    "argon2d" => Ok(Argon2Algorithm::Argon2d),
    "argon2i" => Ok(Argon2Algorithm::Argon2i),
    "argon2id" => Ok(Argon2Algorithm::Argon2id),
    _ => Err(Error::new(
      Status::InvalidArg,
      format!("Unsupported Argon2 algorithm: {s}"),
    )),
  }
}

#[napi(object)]
pub struct Argon2Parameters {
  pub message: Either<String, Uint8Array>,
  pub nonce: Either<String, Uint8Array>,
  pub parallelism: u32,
  pub tag_length: u32,
  pub memory: u32,
  pub passes: u32,
  pub secret: Option<Either<String, Uint8Array>>,
  pub associated_data: Option<Either<String, Uint8Array>>,
}

fn input_bytes(val: Either<String, Uint8Array>) -> Vec<u8> {
  match val {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.as_ref().to_vec(),
  }
}

fn opt_input_bytes(val: Option<Either<String, Uint8Array>>) -> Vec<u8> {
  match val {
    Some(v) => input_bytes(v),
    None => Vec::new(),
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
    Argon2Error::MemoryAllocationError
    | Argon2Error::ThreadFail
    | Argon2Error::OsRandom
    | Argon2Error::VerifyMismatch => Status::GenericFailure,
    _ => Status::InvalidArg,
  };
  Error::new(status, err.to_string())
}

pub struct NodeArgon2Task {
  algorithm: String,
  params: Argon2Parameters,
}

#[napi]
impl Task for NodeArgon2Task {
  type Output = Vec<u8>;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let algo = parse_algorithm(&self.algorithm)?;
    let msg = input_bytes(std::mem::replace(
      &mut self.params.message,
      Either::A(String::new()),
    ));
    let nonce = input_bytes(std::mem::replace(
      &mut self.params.nonce,
      Either::A(String::new()),
    ));
    let secret = opt_input_bytes(self.params.secret.take());
    let ad = opt_input_bytes(self.params.associated_data.take());

    let params = Params::builder()
      .memory(Memory::kib(self.params.memory as u64))
      .passes(self.params.passes)
      .lanes(self.params.parallelism)
      .threads(thread_budget(self.params.parallelism))
      .tag_len(TagLen::bytes(self.params.tag_length as u64))
      .build()
      .map_err(map_error)?;

    let argon2 = Argon2::new(algo, Argon2Version::default(), params);
    let derived = argon2
      .hash_with_ad(&msg, &nonce, &secret, &ad)
      .map_err(map_error)?;
    Ok(derived)
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output.into())
  }
}

#[napi(js_name = "argon2")]
pub fn crypto_argon2(
  algorithm: String,
  parameters: Argon2Parameters,
) -> AsyncTask<NodeArgon2Task> {
  AsyncTask::new(NodeArgon2Task {
    algorithm,
    params: parameters,
  })
}

#[napi(js_name = "argon2Sync")]
pub fn crypto_argon2_sync(
  env: Env,
  algorithm: String,
  parameters: Argon2Parameters,
) -> Result<Buffer> {
  let mut task = NodeArgon2Task {
    algorithm,
    params: parameters,
  };
  let output = task.compute()?;
  task.resolve(env, output)
}
