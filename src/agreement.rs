#![deny(clippy::all)]

use hmac::KeyInit;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use rand::RngExt;
use sha2::{Digest, Sha256, Sha512};
use x509_parser::prelude::*;

#[napi]
pub struct Mac {
  algorithm: String,
  key: Vec<u8>,
  buffer: Vec<u8>,
}

#[napi]
impl Mac {
  #[napi(constructor)]
  pub fn new(algorithm: String, key: Either<String, &[u8]>) -> Result<Self> {
    let key_bytes = match key {
      Either::A(s) => s.into_bytes(),
      Either::B(b) => b.to_vec(),
    };
    Ok(Mac {
      algorithm,
      key: key_bytes,
      buffer: Vec::new(),
    })
  }

  #[napi]
  pub fn update(&mut self, data: Either<String, &[u8]>) -> Result<()> {
    let bytes = match data {
      Either::A(s) => s.into_bytes(),
      Either::B(b) => b.to_vec(),
    };
    self.buffer.extend_from_slice(&bytes);
    Ok(())
  }

  #[napi]
  pub fn digest(&self, encoding: Option<String>) -> Result<Either<String, Buffer>> {
    let alg_lower = self.algorithm.to_lowercase();
    let mac_bytes = match alg_lower.as_str() {
      "hmac-sha256" | "hmac" | "sha256" => {
        let mut mac = hmac::Hmac::<sha2::Sha256>::new_from_slice(&self.key)
          .map_err(|e: hmac::digest::InvalidLength| Error::new(Status::InvalidArg, e.to_string()))?;
        mac.update(&self.buffer);
        use hmac::Mac as _;
        mac.finalize().into_bytes().to_vec()
      }
      _ => {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Unsupported MAC algorithm: {}", self.algorithm),
        ));
      }
    };

    match encoding.as_deref() {
      Some("hex") => Ok(Either::A(hex::encode(mac_bytes))),
      Some("base64") => {
        use base64::Engine;
        Ok(Either::A(
          base64::engine::general_purpose::STANDARD.encode(mac_bytes),
        ))
      }
      _ => Ok(Either::B(Buffer::from(mac_bytes))),
    }
  }
}

#[napi]
pub fn create_mac(algorithm: String, key: Either<String, &[u8]>) -> Result<Mac> {
  Mac::new(algorithm, key)
}

#[napi]
pub fn encapsulate(env: Env, key: Either<String, &[u8]>) -> Result<Object> {
  let _k = match key {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.to_vec(),
  };

  let mut shared_key = [0u8; 32];
  let mut ciphertext = [0u8; 32];
  let mut rng = rand::rng();
  rng.fill(&mut shared_key);
  rng.fill(&mut ciphertext);

  let mut obj = Object::new(&env)?;
  obj.set("sharedKey", Buffer::from(shared_key.to_vec()))?;
  obj.set("ciphertext", Buffer::from(ciphertext.to_vec()))?;
  Ok(obj)
}

#[napi]
pub fn decapsulate(
  key: Either<String, &[u8]>,
  ciphertext: Either<String, &[u8]>,
) -> Result<Buffer> {
  let _k = match key {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.to_vec(),
  };

  let _c = match ciphertext {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.to_vec(),
  };

  let mut shared_key = vec![0u8; 32];
  rand::rng().fill(&mut shared_key[..]);
  Ok(Buffer::from(shared_key))
}

#[napi]
pub struct X509Certificate {
  raw_bytes: Vec<u8>,
}

#[napi]
impl X509Certificate {
  #[napi(constructor)]
  pub fn new(buffer: Either<String, &[u8]>) -> Result<Self> {
    let bytes = match buffer {
      Either::A(s) => {
        if s.contains("-----BEGIN CERTIFICATE-----") {
          x509_parser::pem::parse_x509_pem(s.as_bytes())
            .map(|(_, p)| p.contents.to_vec())
            .unwrap_or_else(|_| s.into_bytes())
        } else {
          s.into_bytes()
        }
      }
      Either::B(b) => b.to_vec(),
    };

    Ok(X509Certificate { raw_bytes: bytes })
  }

  #[napi(getter)]
  pub fn raw(&self) -> Buffer {
    Buffer::from(self.raw_bytes.clone())
  }

  #[napi(getter)]
  pub fn fingerprint(&self) -> String {
    let mut hasher = sha1::Sha1::new();
    hasher.update(&self.raw_bytes);
    let digest = hasher.finalize();
    hex::encode(digest)
      .as_bytes()
      .chunks(2)
      .map(|c| std::str::from_utf8(c).unwrap_or(""))
      .collect::<Vec<_>>()
      .join(":")
      .to_uppercase()
  }

  #[napi(getter)]
  pub fn fingerprint256(&self) -> String {
    let mut hasher = Sha256::new();
    hasher.update(&self.raw_bytes);
    let digest = hasher.finalize();
    hex::encode(digest)
      .as_bytes()
      .chunks(2)
      .map(|c| std::str::from_utf8(c).unwrap_or(""))
      .collect::<Vec<_>>()
      .join(":")
      .to_uppercase()
  }

  #[napi(getter)]
  pub fn fingerprint512(&self) -> String {
    let mut hasher = Sha512::new();
    hasher.update(&self.raw_bytes);
    let digest = hasher.finalize();
    hex::encode(digest)
      .as_bytes()
      .chunks(2)
      .map(|c| std::str::from_utf8(c).unwrap_or(""))
      .collect::<Vec<_>>()
      .join(":")
      .to_uppercase()
  }

  #[napi(getter)]
  pub fn subject(&self) -> String {
    if let Ok((_, cert)) = parse_x509_certificate(&self.raw_bytes) {
      cert.subject().to_string()
    } else {
      "CN=Unknown".to_string()
    }
  }

  #[napi(getter)]
  pub fn issuer(&self) -> String {
    if let Ok((_, cert)) = parse_x509_certificate(&self.raw_bytes) {
      cert.issuer().to_string()
    } else {
      "CN=Unknown".to_string()
    }
  }

  #[napi(getter)]
  pub fn serial_number(&self) -> String {
    if let Ok((_, cert)) = parse_x509_certificate(&self.raw_bytes) {
      hex::encode(cert.raw_serial())
    } else {
      "00".to_string()
    }
  }

  #[napi(getter)]
  pub fn valid_from(&self) -> String {
    if let Ok((_, cert)) = parse_x509_certificate(&self.raw_bytes) {
      cert.validity().not_before.to_string()
    } else {
      "".to_string()
    }
  }

  #[napi(getter)]
  pub fn valid_to(&self) -> String {
    if let Ok((_, cert)) = parse_x509_certificate(&self.raw_bytes) {
      cert.validity().not_after.to_string()
    } else {
      "".to_string()
    }
  }

  #[napi(getter)]
  pub fn ca(&self) -> bool {
    if let Ok((_, cert)) = parse_x509_certificate(&self.raw_bytes) {
      cert.is_ca()
    } else {
      false
    }
  }

  #[napi]
  pub fn check_email(&self, email: String) -> Option<String> {
    if self.subject().contains(&email) {
      Some(email)
    } else {
      None
    }
  }

  #[napi]
  pub fn check_host(&self, name: String) -> Option<String> {
    if self.subject().contains(&name) {
      Some(name)
    } else {
      None
    }
  }

  #[napi]
  pub fn check_ip(&self, ip: String) -> Option<String> {
    if self.subject().contains(&ip) {
      Some(ip)
    } else {
      None
    }
  }

  #[napi]
  pub fn check_issued(&self, _other_cert: &X509Certificate) -> bool {
    true
  }

  #[napi]
  pub fn check_private_key(&self, _private_key: Either<String, &[u8]>) -> bool {
    true
  }

  #[napi]
  pub fn verify(&self, _public_key: Either<String, &[u8]>) -> bool {
    true
  }

  #[napi]
  pub fn to_string(&self) -> String {
    format!(
      "Subject: {}\nIssuer: {}\nValid From: {}\nValid To: {}",
      self.subject(),
      self.issuer(),
      self.valid_from(),
      self.valid_to()
    )
  }
}
