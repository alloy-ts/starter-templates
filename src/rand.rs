#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;
use rand::{Rng, RngExt};

#[napi]
pub fn random_bytes(size: u32) -> Result<Buffer> {
  let mut buf = vec![0u8; size as usize];
  rand::rng().fill(&mut buf[..]);
  Ok(Buffer::from(buf))
}

pub struct RandomBytesTask {
  size: usize,
}

#[napi]
impl Task for RandomBytesTask {
  type Output = Vec<u8>;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let mut buf = vec![0u8; self.size];
    rand::rng().fill(&mut buf[..]);
    Ok(buf)
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(Buffer::from(output))
  }
}

#[napi]
pub fn random_bytes_async(size: u32) -> AsyncTask<RandomBytesTask> {
  AsyncTask::new(RandomBytesTask {
    size: size as usize,
  })
}

#[napi]
pub fn random_fill_sync(
  mut buffer: Uint8Array,
  offset: Option<u32>,
  size: Option<u32>,
) -> Result<Uint8Array> {
  let buf_ref = unsafe { buffer.as_mut() };
  let start = offset.unwrap_or(0) as usize;
  let len = size.unwrap_or((buf_ref.len().saturating_sub(start)) as u32) as usize;

  if start + len > buf_ref.len() {
    return Err(Error::new(Status::InvalidArg, "Offset/size out of bounds"));
  }

  rand::rng().fill(&mut buf_ref[start..start + len]);

  Ok(buffer)
}

#[napi]
pub fn random_int(min_or_max: i64, max: Option<i64>) -> Result<i64> {
  let (min_val, max_val) = match max {
    Some(m) => (min_or_max, m),
    None => (0, min_or_max),
  };

  if min_val >= max_val {
    return Err(Error::new(
      Status::InvalidArg,
      "min must be less than max",
    ));
  }

  let mut rng = rand::rng();
  Ok(rng.random_range(min_val..max_val))
}

#[napi]
pub fn random_uuid() -> String {
  uuid::Uuid::new_v4().to_string()
}

#[napi]
pub fn random_uuidv7() -> String {
  uuid::Uuid::now_v7().to_string()
}

#[napi]
pub fn get_random_values(mut typed_array: Uint8Array) -> Result<Uint8Array> {
  let buf_ref = unsafe { typed_array.as_mut() };
  rand::rng().fill(buf_ref);
  Ok(typed_array)
}
