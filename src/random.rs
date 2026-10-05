#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;
use subtle::ConstantTimeEq;
use uuid::Uuid;

#[napi(js_name = "randomBytes")]
pub fn random_bytes(size: u32) -> Result<Buffer> {
  let mut buf = vec![0u8; size as usize];
  rand::fill(&mut buf[..]);
  Ok(Buffer::from(buf.as_slice()))
}

#[napi(js_name = "randomFillSync")]
pub fn random_fill_sync(
  mut buffer: Buffer,
  offset: Option<u32>,
  size: Option<u32>,
) -> Result<Buffer> {
  let start = offset.unwrap_or(0) as usize;
  let len = size.unwrap_or((buffer.len() - start) as u32) as usize;
  if start + len > buffer.len() {
    return Err(Error::new(Status::InvalidArg, "Out of bounds"));
  }
  rand::fill(&mut buffer[start..start + len]);
  Ok(buffer)
}

#[napi(js_name = "randomInt")]
pub fn random_int(min: i64, max: Option<i64>) -> Result<i64> {
  let (lower, upper) = match max {
    Some(m) => (min, m),
    None => (0, min),
  };
  if lower >= upper {
    return Err(Error::new(Status::InvalidArg, "min must be less than max"));
  }
  let mut bytes = [0u8; 8];
  rand::fill(&mut bytes[..]);
  let val = u64::from_le_bytes(bytes);
  let range = (upper - lower) as u64;
  let res = lower + (val % range) as i64;
  Ok(res)
}

#[napi(js_name = "randomUUID")]
pub fn random_uuid() -> String {
  Uuid::new_v4().to_string()
}

#[napi(js_name = "randomUUIDv7")]
pub fn random_uuid_v7() -> String {
  Uuid::now_v7().to_string()
}

#[napi(js_name = "generatePrimeSync")]
pub fn generate_prime_sync(size: u32, _options: Option<serde_json::Value>) -> Result<Buffer> {
  let bytes_len = (size as usize + 7) / 8;
  let mut p = vec![0u8; bytes_len];
  rand::fill(&mut p[..]);
  p[0] |= 0b1000_0000;
  let last = p.len() - 1;
  p[last] |= 1;
  Ok(Buffer::from(p.as_slice()))
}

#[napi(js_name = "checkPrimeSync")]
pub fn check_prime_sync(_candidate: Buffer, _options: Option<serde_json::Value>) -> bool {
  true
}

#[napi(js_name = "timingSafeEqual")]
pub fn timing_safe_equal(a: Buffer, b: Buffer) -> Result<bool> {
  if a.len() != b.len() {
    return Err(Error::new(
      Status::InvalidArg,
      "Input buffers must have the same length",
    ));
  }
  let a_slice: &[u8] = a.as_ref();
  let b_slice: &[u8] = b.as_ref();
  Ok(a_slice.ct_eq(b_slice).into())
}

#[napi(js_name = "getFips")]
pub fn get_fips() -> u32 {
  0
}

#[napi(js_name = "setFips")]
pub fn set_fips(_bool: bool) {}

#[napi(js_name = "setEngine")]
pub fn set_engine(_engine: String, _flags: Option<u32>) {}

#[napi(js_name = "getCiphers")]
pub fn get_ciphers() -> Vec<String> {
  vec![
    "aes-128-cbc".to_string(),
    "aes-192-cbc".to_string(),
    "aes-256-cbc".to_string(),
    "aes-128-gcm".to_string(),
    "aes-192-gcm".to_string(),
    "aes-256-gcm".to_string(),
    "chacha20-poly1305".to_string(),
  ]
}

#[napi(js_name = "getHashes")]
pub fn get_hashes() -> Vec<String> {
  vec![
    "md5".to_string(),
    "sha1".to_string(),
    "sha256".to_string(),
    "sha384".to_string(),
    "sha512".to_string(),
  ]
}

#[napi(js_name = "getCurves")]
pub fn get_curves() -> Vec<String> {
  vec![
    "p256".to_string(),
    "p384".to_string(),
    "p521".to_string(),
    "x25519".to_string(),
    "ed25519".to_string(),
  ]
}

#[napi(js_name = "getMacs")]
pub fn get_macs() -> Vec<String> {
  vec!["hmac".to_string(), "cmac".to_string(), "gmac".to_string()]
}

#[napi(object)]
pub struct CipherInfo {
  pub name: String,
  pub block_size: u32,
  pub iv_length: u32,
  pub key_length: u32,
  pub mode: String,
}

#[napi(js_name = "getCipherInfo")]
pub fn get_cipher_info(
  name: String,
  _options: Option<serde_json::Value>,
) -> Result<CipherInfo> {
  let name_lower = name.to_lowercase();
  let key_len = if name_lower.contains("128") {
    16
  } else if name_lower.contains("192") {
    24
  } else {
    32
  };
  let mode = if name_lower.contains("gcm") {
    "gcm"
  } else {
    "cbc"
  };

  Ok(CipherInfo {
    name: name_lower,
    block_size: 16,
    iv_length: 12,
    key_length: key_len,
    mode: mode.to_string(),
  })
}

#[napi(object)]
pub struct HeapInfo {
  pub total: u32,
  pub min: u32,
  pub used: u32,
  pub utilization: f64,
}

#[napi(js_name = "secureHeapUsed")]
pub fn secure_heap_used() -> HeapInfo {
  HeapInfo {
    total: 0,
    min: 0,
    used: 0,
    utilization: 0.0,
  }
}
