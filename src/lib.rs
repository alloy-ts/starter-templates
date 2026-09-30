use napi::bindgen_prelude::*;
use napi_derive::napi;
use serde_json::Value;

#[napi]
pub fn decode(data: Uint8Array) -> Result<Value> {
  let val: Value = rmp_serde::from_slice(&data)
    .map_err(|e| Error::new(Status::InvalidArg, format!("Failed to decode MessagePack: {e}")))?;
  Ok(val)
}

#[napi]
pub fn encode(object: Value) -> Result<Uint8Array> {
  let bytes = rmp_serde::to_vec(&object)
    .map_err(|e| Error::new(Status::InvalidArg, format!("Failed to encode MessagePack: {e}")))?;
  Ok(Uint8Array::from(bytes))
}
