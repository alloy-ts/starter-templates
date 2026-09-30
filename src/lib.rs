use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub fn parse(src: Either<String, Buffer>) -> napi::Result<serde_json::Value> {
  let str_slice = match &src {
    Either::A(s) => s.as_str(),
    Either::B(b) => std::str::from_utf8(b.as_ref())
      .map_err(|e| napi::Error::new(napi::Status::InvalidArg, format!("Invalid UTF-8: {}", e)))?,
  };

  let value: serde_json::Value = toml::from_str(str_slice)
    .map_err(|e| napi::Error::new(napi::Status::GenericFailure, format!("TOML parse error: {}", e)))?;

  Ok(value)
}

#[napi]
pub fn decode(src: Either<String, Buffer>) -> napi::Result<serde_json::Value> {
  parse(src)
}

#[napi]
pub fn stringify(val: serde_json::Value) -> napi::Result<String> {
  toml::to_string(&val)
    .map_err(|e| napi::Error::new(napi::Status::GenericFailure, format!("TOML stringify error: {}", e)))
}

#[napi]
pub fn encode(val: serde_json::Value) -> napi::Result<String> {
  stringify(val)
}
