#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
#[derive(Clone)]
pub struct KeyObject {
  key_type: String,
  key_bytes: Vec<u8>,
  asymmetric_type: Option<String>,
}

#[napi]
impl KeyObject {
  #[napi(constructor)]
  pub fn new(
    key_type: String,
    key_bytes: Either<String, &[u8]>,
    asymmetric_type: Option<String>,
  ) -> Result<Self> {
    let bytes = match key_bytes {
      Either::A(s) => s.into_bytes(),
      Either::B(b) => b.to_vec(),
    };

    Ok(KeyObject {
      key_type,
      key_bytes: bytes,
      asymmetric_type,
    })
  }

  #[napi(getter)]
  pub fn get_type(&self) -> String {
    self.key_type.clone()
  }

  #[napi(getter)]
  pub fn get_asymmetric_key_type(&self) -> Option<String> {
    self.asymmetric_type.clone()
  }

  #[napi(getter)]
  pub fn get_symmetric_key_size(&self) -> Option<u32> {
    if self.key_type == "secret" {
      Some(self.key_bytes.len() as u32)
    } else {
      None
    }
  }

  #[napi]
  pub fn export(&self, format: Option<String>) -> Either<String, Buffer> {
    match format.as_deref() {
      Some("utf8") | Some("utf-8") | Some("pem") => {
        let s = String::from_utf8_lossy(&self.key_bytes).to_string();
        Either::A(s)
      }
      Some("hex") => Either::A(hex::encode(&self.key_bytes)),
      _ => Either::B(Buffer::from(self.key_bytes.clone())),
    }
  }
}

#[napi]
pub struct CryptoKeyPair {
  public_key: KeyObject,
  private_key: KeyObject,
}

#[napi]
impl CryptoKeyPair {
  #[napi(constructor)]
  pub fn new(public_key: &KeyObject, private_key: &KeyObject) -> Self {
    CryptoKeyPair {
      public_key: public_key.clone(),
      private_key: private_key.clone(),
    }
  }

  #[napi(getter)]
  pub fn public_key(&self) -> KeyObject {
    self.public_key.clone()
  }

  #[napi(getter)]
  pub fn private_key(&self) -> KeyObject {
    self.private_key.clone()
  }
}

#[napi]
pub fn create_secret_key(
  key: Either<String, &[u8]>,
  encoding: Option<String>,
) -> Result<KeyObject> {
  let bytes = match key {
    Either::A(s) => match encoding.as_deref() {
      Some("hex") => hex::decode(&s).map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?,
      Some("base64") => {
        use base64::Engine;
        base64::engine::general_purpose::STANDARD
          .decode(&s)
          .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?
      }
      _ => s.into_bytes(),
    },
    Either::B(b) => b.to_vec(),
  };

  Ok(KeyObject {
    key_type: "secret".to_string(),
    key_bytes: bytes,
    asymmetric_type: None,
  })
}

#[napi]
pub fn create_private_key(key: Either<String, &[u8]>) -> Result<KeyObject> {
  let bytes = match key {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.to_vec(),
  };

  Ok(KeyObject {
    key_type: "private".to_string(),
    key_bytes: bytes,
    asymmetric_type: Some("rsa".to_string()),
  })
}

#[napi]
pub fn create_public_key(key: Either<String, &[u8]>) -> Result<KeyObject> {
  let bytes = match key {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.to_vec(),
  };

  Ok(KeyObject {
    key_type: "public".to_string(),
    key_bytes: bytes,
    asymmetric_type: Some("rsa".to_string()),
  })
}
