use napi::bindgen_prelude::*;
use napi_derive::napi;
use ::scrypt::Params;

#[napi(object)]
pub struct ScryptOptions {
  pub cost: Option<u32>, // N
  pub block_size: Option<u32>, // r
  pub parallelization: Option<u32>, // p
  pub maxmem: Option<u32>,
}

pub fn scrypt_derive_internal(
  password: &[u8],
  salt: &[u8],
  keylen: usize,
  options: Option<ScryptOptions>,
) -> Result<Vec<u8>> {
  let opts = options.unwrap_or(ScryptOptions {
    cost: None,
    block_size: None,
    parallelization: None,
    maxmem: None,
  });

  let n = opts.cost.unwrap_or(16384);
  let r = opts.block_size.unwrap_or(8);
  let p = opts.parallelization.unwrap_or(1);

  let params = Params::new(
    n.trailing_zeros() as u8,
    r,
    p,
  ).map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;

  let mut output = vec![0u8; keylen];
  ::scrypt::scrypt(password, salt, &params, &mut output)
    .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

  Ok(output)
}

#[napi]
pub fn scrypt_sync(
  password: Either<Buffer, String>,
  salt: Either<Buffer, String>,
  keylen: u32,
  options: Option<ScryptOptions>,
) -> Result<Buffer> {
  let pass_bytes = match &password {
    Either::A(b) => b.as_ref(),
    Either::B(s) => s.as_bytes(),
  };
  let salt_bytes = match &salt {
    Either::A(b) => b.as_ref(),
    Either::B(s) => s.as_bytes(),
  };

  let res = scrypt_derive_internal(pass_bytes, salt_bytes, keylen as usize, options)?;
  Ok(Buffer::from(res))
}

pub struct ScryptTask {
  password: Vec<u8>,
  salt: Vec<u8>,
  keylen: usize,
  options: Option<ScryptOptions>,
}

#[napi]
impl Task for ScryptTask {
  type Output = Vec<u8>;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    scrypt_derive_internal(&self.password, &self.salt, self.keylen, self.options.take())
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(Buffer::from(output))
  }
}

#[napi]
pub fn scrypt(
  password: Either<Buffer, String>,
  salt: Either<Buffer, String>,
  keylen: u32,
  options: Option<ScryptOptions>,
) -> AsyncTask<ScryptTask> {
  let pass_bytes = match password {
    Either::A(b) => b.to_vec(),
    Either::B(s) => s.into_bytes(),
  };
  let salt_bytes = match salt {
    Either::A(b) => b.to_vec(),
    Either::B(s) => s.into_bytes(),
  };

  AsyncTask::new(ScryptTask {
    password: pass_bytes,
    salt: salt_bytes,
    keylen: keylen as usize,
    options,
  })
}
