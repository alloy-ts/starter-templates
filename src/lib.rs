use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::str::FromStr;
use svix_ksuid::KsuidLike;

#[napi]
pub struct Ksuid {
  inner: svix_ksuid::Ksuid,
}

#[napi]
impl Ksuid {
  #[napi(constructor)]
  pub fn new(input: Option<Either<String, Uint8Array>>) -> Result<Self> {
    match input {
      None => Ok(Ksuid {
        inner: svix_ksuid::Ksuid::now(None),
      }),
      Some(Either::A(s)) => {
        let inner = svix_ksuid::Ksuid::from_str(&s)
          .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid KSUID string: {e}")))?;
        Ok(Ksuid { inner })
      }
      Some(Either::B(b)) => {
        if b.len() != 20 {
          return Err(Error::new(
            Status::InvalidArg,
            format!("Invalid byte length for KSUID: expected 20, got {}", b.len()),
          ));
        }
        let mut arr = [0u8; 20];
        arr.copy_from_slice(&b);
        Ok(Ksuid {
          inner: svix_ksuid::Ksuid::from_bytes(arr),
        })
      }
    }
  }

  #[napi(factory)]
  pub fn now(payload: Option<Uint8Array>) -> Result<Self> {
    let payload_ref = payload.as_ref().map(|p| p.as_ref());
    if let Some(p) = payload_ref {
      if p.len() != svix_ksuid::Ksuid::PAYLOAD_BYTES {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Invalid payload length for KSUID: expected {}, got {}", svix_ksuid::Ksuid::PAYLOAD_BYTES, p.len()),
        ));
      }
    }
    let inner = svix_ksuid::Ksuid::now(payload_ref);
    Ok(Ksuid { inner })
  }

  #[napi(factory)]
  pub fn from_seconds(timestamp_seconds: Option<i64>, payload: Option<Uint8Array>) -> Result<Self> {
    let payload_ref = payload.as_ref().map(|p| p.as_ref());
    if let Some(p) = payload_ref {
      if p.len() != svix_ksuid::Ksuid::PAYLOAD_BYTES {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Invalid payload length for KSUID: expected {}, got {}", svix_ksuid::Ksuid::PAYLOAD_BYTES, p.len()),
        ));
      }
    }
    let inner = svix_ksuid::Ksuid::from_seconds(timestamp_seconds, payload_ref);
    Ok(Ksuid { inner })
  }

  #[napi(factory)]
  pub fn from_base62(base62: String) -> Result<Self> {
    let inner = svix_ksuid::Ksuid::from_base62(&base62)
      .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid base62 KSUID: {e}")))?;
    Ok(Ksuid { inner })
  }

  #[napi(factory)]
  pub fn from_str(base62: String) -> Result<Self> {
    Self::from_base62(base62)
  }

  #[napi(factory)]
  pub fn from_bytes(bytes: Uint8Array) -> Result<Self> {
    if bytes.len() != 20 {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Invalid byte length for KSUID: expected 20, got {}", bytes.len()),
      ));
    }
    let mut arr = [0u8; 20];
    arr.copy_from_slice(&bytes);
    Ok(Ksuid {
      inner: svix_ksuid::Ksuid::from_bytes(arr),
    })
  }

  #[napi]
  pub fn to_base62(&self) -> String {
    self.inner.to_base62()
  }

  #[napi]
  pub fn to_string(&self) -> String {
    self.inner.to_string()
  }

  #[napi]
  pub fn bytes(&self) -> Uint8Array {
    Uint8Array::from(self.inner.bytes().as_slice())
  }

  #[napi]
  pub fn payload(&self) -> Uint8Array {
    Uint8Array::from(self.inner.payload())
  }

  #[napi]
  pub fn timestamp_seconds(&self) -> i64 {
    self.inner.timestamp_seconds()
  }

  #[napi]
  pub fn compare(&self, other: &Ksuid) -> i32 {
    match self.inner.cmp(&other.inner) {
      std::cmp::Ordering::Less => -1,
      std::cmp::Ordering::Equal => 0,
      std::cmp::Ordering::Greater => 1,
    }
  }

  #[napi]
  pub fn equals(&self, other: &Ksuid) -> bool {
    self.inner == other.inner
  }
}

#[napi]
pub struct KsuidMs {
  inner: svix_ksuid::KsuidMs,
}

#[napi]
impl KsuidMs {
  #[napi(constructor)]
  pub fn new(input: Option<Either<String, Uint8Array>>) -> Result<Self> {
    match input {
      None => Ok(KsuidMs {
        inner: svix_ksuid::KsuidMs::now(None),
      }),
      Some(Either::A(s)) => {
        let inner = svix_ksuid::KsuidMs::from_str(&s)
          .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid KSUIDms string: {e}")))?;
        Ok(KsuidMs { inner })
      }
      Some(Either::B(b)) => {
        if b.len() != 20 {
          return Err(Error::new(
            Status::InvalidArg,
            format!("Invalid byte length for KSUIDms: expected 20, got {}", b.len()),
          ));
        }
        let mut arr = [0u8; 20];
        arr.copy_from_slice(&b);
        Ok(KsuidMs {
          inner: svix_ksuid::KsuidMs::from_bytes(arr),
        })
      }
    }
  }

  #[napi(factory)]
  pub fn now(payload: Option<Uint8Array>) -> Result<Self> {
    let payload_ref = payload.as_ref().map(|p| p.as_ref());
    if let Some(p) = payload_ref {
      if p.len() != svix_ksuid::KsuidMs::PAYLOAD_BYTES {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Invalid payload length for KSUIDms: expected {}, got {}", svix_ksuid::KsuidMs::PAYLOAD_BYTES, p.len()),
        ));
      }
    }
    let inner = svix_ksuid::KsuidMs::now(payload_ref);
    Ok(KsuidMs { inner })
  }

  #[napi(factory)]
  pub fn from_millis(timestamp_millis: Option<i64>, payload: Option<Uint8Array>) -> Result<Self> {
    let payload_ref = payload.as_ref().map(|p| p.as_ref());
    if let Some(p) = payload_ref {
      if p.len() != svix_ksuid::KsuidMs::PAYLOAD_BYTES {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Invalid payload length for KSUIDms: expected {}, got {}", svix_ksuid::KsuidMs::PAYLOAD_BYTES, p.len()),
        ));
      }
    }
    let inner = svix_ksuid::KsuidMs::from_millis(timestamp_millis, payload_ref);
    Ok(KsuidMs { inner })
  }

  #[napi(factory)]
  pub fn from_base62(base62: String) -> Result<Self> {
    let inner = svix_ksuid::KsuidMs::from_base62(&base62)
      .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid base62 KSUIDms: {e}")))?;
    Ok(KsuidMs { inner })
  }

  #[napi(factory)]
  pub fn from_str(base62: String) -> Result<Self> {
    Self::from_base62(base62)
  }

  #[napi(factory)]
  pub fn from_bytes(bytes: Uint8Array) -> Result<Self> {
    if bytes.len() != 20 {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Invalid byte length for KSUIDms: expected 20, got {}", bytes.len()),
      ));
    }
    let mut arr = [0u8; 20];
    arr.copy_from_slice(&bytes);
    Ok(KsuidMs {
      inner: svix_ksuid::KsuidMs::from_bytes(arr),
    })
  }

  #[napi]
  pub fn to_base62(&self) -> String {
    self.inner.to_base62()
  }

  #[napi]
  pub fn to_string(&self) -> String {
    self.inner.to_string()
  }

  #[napi]
  pub fn bytes(&self) -> Uint8Array {
    Uint8Array::from(self.inner.bytes().as_slice())
  }

  #[napi]
  pub fn payload(&self) -> Uint8Array {
    Uint8Array::from(self.inner.payload())
  }

  #[napi]
  pub fn timestamp_millis(&self) -> i64 {
    self.inner.timestamp_millis()
  }

  #[napi]
  pub fn compare(&self, other: &KsuidMs) -> i32 {
    match self.inner.cmp(&other.inner) {
      std::cmp::Ordering::Less => -1,
      std::cmp::Ordering::Equal => 0,
      std::cmp::Ordering::Greater => 1,
    }
  }

  #[napi]
  pub fn equals(&self, other: &KsuidMs) -> bool {
    self.inner == other.inner
  }
}
