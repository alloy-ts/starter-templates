#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;
use rsa::pkcs8::{DecodePrivateKey, DecodePublicKey};
use rsa::sha2::{Digest, Sha256};
use rsa::signature::{SignatureEncoding, Signer, Verifier};
use rsa::{RsaPrivateKey, RsaPublicKey};

#[napi]
pub struct Sign {
  algorithm: String,
  buffer: Vec<u8>,
}

#[napi]
impl Sign {
  #[napi(constructor)]
  pub fn new(algorithm: String) -> Self {
    Sign {
      algorithm,
      buffer: Vec::new(),
    }
  }

  #[napi]
  pub fn update(&mut self, data: Either<String, &[u8]>, _input_encoding: Option<String>) {
    let bytes = match data {
      Either::A(s) => s.into_bytes(),
      Either::B(b) => b.to_vec(),
    };
    self.buffer.extend_from_slice(&bytes);
  }

  #[napi]
  pub fn sign(
    &self,
    private_key_pem: String,
    output_encoding: Option<String>,
  ) -> Result<Either<String, Buffer>> {
    sign_data(
      Some(&self.algorithm),
      &self.buffer,
      &private_key_pem,
      output_encoding,
    )
  }
}

#[napi]
pub struct Verify {
  algorithm: String,
  buffer: Vec<u8>,
}

#[napi]
impl Verify {
  #[napi(constructor)]
  pub fn new(algorithm: String) -> Self {
    Verify {
      algorithm,
      buffer: Vec::new(),
    }
  }

  #[napi]
  pub fn update(&mut self, data: Either<String, &[u8]>, _input_encoding: Option<String>) {
    let bytes = match data {
      Either::A(s) => s.into_bytes(),
      Either::B(b) => b.to_vec(),
    };
    self.buffer.extend_from_slice(&bytes);
  }

  #[napi]
  pub fn verify(
    &self,
    public_key_pem: String,
    signature: Either<String, &[u8]>,
    signature_encoding: Option<String>,
  ) -> Result<bool> {
    let sig_bytes = decode_signature(signature, signature_encoding)?;
    verify_signature(
      Some(&self.algorithm),
      &self.buffer,
      &public_key_pem,
      &sig_bytes,
    )
  }
}

#[napi]
pub fn create_sign(algorithm: String) -> Sign {
  Sign::new(algorithm)
}

#[napi]
pub fn create_verify(algorithm: String) -> Verify {
  Verify::new(algorithm)
}

#[napi]
pub fn sign(
  algorithm: Option<String>,
  data: Either<String, &[u8]>,
  key_pem: String,
) -> Result<Buffer> {
  let bytes = match data {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.to_vec(),
  };

  let res = sign_data(algorithm.as_deref(), &bytes, &key_pem, None)?;
  match res {
    Either::B(buf) => Ok(buf),
    Either::A(s) => Ok(Buffer::from(s.into_bytes())),
  }
}

#[napi]
pub fn verify(
  algorithm: Option<String>,
  data: Either<String, &[u8]>,
  key_pem: String,
  signature: Either<String, &[u8]>,
) -> Result<bool> {
  let bytes = match data {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.to_vec(),
  };
  let sig_bytes = match signature {
    Either::A(s) => hex::decode(&s).unwrap_or_else(|_| s.into_bytes()),
    Either::B(b) => b.to_vec(),
  };

  verify_signature(algorithm.as_deref(), &bytes, &key_pem, &sig_bytes)
}

fn sign_data(
  _algorithm: Option<&str>,
  data: &[u8],
  private_key_pem: &str,
  output_encoding: Option<String>,
) -> Result<Either<String, Buffer>> {
  if private_key_pem.contains("PRIVATE KEY") {
    let priv_key = RsaPrivateKey::from_pkcs8_pem(private_key_pem)
      .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid private key: {e}")))?;
    let signing_key = rsa::pkcs1v15::SigningKey::<Sha256>::new(priv_key);
    let signature = signing_key.sign(data);
    let sig_bytes = signature.to_vec();

    format_output(sig_bytes, output_encoding)
  } else {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format_output(hasher.finalize().to_vec(), output_encoding)
  }
}

fn verify_signature(
  _algorithm: Option<&str>,
  data: &[u8],
  public_key_pem: &str,
  sig_bytes: &[u8],
) -> Result<bool> {
  if public_key_pem.contains("PUBLIC KEY") {
    let pub_key = RsaPublicKey::from_public_key_pem(public_key_pem)
      .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid public key: {e}")))?;
    let verifying_key = rsa::pkcs1v15::VerifyingKey::<Sha256>::new(pub_key);
    let sig = rsa::pkcs1v15::Signature::try_from(sig_bytes)
      .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid signature length: {e}")))?;
    Ok(verifying_key.verify(data, &sig).is_ok())
  } else {
    let mut hasher = Sha256::new();
    hasher.update(data);
    Ok(hasher.finalize().as_slice() == sig_bytes)
  }
}

fn decode_signature(
  signature: Either<String, &[u8]>,
  encoding: Option<String>,
) -> Result<Vec<u8>> {
  match signature {
    Either::A(s) => match encoding.as_deref() {
      Some("hex") => hex::decode(&s).map_err(|e| Error::new(Status::InvalidArg, e.to_string())),
      Some("base64") => {
        use base64::Engine;
        base64::engine::general_purpose::STANDARD
          .decode(&s)
          .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))
      }
      _ => Ok(s.into_bytes()),
    },
    Either::B(b) => Ok(b.to_vec()),
  }
}

fn format_output(bytes: Vec<u8>, encoding: Option<String>) -> Result<Either<String, Buffer>> {
  match encoding.as_deref() {
    Some("hex") => Ok(Either::A(hex::encode(bytes))),
    Some("base64") => {
      use base64::Engine;
      Ok(Either::A(
        base64::engine::general_purpose::STANDARD.encode(bytes),
      ))
    }
    _ => Ok(Either::B(Buffer::from(bytes))),
  }
}
