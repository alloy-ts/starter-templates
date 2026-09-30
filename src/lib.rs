use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::str::FromStr;

/// BSON ObjectId wrapper
#[napi]
pub struct ObjectId {
  inner: bson::oid::ObjectId,
}

#[napi]
impl ObjectId {
  #[napi(constructor)]
  pub fn new(hex_str: Option<String>) -> Result<Self> {
    if let Some(hex) = hex_str {
      let inner = bson::oid::ObjectId::from_str(&hex)
        .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid ObjectId hex: {}", e)))?;
      Ok(Self { inner })
    } else {
      Ok(Self {
        inner: bson::oid::ObjectId::new(),
      })
    }
  }

  #[napi]
  pub fn to_hex(&self) -> String {
    self.inner.to_hex()
  }

  #[napi]
  pub fn to_string(&self) -> String {
    self.inner.to_hex()
  }

  #[napi]
  pub fn get_timestamp(&self) -> f64 {
    self.inner.timestamp().timestamp_millis() as f64
  }
}

/// BSON DateTime wrapper
#[napi]
pub struct BsonDateTime {
  inner: bson::DateTime,
}

#[napi]
impl BsonDateTime {
  #[napi(constructor)]
  pub fn new(millis: Option<f64>) -> Result<Self> {
    let inner = if let Some(ms) = millis {
      bson::DateTime::from_millis(ms as i64)
    } else {
      bson::DateTime::now()
    };
    Ok(Self { inner })
  }

  #[napi(factory)]
  pub fn from_date_string(iso: String) -> Result<Self> {
    let dt = chrono::DateTime::parse_from_rfc3339(&iso)
      .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid date string: {}", e)))?;
    let inner = bson::DateTime::from_millis(dt.timestamp_millis());
    Ok(Self { inner })
  }

  #[napi]
  pub fn to_millis(&self) -> f64 {
    self.inner.timestamp_millis() as f64
  }

  #[napi]
  pub fn to_iso_string(&self) -> String {
    self.inner.to_chrono().to_rfc3339()
  }
}

/// BSON Binary data wrapper
#[napi]
pub struct Binary {
  bytes_vec: Vec<u8>,
  #[napi(readonly)]
  pub sub_type: u8,
}

#[napi]
impl Binary {
  #[napi(constructor)]
  pub fn new(bytes: Buffer, sub_type: Option<u8>) -> Self {
    Self {
      bytes_vec: bytes.as_ref().to_vec(),
      sub_type: sub_type.unwrap_or(0),
    }
  }

  #[napi(getter)]
  pub fn bytes(&self) -> Buffer {
    Buffer::from(self.bytes_vec.clone())
  }
}

/// BSON Timestamp wrapper
#[napi]
pub struct Timestamp {
  #[napi(readonly)]
  pub time: u32,
  #[napi(readonly)]
  pub increment: u32,
}

#[napi]
impl Timestamp {
  #[napi(constructor)]
  pub fn new(time: u32, increment: u32) -> Self {
    Self { time, increment }
  }
}

/// BSON Regular Expression wrapper
#[napi]
pub struct Regex {
  #[napi(readonly)]
  pub pattern: String,
  #[napi(readonly)]
  pub options: String,
}

#[napi]
impl Regex {
  #[napi(constructor)]
  pub fn new(pattern: String, options: String) -> Self {
    Self { pattern, options }
  }
}

/// BSON MinKey sentinel
#[napi]
pub struct MinKey {}

#[napi]
impl MinKey {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self {}
  }
}

/// BSON MaxKey sentinel
#[napi]
pub struct MaxKey {}

#[napi]
impl MaxKey {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self {}
  }
}

/// BSON Code wrapper
#[napi]
pub struct Code {
  #[napi(readonly)]
  pub code: String,
}

#[napi]
impl Code {
  #[napi(constructor)]
  pub fn new(code: String) -> Self {
    Self { code }
  }
}

/// Convert JS value/object to raw BSON byte Buffer
#[napi]
pub fn serialize(val: serde_json::Value) -> Result<Buffer> {
  let bson_val = bson::serialize_to_bson(&val)
    .map_err(|e| Error::new(Status::InvalidArg, format!("Failed to convert JS value to BSON: {}", e)))?;

  let doc = match bson_val {
    bson::Bson::Document(d) => d,
    _ => return Err(Error::new(Status::InvalidArg, "Root BSON value must be a document / object")),
  };

  let mut bytes = Vec::new();
  doc.to_writer(&mut bytes)
    .map_err(|e| Error::new(Status::GenericFailure, format!("Failed to serialize BSON document: {}", e)))?;

  Ok(Buffer::from(bytes))
}

/// Convert raw BSON byte Buffer back to JS value/object
#[napi]
pub fn deserialize(bytes: Buffer) -> Result<serde_json::Value> {
  let mut cursor = std::io::Cursor::new(bytes.as_ref());
  let doc = bson::Document::from_reader(&mut cursor)
    .map_err(|e| Error::new(Status::InvalidArg, format!("Failed to parse BSON document: {}", e)))?;

  let bson_val = bson::Bson::Document(doc);
  let json_val = bson::deserialize_from_bson::<serde_json::Value>(bson_val)
    .map_err(|e| Error::new(Status::GenericFailure, format!("Failed to convert BSON to JS value: {}", e)))?;

  Ok(json_val)
}

/// Converts JS object/value into extended JSON represented BSON value
#[napi]
pub fn to_bson(val: serde_json::Value) -> Result<serde_json::Value> {
  let bson_val = bson::serialize_to_bson(&val)
    .map_err(|e| Error::new(Status::InvalidArg, format!("Failed to convert to BSON: {}", e)))?;

  let ext_json = bson_val.into_canonical_extjson();
  Ok(ext_json)
}

/// Converts BSON extended JSON or BSON value into canonical JS value
#[napi]
pub fn from_bson(val: serde_json::Value) -> Result<serde_json::Value> {
  let bson_val: bson::Bson = serde_json::from_value(val)
    .map_err(|e| Error::new(Status::InvalidArg, format!("Failed to parse BSON from JS value: {}", e)))?;

  let json_val = bson::deserialize_from_bson::<serde_json::Value>(bson_val)
    .map_err(|e| Error::new(Status::GenericFailure, format!("Failed to convert BSON to JS value: {}", e)))?;

  Ok(json_val)
}
