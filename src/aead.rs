#![deny(clippy::all)]

use aes_gcm::aead::{Aead, KeyInit, Payload};
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub struct Cipher {
  algorithm: String,
  key: Vec<u8>,
  iv: Vec<u8>,
  aad: Vec<u8>,
  auth_tag: Option<Vec<u8>>,
  buffer: Vec<u8>,
}

#[napi]
impl Cipher {
  #[napi(constructor)]
  pub fn new(
    algorithm: String,
    key: Either<String, &[u8]>,
    iv: Either<String, &[u8]>,
  ) -> Result<Self> {
    let key_bytes = to_bytes(key);
    let iv_bytes = to_bytes(iv);

    Ok(Cipher {
      algorithm,
      key: key_bytes,
      iv: iv_bytes,
      aad: Vec::new(),
      auth_tag: None,
      buffer: Vec::new(),
    })
  }

  #[napi]
  pub fn set_aad(&mut self, data: Either<String, &[u8]>) -> Result<()> {
    self.aad.extend_from_slice(&to_bytes(data));
    Ok(())
  }

  #[napi]
  pub fn update(
    &mut self,
    data: Either<String, &[u8]>,
    _input_encoding: Option<String>,
    _output_encoding: Option<String>,
  ) -> Result<Buffer> {
    let bytes = to_bytes(data);
    self.buffer.extend_from_slice(&bytes);
    Ok(Buffer::from(vec![]))
  }

  #[napi]
  pub fn final_cipher(&mut self, output_encoding: Option<String>) -> Result<Either<String, Buffer>> {
    let alg_lower = self.algorithm.to_lowercase();
    let ciphertext_and_tag = match alg_lower.as_str() {
      "aes-128-gcm" => {
        use aes_gcm::KeyInit;
        let cipher = aes_gcm::Aes128Gcm::new_from_slice(&self.key)
          .map_err(|e| Error::new(Status::InvalidArg, format!("{e:?}")))?;
        let nonce = aes_gcm::Nonce::from_slice(&self.iv);
        let payload = Payload {
          msg: &self.buffer,
          aad: &self.aad,
        };
        Aead::encrypt(&cipher, nonce, payload)
          .map_err(|e| Error::new(Status::GenericFailure, format!("{e:?}")))?
      }
      "aes-256-gcm" => {
        use aes_gcm::KeyInit;
        let cipher = aes_gcm::Aes256Gcm::new_from_slice(&self.key)
          .map_err(|e| Error::new(Status::InvalidArg, format!("{e:?}")))?;
        let nonce = aes_gcm::Nonce::from_slice(&self.iv);
        let payload = Payload {
          msg: &self.buffer,
          aad: &self.aad,
        };
        Aead::encrypt(&cipher, nonce, payload)
          .map_err(|e| Error::new(Status::GenericFailure, format!("{e:?}")))?
      }
      "chacha20-poly1305" => {
        use chacha20poly1305::KeyInit;
        let cipher = chacha20poly1305::ChaCha20Poly1305::new_from_slice(&self.key)
          .map_err(|e| Error::new(Status::InvalidArg, format!("{e:?}")))?;
        let nonce = chacha20poly1305::Nonce::from_slice(&self.iv);
        let payload = Payload {
          msg: &self.buffer,
          aad: &self.aad,
        };
        Aead::encrypt(&cipher, nonce, payload)
          .map_err(|e| Error::new(Status::GenericFailure, format!("{e:?}")))?
      }
      "aes-128-siv" => {
        use aes_siv::KeyInit;
        let cipher = aes_siv::Aes128SivAead::new_from_slice(&self.key)
          .map_err(|e| Error::new(Status::InvalidArg, format!("{e:?}")))?;
        let nonce = aes_siv::Nonce::from_slice(&self.iv);
        let payload = Payload {
          msg: &self.buffer,
          aad: &self.aad,
        };
        Aead::encrypt(&cipher, nonce, payload)
          .map_err(|e| Error::new(Status::GenericFailure, format!("{e:?}")))?
      }
      "aes-256-siv" => {
        use aes_siv::KeyInit;
        let cipher = aes_siv::Aes256SivAead::new_from_slice(&self.key)
          .map_err(|e| Error::new(Status::InvalidArg, format!("{e:?}")))?;
        let nonce = aes_siv::Nonce::from_slice(&self.iv);
        let payload = Payload {
          msg: &self.buffer,
          aad: &self.aad,
        };
        Aead::encrypt(&cipher, nonce, payload)
          .map_err(|e| Error::new(Status::GenericFailure, format!("{e:?}")))?
      }
      _ => {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Unsupported AEAD cipher: {}", self.algorithm),
        ));
      }
    };

    let tag_len = 16;
    if ciphertext_and_tag.len() >= tag_len {
      let split_pos = ciphertext_and_tag.len() - tag_len;
      let ciphertext = ciphertext_and_tag[..split_pos].to_vec();
      let tag = ciphertext_and_tag[split_pos..].to_vec();
      self.auth_tag = Some(tag);

      format_output(ciphertext, output_encoding)
    } else {
      format_output(ciphertext_and_tag, output_encoding)
    }
  }

  #[napi]
  pub fn get_auth_tag(&self) -> Result<Buffer> {
    self
      .auth_tag
      .clone()
      .map(Buffer::from)
      .ok_or_else(|| Error::new(Status::GenericFailure, "Auth tag not generated yet"))
  }
}

#[napi]
pub struct Decipher {
  algorithm: String,
  key: Vec<u8>,
  iv: Vec<u8>,
  aad: Vec<u8>,
  auth_tag: Option<Vec<u8>>,
  buffer: Vec<u8>,
}

#[napi]
impl Decipher {
  #[napi(constructor)]
  pub fn new(
    algorithm: String,
    key: Either<String, &[u8]>,
    iv: Either<String, &[u8]>,
  ) -> Result<Self> {
    Ok(Decipher {
      algorithm,
      key: to_bytes(key),
      iv: to_bytes(iv),
      aad: Vec::new(),
      auth_tag: None,
      buffer: Vec::new(),
    })
  }

  #[napi]
  pub fn set_aad(&mut self, data: Either<String, &[u8]>) -> Result<()> {
    self.aad.extend_from_slice(&to_bytes(data));
    Ok(())
  }

  #[napi]
  pub fn set_auth_tag(&mut self, tag: Either<String, &[u8]>) -> Result<()> {
    self.auth_tag = Some(to_bytes(tag));
    Ok(())
  }

  #[napi]
  pub fn update(
    &mut self,
    data: Either<String, &[u8]>,
    _input_encoding: Option<String>,
    _output_encoding: Option<String>,
  ) -> Result<Buffer> {
    self.buffer.extend_from_slice(&to_bytes(data));
    Ok(Buffer::from(vec![]))
  }

  #[napi]
  pub fn final_decipher(&mut self, output_encoding: Option<String>) -> Result<Either<String, Buffer>> {
    let mut payload_bytes = self.buffer.clone();
    if let Some(tag) = &self.auth_tag {
      payload_bytes.extend_from_slice(tag);
    }

    let alg_lower = self.algorithm.to_lowercase();
    let plaintext = match alg_lower.as_str() {
      "aes-128-gcm" => {
        use aes_gcm::KeyInit;
        let cipher = aes_gcm::Aes128Gcm::new_from_slice(&self.key)
          .map_err(|e| Error::new(Status::InvalidArg, format!("{e:?}")))?;
        let nonce = aes_gcm::Nonce::from_slice(&self.iv);
        let payload = Payload {
          msg: &payload_bytes,
          aad: &self.aad,
        };
        Aead::decrypt(&cipher, nonce, payload)
          .map_err(|e| Error::new(Status::GenericFailure, format!("Decryption failed: {e:?}")))?
      }
      "aes-256-gcm" => {
        use aes_gcm::KeyInit;
        let cipher = aes_gcm::Aes256Gcm::new_from_slice(&self.key)
          .map_err(|e| Error::new(Status::InvalidArg, format!("{e:?}")))?;
        let nonce = aes_gcm::Nonce::from_slice(&self.iv);
        let payload = Payload {
          msg: &payload_bytes,
          aad: &self.aad,
        };
        Aead::decrypt(&cipher, nonce, payload)
          .map_err(|e| Error::new(Status::GenericFailure, format!("Decryption failed: {e:?}")))?
      }
      "chacha20-poly1305" => {
        use chacha20poly1305::KeyInit;
        let cipher = chacha20poly1305::ChaCha20Poly1305::new_from_slice(&self.key)
          .map_err(|e| Error::new(Status::InvalidArg, format!("{e:?}")))?;
        let nonce = chacha20poly1305::Nonce::from_slice(&self.iv);
        let payload = Payload {
          msg: &payload_bytes,
          aad: &self.aad,
        };
        Aead::decrypt(&cipher, nonce, payload)
          .map_err(|e| Error::new(Status::GenericFailure, format!("Decryption failed: {e:?}")))?
      }
      "aes-128-siv" => {
        use aes_siv::KeyInit;
        let cipher = aes_siv::Aes128SivAead::new_from_slice(&self.key)
          .map_err(|e| Error::new(Status::InvalidArg, format!("{e:?}")))?;
        let nonce = aes_siv::Nonce::from_slice(&self.iv);
        let payload = Payload {
          msg: &payload_bytes,
          aad: &self.aad,
        };
        Aead::decrypt(&cipher, nonce, payload)
          .map_err(|e| Error::new(Status::GenericFailure, format!("Decryption failed: {e:?}")))?
      }
      "aes-256-siv" => {
        use aes_siv::KeyInit;
        let cipher = aes_siv::Aes256SivAead::new_from_slice(&self.key)
          .map_err(|e| Error::new(Status::InvalidArg, format!("{e:?}")))?;
        let nonce = aes_siv::Nonce::from_slice(&self.iv);
        let payload = Payload {
          msg: &payload_bytes,
          aad: &self.aad,
        };
        Aead::decrypt(&cipher, nonce, payload)
          .map_err(|e| Error::new(Status::GenericFailure, format!("Decryption failed: {e:?}")))?
      }
      _ => {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Unsupported AEAD decipher: {}", self.algorithm),
        ));
      }
    };

    format_output(plaintext, output_encoding)
  }
}

fn to_bytes(data: Either<String, &[u8]>) -> Vec<u8> {
  match data {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.to_vec(),
  }
}

fn format_output(bytes: Vec<u8>, output_encoding: Option<String>) -> Result<Either<String, Buffer>> {
  match output_encoding.as_deref() {
    Some("hex") => Ok(Either::A(hex::encode(bytes))),
    Some("base64") => {
      use base64::Engine;
      Ok(Either::A(
        base64::engine::general_purpose::STANDARD.encode(bytes),
      ))
    }
    Some("utf8") | Some("utf-8") => {
      let s = String::from_utf8(bytes)
        .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
      Ok(Either::A(s))
    }
    _ => Ok(Either::B(Buffer::from(bytes))),
  }
}

#[napi]
pub fn create_cipheriv(
  algorithm: String,
  key: Either<String, &[u8]>,
  iv: Either<String, &[u8]>,
) -> Result<Cipher> {
  Cipher::new(algorithm, key, iv)
}

#[napi]
pub fn create_decipheriv(
  algorithm: String,
  key: Either<String, &[u8]>,
  iv: Either<String, &[u8]>,
) -> Result<Decipher> {
  Decipher::new(algorithm, key, iv)
}
