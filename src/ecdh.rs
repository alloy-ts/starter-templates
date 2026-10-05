use napi::bindgen_prelude::*;
use napi_derive::napi;
use p256::elliptic_curve::ecdh::diffie_hellman;
use p256::elliptic_curve::sec1::ToSec1Point;
use p256::SecretKey as P256SecretKey;
use p256::PublicKey as P256PublicKey;
use p384::SecretKey as P384SecretKey;
use p384::PublicKey as P384PublicKey;

#[napi]
pub struct ECDH {
  curve: String,
  private_key: Vec<u8>,
  public_key: Vec<u8>,
}

#[napi]
impl ECDH {
  #[napi(constructor)]
  pub fn new(curve_name: String) -> Result<Self> {
    let curve = curve_name.to_lowercase();
    match curve.as_str() {
      "prime256v1" | "secp256r1" | "p-256" | "secp384r1" | "p-384" => {},
      other => return Err(Error::new(Status::InvalidArg, format!("Unsupported elliptic curve: {}", other))),
    }

    let mut ecdh = Self {
      curve,
      private_key: Vec::new(),
      public_key: Vec::new(),
    };
    ecdh.generate_keys(None, None)?;
    Ok(ecdh)
  }

  #[napi]
  pub fn generate_keys(
    &mut self,
    output_encoding: Option<String>,
    format: Option<String>,
  ) -> Result<Either<Buffer, String>> {
    let mut rng = rand::rng();

    match self.curve.as_str() {
      "prime256v1" | "secp256r1" | "p-256" => {
        let secret = P256SecretKey::random(&mut rng);
        let public = secret.public_key();
        self.private_key = secret.to_bytes().to_vec();
        let uncompressed = format.as_deref() == Some("uncompressed") || format.is_none();
        self.public_key = public.to_sec1_point(!uncompressed).as_bytes().to_vec();
      }
      "secp384r1" | "p-384" => {
        let secret = P384SecretKey::random(&mut rng);
        let public = secret.public_key();
        self.private_key = secret.to_bytes().to_vec();
        let uncompressed = format.as_deref() == Some("uncompressed") || format.is_none();
        self.public_key = public.to_sec1_point(!uncompressed).as_bytes().to_vec();
      }
      other => {
        return Err(Error::new(Status::InvalidArg, format!("Unsupported curve for key generation: {}", other)));
      }
    }

    self.get_public_key(output_encoding, format)
  }

  #[napi]
  pub fn compute_secret(
    &self,
    other_public_key: Either<Buffer, String>,
    _input_encoding: Option<String>,
    output_encoding: Option<String>,
  ) -> Result<Either<Buffer, String>> {
    let pub_bytes = match other_public_key {
      Either::A(b) => b.to_vec(),
      Either::B(s) => hex::decode(&s).map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?,
    };

    let secret_bytes = match self.curve.as_str() {
      "prime256v1" | "secp256r1" | "p-256" => {
        let sk = P256SecretKey::from_slice(&self.private_key)
          .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;
        let pk = P256PublicKey::from_sec1_bytes(&pub_bytes)
          .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;
        let shared = diffie_hellman(sk.to_nonzero_scalar(), pk.as_affine());
        shared.raw_secret_bytes().to_vec()
      }
      "secp384r1" | "p-384" => {
        let sk = P384SecretKey::from_slice(&self.private_key)
          .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;
        let pk = P384PublicKey::from_sec1_bytes(&pub_bytes)
          .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;
        let shared = diffie_hellman(sk.to_nonzero_scalar(), pk.as_affine());
        shared.raw_secret_bytes().to_vec()
      }
      other => {
        return Err(Error::new(Status::InvalidArg, format!("Unsupported curve for secret computation: {}", other)));
      }
    };

    match output_encoding.as_deref() {
      Some("hex") => Ok(Either::B(hex::encode(&secret_bytes))),
      Some("base64") => Ok(Either::B(crate::crypto_hasher::base64_encode(&secret_bytes))),
      _ => Ok(Either::A(Buffer::from(secret_bytes))),
    }
  }

  #[napi]
  pub fn get_private_key(&self, encoding: Option<String>) -> Result<Either<Buffer, String>> {
    match encoding.as_deref() {
      Some("hex") => Ok(Either::B(hex::encode(&self.private_key))),
      Some("base64") => Ok(Either::B(crate::crypto_hasher::base64_encode(&self.private_key))),
      _ => Ok(Either::A(Buffer::from(self.private_key.clone()))),
    }
  }

  #[napi]
  pub fn set_private_key(&mut self, private_key: Either<Buffer, String>, _encoding: Option<String>) -> Result<()> {
    let bytes = match private_key {
      Either::A(b) => b.to_vec(),
      Either::B(s) => hex::decode(&s).map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?,
    };
    self.private_key = bytes;
    Ok(())
  }

  #[napi]
  pub fn get_public_key(&self, encoding: Option<String>, _format: Option<String>) -> Result<Either<Buffer, String>> {
    match encoding.as_deref() {
      Some("hex") => Ok(Either::B(hex::encode(&self.public_key))),
      Some("base64") => Ok(Either::B(crate::crypto_hasher::base64_encode(&self.public_key))),
      _ => Ok(Either::A(Buffer::from(self.public_key.clone()))),
    }
  }

  #[napi]
  pub fn set_public_key(&mut self, public_key: Either<Buffer, String>, _encoding: Option<String>) -> Result<()> {
    let bytes = match public_key {
      Either::A(b) => b.to_vec(),
      Either::B(s) => hex::decode(&s).map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?,
    };
    self.public_key = bytes;
    Ok(())
  }
}

#[napi]
pub fn create_ecdh(curve_name: String) -> Result<ECDH> {
  ECDH::new(curve_name)
}

#[napi]
pub fn create_diffie_hellman(
  prime: Either<Buffer, u32>,
  _generator: Option<Either<Buffer, u32>>,
) -> Result<ECDH> {
  let curve_name = match prime {
    Either::A(_) => "prime256v1".to_string(),
    Either::B(_) => "prime256v1".to_string(),
  };
  ECDH::new(curve_name)
}

#[napi]
pub fn create_diffie_hellman_group(group_name: String) -> Result<ECDH> {
  let curve = match group_name.to_lowercase().as_str() {
    "modp14" | "modp15" => "secp384r1",
    _ => "prime256v1",
  };
  ECDH::new(curve.to_string())
}
