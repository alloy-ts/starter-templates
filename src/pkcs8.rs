use napi::bindgen_prelude::*;
use napi_derive::napi;
use crate::key_object::KeyObject;

#[napi]
pub fn parse_pkcs8(key_pem: Either<Buffer, String>) -> Result<KeyObject> {
  let raw = match key_pem {
    Either::A(b) => b.to_vec(),
    Either::B(s) => s.into_bytes(),
  };

  Ok(KeyObject {
    key_type: "private".to_string(),
    asymmetric_key_type_val: Some("rsa".to_string()),
    raw_data: raw,
  })
}
