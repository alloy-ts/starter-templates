use napi::bindgen_prelude::*;
use napi_derive::napi;
use rand::Rng;
use uuid::Uuid;

#[napi]
pub fn random_bytes_sync(size: u32) -> Result<Buffer> {
  let mut buf = vec![0u8; size as usize];
  rand::rng().fill_bytes(&mut buf);
  Ok(Buffer::from(buf))
}

#[napi]
pub fn random_bytes(size: u32) -> Result<Buffer> {
  random_bytes_sync(size)
}

#[napi]
pub fn random_fill_sync(
  mut buffer: Buffer,
  offset: Option<u32>,
  size: Option<u32>,
) -> Result<Buffer> {
  let start = offset.unwrap_or(0) as usize;
  let len = size.map(|s| s as usize).unwrap_or(buffer.len().saturating_sub(start));

  if start + len > buffer.len() {
    return Err(Error::new(
      Status::InvalidArg,
      "Offset and size out of buffer bounds",
    ));
  }

  rand::rng().fill_bytes(&mut buffer[start..start + len]);
  Ok(buffer)
}

#[napi]
pub fn random_fill(
  buffer: Buffer,
  offset: Option<u32>,
  size: Option<u32>,
) -> Result<Buffer> {
  random_fill_sync(buffer, offset, size)
}

#[napi]
pub fn random_int(min_or_max: i64, max: Option<i64>) -> Result<i64> {
  let (min, max_val) = match max {
    Some(m) => (min_or_max, m),
    None => (0, min_or_max),
  };

  if min >= max_val {
    return Err(Error::new(
      Status::InvalidArg,
      "min must be less than max",
    ));
  }

  let range = (max_val - min) as u64;
  let mut random_u64 = [0u8; 8];
  rand::rng().fill_bytes(&mut random_u64);
  let val = u64::from_le_bytes(random_u64) % range;

  Ok(min + val as i64)
}

#[napi]
pub fn random_uuid() -> String {
  Uuid::new_v4().to_string()
}

#[napi]
pub fn random_uuidv7() -> String {
  Uuid::now_v7().to_string()
}
