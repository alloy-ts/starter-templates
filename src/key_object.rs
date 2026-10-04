use napi::bindgen_prelude::*;
use napi_derive::napi;
use x509_parser::prelude::FromDer;

#[napi]
pub enum KeyType {
  Secret,
  Public,
  Private,
}

#[napi]
pub struct KeyObject {
  pub key_type: KeyType,
  pub raw: Vec<u8>,
}

#[napi]
impl KeyObject {
  #[napi(constructor)]
  pub fn new(key_type: KeyType, raw: Buffer) -> Self {
    KeyObject {
      key_type,
      raw: raw.as_ref().to_vec(),
    }
  }

  #[napi]
  pub fn export(&self) -> Buffer {
    Buffer::from(self.raw.clone())
  }
}

#[napi(object)]
pub struct CryptoKeyPair {
  pub public_key: Buffer,
  pub private_key: Buffer,
}

#[napi]
pub struct X509Certificate {
  raw_bytes: Vec<u8>,
  subject_name: String,
  issuer_name: String,
}

#[napi]
impl X509Certificate {
  #[napi(constructor)]
  pub fn new(buffer: Buffer) -> Result<Self> {
    let (_, cert) = x509_parser::certificate::X509Certificate::from_der(buffer.as_ref())
      .map_err(|e| Error::new(Status::InvalidArg, format!("X509 parse error: {}", e)))?;

    let subject = cert.subject().to_string();
    let issuer = cert.issuer().to_string();

    Ok(X509Certificate {
      raw_bytes: buffer.as_ref().to_vec(),
      subject_name: subject,
      issuer_name: issuer,
    })
  }

  #[napi(getter)]
  pub fn subject(&self) -> String {
    self.subject_name.clone()
  }

  #[napi(getter)]
  pub fn issuer(&self) -> String {
    self.issuer_name.clone()
  }

  #[napi(getter)]
  pub fn raw(&self) -> Buffer {
    Buffer::from(self.raw_bytes.clone())
  }
}

#[napi]
pub fn create_secret_key(key: Buffer) -> KeyObject {
  KeyObject::new(KeyType::Secret, key)
}
