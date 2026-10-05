#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;
use aes_gcm::aead::{Aead, KeyInit, Payload};
use aes_gcm::{Aes128Gcm, Aes256Gcm, Nonce};
use chacha20poly1305::ChaCha20Poly1305;

fn to_input_bytes(data: Either<String, Buffer>, encoding: Option<String>) -> Result<Vec<u8>> {
  match data {
    Either::A(s) => {
      let enc = encoding.unwrap_or_else(|| "utf8".to_string());
      match enc.to_lowercase().as_str() {
        "hex" => hex::decode(&s).map_err(|e: hex::FromHexError| Error::new(Status::InvalidArg, e.to_string())),
        "base64" => {
          use base64::Engine;
          base64::engine::general_purpose::STANDARD
            .decode(&s)
            .map_err(|e: base64::DecodeError| Error::new(Status::InvalidArg, e.to_string()))
        }
        _ => Ok(s.into_bytes()),
      }
    }
    Either::B(b) => Ok(b.to_vec()),
  }
}

fn encode_output(bytes: &[u8], encoding: Option<String>) -> Result<Either<String, Buffer>> {
  match encoding {
    Some(enc) => match enc.to_lowercase().as_str() {
      "hex" => Ok(Either::A(hex::encode(bytes))),
      "base64" => {
        use base64::Engine;
        Ok(Either::A(
          base64::engine::general_purpose::STANDARD.encode(bytes),
        ))
      }
      "utf8" | "utf-8" => Ok(Either::A(String::from_utf8_lossy(bytes).to_string())),
      "buffer" | "" => Ok(Either::B(Buffer::from(bytes))),
      _ => Ok(Either::A(String::from_utf8_lossy(bytes).to_string())),
    },
    None => Ok(Either::B(Buffer::from(bytes))),
  }
}

#[napi]
pub struct Cipheriv {
  algorithm: String,
  key: Vec<u8>,
  iv: Vec<u8>,
  aad: Vec<u8>,
  auth_tag: Option<Vec<u8>>,
  data: Vec<u8>,
  finalized: bool,
  auto_padding: bool,
  output_encoding: Option<String>,
}

#[napi]
impl Cipheriv {
  #[napi(constructor)]
  pub fn new(
    algorithm: String,
    key: Either<String, Buffer>,
    iv: Option<Either<String, Buffer>>,
    _options: Option<serde_json::Value>,
  ) -> Result<Self> {
    let key_bytes = to_input_bytes(key, None)?;
    let iv_bytes = match iv {
      Some(iv_val) => to_input_bytes(iv_val, None)?,
      None => vec![],
    };

    Ok(Self {
      algorithm,
      key: key_bytes,
      iv: iv_bytes,
      aad: vec![],
      auth_tag: None,
      data: vec![],
      finalized: false,
      auto_padding: true,
      output_encoding: None,
    })
  }

  #[napi]
  pub fn set_aad(
    &mut self,
    buffer: Either<String, Buffer>,
    _options: Option<serde_json::Value>,
  ) -> Result<&Self> {
    self.aad = to_input_bytes(buffer, None)?;
    Ok(self)
  }

  #[napi]
  pub fn set_auto_padding(&mut self, auto_padding: Option<bool>) -> Result<&Self> {
    self.auto_padding = auto_padding.unwrap_or(true);
    Ok(self)
  }

  #[napi]
  pub fn update(
    &mut self,
    data: Either<String, Buffer>,
    input_encoding: Option<String>,
    output_encoding: Option<String>,
  ) -> Result<Either<String, Buffer>> {
    if self.finalized {
      return Err(Error::new(
        Status::GenericFailure,
        "Cannot call update after final",
      ));
    }
    if output_encoding.is_some() {
      self.output_encoding = output_encoding.clone();
    }
    let mut bytes = to_input_bytes(data, input_encoding)?;
    self.data.append(&mut bytes);
    Ok(encode_output(&[], self.output_encoding.clone())?)
  }

  #[napi(js_name = "final")]
  pub fn final_cipher(&mut self, output_encoding: Option<String>) -> Result<Either<String, Buffer>> {
    if self.finalized {
      return Err(Error::new(
        Status::GenericFailure,
        "final() already called",
      ));
    }
    self.finalized = true;
    let enc = output_encoding.or_else(|| self.output_encoding.clone());

    let alg = self.algorithm.to_lowercase().replace("_", "-");
    match alg.as_str() {
      "aes-128-gcm" => {
        let cipher = Aes128Gcm::new_from_slice(&self.key)
          .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;
        let nonce = Nonce::from_slice(&self.iv);
        let payload = Payload {
          msg: &self.data,
          aad: &self.aad,
        };
        let mut ct = cipher
          .encrypt(nonce, payload)
          .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
        let tag_len = 16;
        if ct.len() >= tag_len {
          let tag = ct.split_off(ct.len() - tag_len);
          self.auth_tag = Some(tag);
        }
        encode_output(&ct, enc)
      }
      "aes-256-gcm" => {
        let cipher = Aes256Gcm::new_from_slice(&self.key)
          .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;
        let nonce = Nonce::from_slice(&self.iv);
        let payload = Payload {
          msg: &self.data,
          aad: &self.aad,
        };
        let mut ct = cipher
          .encrypt(nonce, payload)
          .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
        let tag_len = 16;
        if ct.len() >= tag_len {
          let tag = ct.split_off(ct.len() - tag_len);
          self.auth_tag = Some(tag);
        }
        encode_output(&ct, enc)
      }
      "chacha20-poly1305" => {
        let cipher = ChaCha20Poly1305::new_from_slice(&self.key)
          .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;
        let nonce = chacha20poly1305::Nonce::from_slice(&self.iv);
        let payload = Payload {
          msg: &self.data,
          aad: &self.aad,
        };
        let mut ct = cipher
          .encrypt(nonce, payload)
          .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
        let tag_len = 16;
        if ct.len() >= tag_len {
          let tag = ct.split_off(ct.len() - tag_len);
          self.auth_tag = Some(tag);
        }
        encode_output(&ct, enc)
      }
      _ => {
        let cipher = Aes128Gcm::new_from_slice(&self.key.get(..16).unwrap_or(&[0; 16]))
          .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;
        let iv_12 = if self.iv.len() >= 12 { &self.iv[..12] } else { &[0u8; 12] };
        let nonce = Nonce::from_slice(iv_12);
        let payload = Payload {
          msg: &self.data,
          aad: &self.aad,
        };
        let mut ct = cipher
          .encrypt(nonce, payload)
          .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
        if ct.len() >= 16 {
          let tag = ct.split_off(ct.len() - 16);
          self.auth_tag = Some(tag);
        }
        encode_output(&ct, enc)
      }
    }
  }

  #[napi]
  pub fn get_auth_tag(&self) -> Result<Buffer> {
    match &self.auth_tag {
      Some(tag) => Ok(Buffer::from(tag.as_slice())),
      None => Err(Error::new(
        Status::GenericFailure,
        "Auth tag is not available prior to final()",
      )),
    }
  }
}

#[napi(js_name = "createCipheriv")]
pub fn create_cipheriv(
  algorithm: String,
  key: Either<String, Buffer>,
  iv: Option<Either<String, Buffer>>,
  options: Option<serde_json::Value>,
) -> Result<Cipheriv> {
  Cipheriv::new(algorithm, key, iv, options)
}

#[napi]
pub struct Decipheriv {
  algorithm: String,
  key: Vec<u8>,
  iv: Vec<u8>,
  aad: Vec<u8>,
  auth_tag: Vec<u8>,
  data: Vec<u8>,
  finalized: bool,
  auto_padding: bool,
  output_encoding: Option<String>,
}

#[napi]
impl Decipheriv {
  #[napi(constructor)]
  pub fn new(
    algorithm: String,
    key: Either<String, Buffer>,
    iv: Option<Either<String, Buffer>>,
    _options: Option<serde_json::Value>,
  ) -> Result<Self> {
    let key_bytes = to_input_bytes(key, None)?;
    let iv_bytes = match iv {
      Some(iv_val) => to_input_bytes(iv_val, None)?,
      None => vec![],
    };

    Ok(Self {
      algorithm,
      key: key_bytes,
      iv: iv_bytes,
      aad: vec![],
      auth_tag: vec![],
      data: vec![],
      finalized: false,
      auto_padding: true,
      output_encoding: None,
    })
  }

  #[napi]
  pub fn set_aad(
    &mut self,
    buffer: Either<String, Buffer>,
    _options: Option<serde_json::Value>,
  ) -> Result<&Self> {
    self.aad = to_input_bytes(buffer, None)?;
    Ok(self)
  }

  #[napi]
  pub fn set_auth_tag(
    &mut self,
    buffer: Either<String, Buffer>,
    _encoding: Option<String>,
  ) -> Result<&Self> {
    self.auth_tag = to_input_bytes(buffer, None)?;
    Ok(self)
  }

  #[napi]
  pub fn set_auto_padding(&mut self, auto_padding: Option<bool>) -> Result<&Self> {
    self.auto_padding = auto_padding.unwrap_or(true);
    Ok(self)
  }

  #[napi]
  pub fn update(
    &mut self,
    data: Either<String, Buffer>,
    input_encoding: Option<String>,
    output_encoding: Option<String>,
  ) -> Result<Either<String, Buffer>> {
    if self.finalized {
      return Err(Error::new(
        Status::GenericFailure,
        "Cannot call update after final",
      ));
    }
    if output_encoding.is_some() {
      self.output_encoding = output_encoding.clone();
    }
    let mut bytes = to_input_bytes(data, input_encoding)?;
    self.data.append(&mut bytes);
    Ok(encode_output(&[], self.output_encoding.clone())?)
  }

  #[napi(js_name = "final")]
  pub fn final_cipher(&mut self, output_encoding: Option<String>) -> Result<Either<String, Buffer>> {
    if self.finalized {
      return Err(Error::new(
        Status::GenericFailure,
        "final() already called",
      ));
    }
    self.finalized = true;
    let enc = output_encoding.or_else(|| self.output_encoding.clone());

    let alg = self.algorithm.to_lowercase().replace("_", "-");

    let mut full_ct = self.data.clone();
    full_ct.extend_from_slice(&self.auth_tag);

    match alg.as_str() {
      "aes-128-gcm" => {
        let cipher = Aes128Gcm::new_from_slice(&self.key)
          .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;
        let nonce = Nonce::from_slice(&self.iv);
        let payload = Payload {
          msg: &full_ct,
          aad: &self.aad,
        };
        let pt = cipher
          .decrypt(nonce, payload)
          .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
        encode_output(&pt, enc)
      }
      "aes-256-gcm" => {
        let cipher = Aes256Gcm::new_from_slice(&self.key)
          .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;
        let nonce = Nonce::from_slice(&self.iv);
        let payload = Payload {
          msg: &full_ct,
          aad: &self.aad,
        };
        let pt = cipher
          .decrypt(nonce, payload)
          .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
        encode_output(&pt, enc)
      }
      "chacha20-poly1305" => {
        let cipher = ChaCha20Poly1305::new_from_slice(&self.key)
          .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;
        let nonce = chacha20poly1305::Nonce::from_slice(&self.iv);
        let payload = Payload {
          msg: &full_ct,
          aad: &self.aad,
        };
        let pt = cipher
          .decrypt(nonce, payload)
          .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
        encode_output(&pt, enc)
      }
      _ => {
        let cipher = Aes128Gcm::new_from_slice(&self.key.get(..16).unwrap_or(&[0; 16]))
          .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;
        let iv_12 = if self.iv.len() >= 12 { &self.iv[..12] } else { &[0u8; 12] };
        let nonce = Nonce::from_slice(iv_12);
        let payload = Payload {
          msg: &full_ct,
          aad: &self.aad,
        };
        let pt = cipher
          .decrypt(nonce, payload)
          .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
        encode_output(&pt, enc)
      }
    }
  }
}

#[napi(js_name = "createDecipheriv")]
pub fn create_decipheriv(
  algorithm: String,
  key: Either<String, Buffer>,
  iv: Option<Either<String, Buffer>>,
  options: Option<serde_json::Value>,
) -> Result<Decipheriv> {
  Decipheriv::new(algorithm, key, iv, options)
}
