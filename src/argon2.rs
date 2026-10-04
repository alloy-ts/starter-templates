use argon2::{Algorithm, Argon2, Params, Version};
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi(object)]
pub struct Argon2Options {
  pub memory_cost: Option<u32>,
  pub time_cost: Option<u32>,
  pub parallelism: Option<u32>,
  pub output_length: Option<u32>,
}

#[napi]
pub fn argon2_sync(password: Buffer, salt: Buffer, options: Option<Argon2Options>) -> Result<Buffer> {
  let opts = options.unwrap_or(Argon2Options {
    memory_cost: None,
    time_cost: None,
    parallelism: None,
    output_length: None,
  });

  let m_cost = opts.memory_cost.unwrap_or(65536);
  let t_cost = opts.time_cost.unwrap_or(3);
  let p_cost = opts.parallelism.unwrap_or(4);
  let out_len = opts.output_length.unwrap_or(32) as usize;

  let params = Params::new(m_cost, t_cost, p_cost, Some(out_len))
    .map_err(|e| Error::new(Status::InvalidArg, format!("Argon2 params error: {}", e)))?;

  let ctx = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

  let mut output = vec![0u8; out_len];
  ctx
    .hash_password_into(&password, &salt, &mut output)
    .map_err(|e| Error::new(Status::GenericFailure, format!("Argon2 hashing error: {}", e)))?;

  Ok(Buffer::from(output))
}

#[napi]
pub fn argon2(password: Buffer, salt: Buffer, options: Option<Argon2Options>) -> Result<Buffer> {
  argon2_sync(password, salt, options)
}
