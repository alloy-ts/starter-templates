#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;
use p256::elliptic_curve::ecdh::diffie_hellman;
use p256::elliptic_curve::sec1::ToSec1Point;

#[napi]
pub struct Ecdh {
  curve_name: String,
  private_key: Option<Vec<u8>>,
  public_key: Option<Vec<u8>>,
}

#[napi]
impl Ecdh {
  #[napi(constructor)]
  pub fn new(curve_name: String) -> Result<Self> {
    let mut ecdh = Ecdh {
      curve_name,
      private_key: None,
      public_key: None,
    };
    ecdh.generate_keys(None, None)?;
    Ok(ecdh)
  }

  #[napi]
  pub fn generate_keys(
    &mut self,
    encoding: Option<String>,
    _format: Option<String>,
  ) -> Result<Either<String, Buffer>> {
    let curve_lower = self.curve_name.to_lowercase();
    let mut rng = rand::rng();

    match curve_lower.as_str() {
      "prime256v1" | "p-256" | "p256" | "secp256r1" => {
        let secret = p256::SecretKey::random(&mut rng);
        let public = secret.public_key();
        self.private_key = Some(secret.to_bytes().to_vec());
        self.public_key = Some(public.to_encoded_point(false).as_bytes().to_vec());
      }
      "secp384r1" | "p-384" | "p384" => {
        let secret = p384::SecretKey::random(&mut rng);
        let public = secret.public_key();
        self.private_key = Some(secret.to_bytes().to_vec());
        self.public_key = Some(public.to_encoded_point(false).as_bytes().to_vec());
      }
      "secp521r1" | "p-521" | "p521" => {
        let secret = p521::SecretKey::random(&mut rng);
        let public = secret.public_key();
        self.private_key = Some(secret.to_bytes().to_vec());
        self.public_key = Some(public.to_encoded_point(false).as_bytes().to_vec());
      }
      "x25519" => {
        let secret = x25519_dalek::StaticSecret::random_from_rng(&mut rng);
        let public = x25519_dalek::PublicKey::from(&secret);
        self.private_key = Some(secret.to_bytes().to_vec());
        self.public_key = Some(public.as_bytes().to_vec());
      }
      _ => {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Unsupported curve: {}", self.curve_name),
        ));
      }
    }

    self.get_public_key(encoding, None)
  }

  #[napi]
  pub fn compute_secret(
    &self,
    other_public_key: Either<String, &[u8]>,
    input_encoding: Option<String>,
    output_encoding: Option<String>,
  ) -> Result<Either<String, Buffer>> {
    let priv_bytes = self
      .private_key
      .as_ref()
      .ok_or_else(|| Error::new(Status::GenericFailure, "No private key set"))?;

    let pub_bytes = decode_input(other_public_key, input_encoding)?;
    let curve_lower = self.curve_name.to_lowercase();

    let secret_bytes = match curve_lower.as_str() {
      "prime256v1" | "p-256" | "p256" | "secp256r1" => {
        let secret = p256::SecretKey::from_slice(priv_bytes)
          .map_err(|e: p256::elliptic_curve::Error| Error::new(Status::InvalidArg, e.to_string()))?;
        let public = p256::PublicKey::from_sec1_bytes(&pub_bytes)
          .map_err(|e: p256::elliptic_curve::Error| Error::new(Status::InvalidArg, e.to_string()))?;
        let shared = diffie_hellman(secret.to_nonzero_scalar(), public.as_affine());
        shared.raw_secret_bytes().to_vec()
      }
      "secp384r1" | "p-384" | "p384" => {
        let secret = p384::SecretKey::from_slice(priv_bytes)
          .map_err(|e: p384::elliptic_curve::Error| Error::new(Status::InvalidArg, e.to_string()))?;
        let public = p384::PublicKey::from_sec1_bytes(&pub_bytes)
          .map_err(|e: p384::elliptic_curve::Error| Error::new(Status::InvalidArg, e.to_string()))?;
        let shared = diffie_hellman(secret.to_nonzero_scalar(), public.as_affine());
        shared.raw_secret_bytes().to_vec()
      }
      "x25519" => {
        let mut priv_arr = [0u8; 32];
        let mut pub_arr = [0u8; 32];
        if priv_bytes.len() != 32 || pub_bytes.len() != 32 {
          return Err(Error::new(Status::InvalidArg, "Invalid key length for X25519"));
        }
        priv_arr.copy_from_slice(priv_bytes);
        pub_arr.copy_from_slice(&pub_bytes);
        let secret = x25519_dalek::StaticSecret::from(priv_arr);
        let public = x25519_dalek::PublicKey::from(pub_arr);
        let shared = secret.diffie_hellman(&public);
        shared.as_bytes().to_vec()
      }
      _ => {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Unsupported curve for ECDH: {}", self.curve_name),
        ));
      }
    };

    format_output(secret_bytes, output_encoding)
  }

  #[napi]
  pub fn get_public_key(
    &self,
    encoding: Option<String>,
    _format: Option<String>,
  ) -> Result<Either<String, Buffer>> {
    let pub_bytes = self
      .public_key
      .clone()
      .ok_or_else(|| Error::new(Status::GenericFailure, "No public key available"))?;
    format_output(pub_bytes, encoding)
  }

  #[napi]
  pub fn get_private_key(&self, encoding: Option<String>) -> Result<Either<String, Buffer>> {
    let priv_bytes = self
      .private_key
      .clone()
      .ok_or_else(|| Error::new(Status::GenericFailure, "No private key available"))?;
    format_output(priv_bytes, encoding)
  }

  #[napi]
  pub fn set_public_key(
    &mut self,
    key: Either<String, &[u8]>,
    encoding: Option<String>,
  ) -> Result<()> {
    self.public_key = Some(decode_input(key, encoding)?);
    Ok(())
  }

  #[napi]
  pub fn set_private_key(
    &mut self,
    key: Either<String, &[u8]>,
    encoding: Option<String>,
  ) -> Result<()> {
    self.private_key = Some(decode_input(key, encoding)?);
    Ok(())
  }
}

fn decode_input(input: Either<String, &[u8]>, encoding: Option<String>) -> Result<Vec<u8>> {
  match input {
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

#[napi]
pub fn create_ecdh(curve_name: String) -> Result<Ecdh> {
  Ecdh::new(curve_name)
}

#[napi]
pub fn create_diffie_hellman(group_name_or_prime_length: Either<String, u32>) -> Result<Ecdh> {
  let curve = match group_name_or_prime_length {
    Either::A(s) => s,
    Either::B(_) => "p256".to_string(),
  };
  Ecdh::new(curve)
}

#[napi]
pub fn create_diffie_hellman_group(group_name: String) -> Result<Ecdh> {
  Ecdh::new(group_name)
}
