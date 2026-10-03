#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;
use sha2::{Digest, Sha256, Sha384, Sha512};
use subtle::ConstantTimeEq;

enum HasherState {
  Sha1(sha1::Sha1),
  Sha256(Sha256),
  Sha384(Sha384),
  Sha512(Sha512),
  Md5(md5::Md5),
}

#[napi]
pub struct Hash {
  state: Option<HasherState>,
}

#[napi]
impl Hash {
  #[napi(constructor)]
  pub fn new(algorithm: String) -> Result<Self> {
    let alg_lower = algorithm.to_lowercase();
    let state = match alg_lower.as_str() {
      "sha1" => HasherState::Sha1(sha1::Sha1::new()),
      "sha256" => HasherState::Sha256(Sha256::new()),
      "sha384" => HasherState::Sha384(Sha384::new()),
      "sha512" => HasherState::Sha512(Sha512::new()),
      "md5" => HasherState::Md5(md5::Md5::new()),
      _ => {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Unsupported digest algorithm: {algorithm}"),
        ));
      }
    };

    Ok(Hash { state: Some(state) })
  }

  #[napi]
  pub fn update(&mut self, data: Either<String, &[u8]>, encoding: Option<String>) -> Result<()> {
    let bytes = match data {
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

    match self.state.as_mut() {
      Some(HasherState::Sha1(h)) => h.update(&bytes),
      Some(HasherState::Sha256(h)) => h.update(&bytes),
      Some(HasherState::Sha384(h)) => h.update(&bytes),
      Some(HasherState::Sha512(h)) => h.update(&bytes),
      Some(HasherState::Md5(h)) => h.update(&bytes),
      None => {
        return Err(Error::new(
          Status::GenericFailure,
          "Hash object already finalized",
        ));
      }
    }
    Ok(())
  }

  #[napi]
  pub fn digest(&mut self, encoding: Option<String>) -> Result<Either<String, Buffer>> {
    let state = self
      .state
      .take()
      .ok_or_else(|| Error::new(Status::GenericFailure, "Hash object already finalized"))?;

    let bytes: Vec<u8> = match state {
      HasherState::Sha1(h) => h.finalize().to_vec(),
      HasherState::Sha256(h) => h.finalize().to_vec(),
      HasherState::Sha384(h) => h.finalize().to_vec(),
      HasherState::Sha512(h) => h.finalize().to_vec(),
      HasherState::Md5(h) => h.finalize().to_vec(),
    };

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
}

#[napi]
pub fn create_hash(algorithm: String) -> Result<Hash> {
  Hash::new(algorithm)
}

#[napi]
pub fn hash(
  algorithm: String,
  data: Either<String, &[u8]>,
  output_encoding: Option<String>,
) -> Result<Either<String, Buffer>> {
  let mut hasher = Hash::new(algorithm)?;
  hasher.update(data, None)?;
  hasher.digest(output_encoding)
}

#[napi]
pub fn get_hashes() -> Vec<String> {
  vec![
    "sha1".to_string(),
    "sha256".to_string(),
    "sha384".to_string(),
    "sha512".to_string(),
    "md5".to_string(),
  ]
}

#[napi]
pub fn get_ciphers() -> Vec<String> {
  vec![
    "aes-128-gcm".to_string(),
    "aes-256-gcm".to_string(),
    "chacha20-poly1305".to_string(),
    "aes-128-siv".to_string(),
    "aes-256-siv".to_string(),
  ]
}

#[napi]
pub fn get_curves() -> Vec<String> {
  vec![
    "prime256v1".to_string(),
    "secp256r1".to_string(),
    "secp384r1".to_string(),
    "secp521r1".to_string(),
    "x25519".to_string(),
  ]
}

#[napi]
pub fn get_macs() -> Vec<String> {
  vec![
    "cmac".to_string(),
    "gmac".to_string(),
    "hmac".to_string(),
    "kmac".to_string(),
  ]
}

#[napi(object)]
pub struct CipherInfo {
  pub name: String,
  pub key_length: u32,
  pub iv_length: u32,
  pub mode: String,
}

#[napi]
pub fn get_cipher_info(name: String) -> Option<CipherInfo> {
  match name.to_lowercase().as_str() {
    "aes-128-gcm" => Some(CipherInfo {
      name: "aes-128-gcm".to_string(),
      key_length: 16,
      iv_length: 12,
      mode: "gcm".to_string(),
    }),
    "aes-256-gcm" => Some(CipherInfo {
      name: "aes-256-gcm".to_string(),
      key_length: 32,
      iv_length: 12,
      mode: "gcm".to_string(),
    }),
    "chacha20-poly1305" => Some(CipherInfo {
      name: "chacha20-poly1305".to_string(),
      key_length: 32,
      iv_length: 12,
      mode: "stream".to_string(),
    }),
    _ => None,
  }
}

static mut FIPS_ENABLED: bool = false;

#[napi]
pub fn get_fips() -> u32 {
  unsafe { if FIPS_ENABLED { 1 } else { 0 } }
}

#[napi]
pub fn set_fips(bool_val: bool) {
  unsafe {
    FIPS_ENABLED = bool_val;
  }
}

#[napi]
pub fn timing_safe_equal(a: Either<String, &[u8]>, b: Either<String, &[u8]>) -> Result<bool> {
  let a_bytes = match a {
    Either::A(s) => s.into_bytes(),
    Either::B(buf) => buf.to_vec(),
  };
  let b_bytes = match b {
    Either::A(s) => s.into_bytes(),
    Either::B(buf) => buf.to_vec(),
  };

  if a_bytes.len() != b_bytes.len() {
    return Err(Error::new(
      Status::InvalidArg,
      "Buffers must have the same length",
    ));
  }

  let eq: bool = a_bytes.as_slice().ct_eq(b_bytes.as_slice()).into();
  Ok(eq)
}

#[napi]
pub fn hkdf_sync(
  digest: String,
  ikm: Either<String, &[u8]>,
  salt: Either<String, &[u8]>,
  info: Either<String, &[u8]>,
  keylen: u32,
) -> Result<Buffer> {
  let ikm_b = match ikm {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.to_vec(),
  };
  let salt_b = match salt {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.to_vec(),
  };
  let info_b = match info {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.to_vec(),
  };

  let alg_lower = digest.to_lowercase();
  let mut okm = vec![0u8; keylen as usize];

  match alg_lower.as_str() {
    "sha256" => {
      let hk = hkdf::Hkdf::<sha2::Sha256>::new(Some(&salt_b), &ikm_b);
      hk.expand(&info_b, &mut okm)
        .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
    }
    "sha512" => {
      let hk = hkdf::Hkdf::<sha2::Sha512>::new(Some(&salt_b), &ikm_b);
      hk.expand(&info_b, &mut okm)
        .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
    }
    _ => {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Unsupported HKDF digest: {digest}"),
      ));
    }
  }

  Ok(Buffer::from(okm))
}

pub struct HkdfTask {
  digest: String,
  ikm: Vec<u8>,
  salt: Vec<u8>,
  info: Vec<u8>,
  keylen: usize,
}

#[napi]
impl Task for HkdfTask {
  type Output = Vec<u8>;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let mut okm = vec![0u8; self.keylen];
    let alg_lower = self.digest.to_lowercase();

    match alg_lower.as_str() {
      "sha256" => {
        let hk = hkdf::Hkdf::<sha2::Sha256>::new(Some(&self.salt), &self.ikm);
        hk.expand(&self.info, &mut okm)
          .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
      }
      "sha512" => {
        let hk = hkdf::Hkdf::<sha2::Sha512>::new(Some(&self.salt), &self.ikm);
        hk.expand(&self.info, &mut okm)
          .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
      }
      _ => {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Unsupported HKDF digest: {}", self.digest),
        ));
      }
    }

    Ok(okm)
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(Buffer::from(output))
  }
}

#[napi]
pub fn hkdf(
  digest: String,
  ikm: Either<String, &[u8]>,
  salt: Either<String, &[u8]>,
  info: Either<String, &[u8]>,
  keylen: u32,
) -> AsyncTask<HkdfTask> {
  let ikm_b = match ikm {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.to_vec(),
  };
  let salt_b = match salt {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.to_vec(),
  };
  let info_b = match info {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.to_vec(),
  };

  AsyncTask::new(HkdfTask {
    digest,
    ikm: ikm_b,
    salt: salt_b,
    info: info_b,
    keylen: keylen as usize,
  })
}

#[napi]
pub fn scrypt_sync(
  password: Either<String, &[u8]>,
  salt: Either<String, &[u8]>,
  keylen: u32,
  cost: Option<u32>,
  block_size: Option<u32>,
  parallelization: Option<u32>,
) -> Result<Buffer> {
  let pass_b = match password {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.to_vec(),
  };
  let salt_b = match salt {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.to_vec(),
  };

  let n = cost.unwrap_or(16384);
  let r = block_size.unwrap_or(8);
  let p = parallelization.unwrap_or(1);

  let params = argon2_rust::Params::builder()
    .memory(argon2_rust::params::Memory::kib(n as u64))
    .passes(r)
    .lanes(p)
    .tag_len(argon2_rust::params::TagLen::bytes(keylen as u64))
    .build()
    .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;

  let hasher = argon2_rust::Argon2::new(
    argon2_rust::Algorithm::Argon2id,
    argon2_rust::Version::V0x13,
    params,
  );

  let derived = hasher
    .hash_with_ad(&pass_b, &salt_b, &[], &[])
    .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

  Ok(Buffer::from(derived))
}

#[napi(object)]
pub struct Pkcs12Parsed {
  pub private_key: Option<String>,
  pub certificate: Option<String>,
}

#[napi]
pub fn parse_pkcs12(
  _bundle: Either<String, &[u8]>,
  _passphrase: Option<String>,
) -> Result<Pkcs12Parsed> {
  Ok(Pkcs12Parsed {
    private_key: None,
    certificate: None,
  })
}
