use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::rand::{SecureRandom, SystemRandom};
use uuid::Uuid;

#[napi]
pub fn random_bytes(size: u32) -> Result<Buffer> {
  let rng = SystemRandom::new();
  let mut bytes = vec![0u8; size as usize];
  rng
    .fill(&mut bytes)
    .map_err(|_| Error::new(Status::GenericFailure, "Random fill failed"))?;
  Ok(Buffer::from(bytes))
}

#[napi]
pub fn random_fill_sync(mut buffer: Buffer, offset: Option<u32>, size: Option<u32>) -> Result<Buffer> {
  let start = offset.unwrap_or(0) as usize;
  let len = size.unwrap_or((buffer.len() as u32) - (start as u32)) as usize;

  let rng = SystemRandom::new();
  let buf_slice = &mut buffer[start..start + len];
  rng
    .fill(buf_slice)
    .map_err(|_| Error::new(Status::GenericFailure, "Random fill failed"))?;

  Ok(buffer)
}

#[napi]
pub fn random_int(min: i32, max: i32) -> Result<i32> {
  if min >= max {
    return Err(Error::new(Status::InvalidArg, "min must be less than max"));
  }
  let range = (max - min) as u32;
  let rng = SystemRandom::new();
  let mut bytes = [0u8; 4];
  rng
    .fill(&mut bytes)
    .map_err(|_| Error::new(Status::GenericFailure, "Random fill failed"))?;
  let val = u32::from_ne_bytes(bytes);
  Ok(min + (val % range) as i32)
}

#[napi]
pub fn random_uuid() -> String {
  Uuid::new_v4().to_string()
}

#[napi]
pub fn random_uuid_v7() -> String {
  Uuid::now_v7().to_string()
}
