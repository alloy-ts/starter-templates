use aes_gcm::{aead::{Aead, KeyInit, Payload}, Aes128Gcm, Aes256Gcm, Nonce as GcmNonce};
use aes_siv::{Aes128SivAead, Aes256SivAead};
use ccm::{Ccm, consts::{U12, U16}};
use cipher::Array;
use aes::Aes128;
use napi::bindgen_prelude::*;
use napi_derive::napi;

type Aes128Ccm = Ccm<Aes128, U16, U12>;

#[napi(object)]
pub struct AeadEncryptResult {
  pub ciphertext: Buffer,
  pub auth_tag: Buffer,
}

#[napi]
pub fn encrypt_aead(
  algorithm: String,
  key: Buffer,
  iv: Buffer,
  plaintext: Buffer,
  aad: Option<Buffer>,
) -> Result<AeadEncryptResult> {
  let algo = algorithm.to_lowercase();
  let aad_bytes = aad.as_deref().unwrap_or(&[]);

  match algo.as_str() {
    "aes-128-gcm" | "aes-256-gcm" => {
      if algo == "aes-128-gcm" {
        let cipher = Aes128Gcm::new_from_slice(&key)
          .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;
        let nonce = GcmNonce::from_slice(&iv);
        let payload = Payload {
          msg: &plaintext,
          aad: aad_bytes,
        };
        let encrypted = cipher.encrypt(nonce, payload)
          .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
        let tag_start = encrypted.len().saturating_sub(16);
        let ct = encrypted[..tag_start].to_vec();
        let tag = encrypted[tag_start..].to_vec();
        Ok(AeadEncryptResult {
          ciphertext: Buffer::from(ct),
          auth_tag: Buffer::from(tag),
        })
      } else {
        let cipher = Aes256Gcm::new_from_slice(&key)
          .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;
        let nonce = GcmNonce::from_slice(&iv);
        let payload = Payload {
          msg: &plaintext,
          aad: aad_bytes,
        };
        let encrypted = cipher.encrypt(nonce, payload)
          .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
        let tag_start = encrypted.len().saturating_sub(16);
        let ct = encrypted[..tag_start].to_vec();
        let tag = encrypted[tag_start..].to_vec();
        Ok(AeadEncryptResult {
          ciphertext: Buffer::from(ct),
          auth_tag: Buffer::from(tag),
        })
      }
    }
    "aes-128-siv" | "aes-256-siv" => {
      if algo == "aes-128-siv" {
        let cipher = Aes128SivAead::new_from_slice(&key)
          .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;
        let nonce = aes_siv::Nonce::from_slice(&iv);
        let payload = Payload {
          msg: &plaintext,
          aad: aad_bytes,
        };
        let encrypted = cipher.encrypt(nonce, payload)
          .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
        let tag_start = encrypted.len().saturating_sub(16);
        let ct = encrypted[..tag_start].to_vec();
        let tag = encrypted[tag_start..].to_vec();
        Ok(AeadEncryptResult {
          ciphertext: Buffer::from(ct),
          auth_tag: Buffer::from(tag),
        })
      } else {
        let cipher = Aes256SivAead::new_from_slice(&key)
          .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;
        let nonce = aes_siv::Nonce::from_slice(&iv);
        let payload = Payload {
          msg: &plaintext,
          aad: aad_bytes,
        };
        let encrypted = cipher.encrypt(nonce, payload)
          .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
        let tag_start = encrypted.len().saturating_sub(16);
        let ct = encrypted[..tag_start].to_vec();
        let tag = encrypted[tag_start..].to_vec();
        Ok(AeadEncryptResult {
          ciphertext: Buffer::from(ct),
          auth_tag: Buffer::from(tag),
        })
      }
    }
    "aes-128-ccm" => {
      let cipher = Aes128Ccm::new_from_slice(&key)
        .map_err(|_| Error::new(Status::InvalidArg, "Invalid CCM key length"))?;
      let nonce = Array::from_slice(&iv);
      let payload = Payload {
        msg: &plaintext,
        aad: aad_bytes,
      };
      let encrypted = cipher.encrypt(nonce, payload)
        .map_err(|_| Error::new(Status::GenericFailure, "CCM encryption failed"))?;
      let tag_start = encrypted.len().saturating_sub(16);
      let ct = encrypted[..tag_start].to_vec();
      let tag = encrypted[tag_start..].to_vec();
      Ok(AeadEncryptResult {
        ciphertext: Buffer::from(ct),
        auth_tag: Buffer::from(tag),
      })
    }
    _ => Err(Error::new(
      Status::InvalidArg,
      format!("Unsupported AEAD algorithm: {}", algorithm),
    )),
  }
}

#[napi]
pub fn decrypt_aead(
  algorithm: String,
  key: Buffer,
  iv: Buffer,
  ciphertext: Buffer,
  auth_tag: Buffer,
  aad: Option<Buffer>,
) -> Result<Buffer> {
  let algo = algorithm.to_lowercase();
  let aad_bytes = aad.as_deref().unwrap_or(&[]);

  let mut combined = ciphertext.to_vec();
  combined.extend_from_slice(&auth_tag);

  match algo.as_str() {
    "aes-128-gcm" => {
      let cipher = Aes128Gcm::new_from_slice(&key)
        .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;
      let nonce = GcmNonce::from_slice(&iv);
      let payload = Payload {
        msg: &combined,
        aad: aad_bytes,
      };
      let pt = cipher.decrypt(nonce, payload)
        .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
      Ok(Buffer::from(pt))
    }
    "aes-256-gcm" => {
      let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;
      let nonce = GcmNonce::from_slice(&iv);
      let payload = Payload {
        msg: &combined,
        aad: aad_bytes,
      };
      let pt = cipher.decrypt(nonce, payload)
        .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
      Ok(Buffer::from(pt))
    }
    "aes-128-siv" => {
      let cipher = Aes128SivAead::new_from_slice(&key)
        .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;
      let nonce = aes_siv::Nonce::from_slice(&iv);
      let payload = Payload {
        msg: &combined,
        aad: aad_bytes,
      };
      let pt = cipher.decrypt(nonce, payload)
        .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
      Ok(Buffer::from(pt))
    }
    "aes-256-siv" => {
      let cipher = Aes256SivAead::new_from_slice(&key)
        .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;
      let nonce = aes_siv::Nonce::from_slice(&iv);
      let payload = Payload {
        msg: &combined,
        aad: aad_bytes,
      };
      let pt = cipher.decrypt(nonce, payload)
        .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
      Ok(Buffer::from(pt))
    }
    "aes-128-ccm" => {
      let cipher = Aes128Ccm::new_from_slice(&key)
        .map_err(|_| Error::new(Status::InvalidArg, "Invalid CCM key length"))?;
      let nonce = Array::from_slice(&iv);
      let payload = Payload {
        msg: &combined,
        aad: aad_bytes,
      };
      let pt = cipher.decrypt(nonce, payload)
        .map_err(|_| Error::new(Status::GenericFailure, "CCM decryption failed"))?;
      Ok(Buffer::from(pt))
    }
    _ => Err(Error::new(
      Status::InvalidArg,
      format!("Unsupported AEAD algorithm: {}", algorithm),
    )),
  }
}
