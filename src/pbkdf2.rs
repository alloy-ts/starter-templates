#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;

fn to_bytes(input: Either<String, Buffer>) -> Vec<u8> {
  match input {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.to_vec(),
  }
}

fn parse_pbkdf2_digest(digest: &str) -> Result<ring::pbkdf2::Algorithm> {
  match digest.to_lowercase().replace("-", "").as_str() {
    "sha1" => Ok(ring::pbkdf2::PBKDF2_HMAC_SHA1),
    "sha256" => Ok(ring::pbkdf2::PBKDF2_HMAC_SHA256),
    "sha384" => Ok(ring::pbkdf2::PBKDF2_HMAC_SHA384),
    "sha512" => Ok(ring::pbkdf2::PBKDF2_HMAC_SHA512),
    _ => Err(Error::new(
      Status::InvalidArg,
      format!("Unsupported PBKDF2 digest: {}", digest),
    )),
  }
}

fn compute_pbkdf2(
  password: Either<String, Buffer>,
  salt: Either<String, Buffer>,
  iterations: u32,
  keylen: u32,
  digest: &str,
) -> Result<Vec<u8>> {
  let alg = parse_pbkdf2_digest(digest)?;
  let pass_bytes = to_bytes(password);
  let salt_bytes = to_bytes(salt);

  let num_iters = std::num::NonZeroU32::new(iterations).ok_or_else(|| {
    Error::new(Status::InvalidArg, "iterations must be greater than 0")
  })?;

  let mut out = vec![0u8; keylen as usize];
  ring::pbkdf2::derive(alg, num_iters, &salt_bytes, &pass_bytes, &mut out);
  Ok(out)
}

#[napi(js_name = "pbkdf2Sync")]
pub fn pbkdf2_sync(
  password: Either<String, Buffer>,
  salt: Either<String, Buffer>,
  iterations: u32,
  keylen: u32,
  digest: String,
) -> Result<Buffer> {
  let res = compute_pbkdf2(password, salt, iterations, keylen, &digest)?;
  Ok(res.into())
}

pub struct Pbkdf2Task {
  password: Either<String, Buffer>,
  salt: Either<String, Buffer>,
  iterations: u32,
  keylen: u32,
  digest: String,
}

#[napi]
impl Task for Pbkdf2Task {
  type Output = Vec<u8>;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let pass = std::mem::replace(&mut self.password, Either::A(String::new()));
    let salt = std::mem::replace(&mut self.salt, Either::A(String::new()));
    compute_pbkdf2(pass, salt, self.iterations, self.keylen, &self.digest)
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output.into())
  }
}

#[napi(js_name = "pbkdf2")]
pub fn pbkdf2_async(
  password: Either<String, Buffer>,
  salt: Either<String, Buffer>,
  iterations: u32,
  keylen: u32,
  digest: String,
  signal: Option<AbortSignal>,
) -> AsyncTask<Pbkdf2Task> {
  AsyncTask::with_optional_signal(
    Pbkdf2Task {
      password,
      salt,
      iterations,
      keylen,
      digest,
    },
    signal,
  )
}

fn compute_hkdf(
  digest: &str,
  ikm: Either<String, Buffer>,
  salt: Either<String, Buffer>,
  info: Either<String, Buffer>,
  keylen: u32,
) -> Result<Vec<u8>> {
  let ikm_bytes = to_bytes(ikm);
  let salt_bytes = to_bytes(salt);
  let info_bytes = to_bytes(info);

  let mut okm = vec![0u8; keylen as usize];

  match digest.to_lowercase().replace("-", "").as_str() {
    "sha256" => {
      let hk = hkdf::Hkdf::<sha2::Sha256>::new(Some(&salt_bytes), &ikm_bytes);
      hk.expand(&info_bytes, &mut okm)
        .map_err(|e| Error::new(Status::GenericFailure, format!("{:?}", e)))?;
    }
    "sha512" => {
      let hk = hkdf::Hkdf::<sha2::Sha512>::new(Some(&salt_bytes), &ikm_bytes);
      hk.expand(&info_bytes, &mut okm)
        .map_err(|e| Error::new(Status::GenericFailure, format!("{:?}", e)))?;
    }
    "sha1" => {
      let hk = hkdf::Hkdf::<sha1::Sha1>::new(Some(&salt_bytes), &ikm_bytes);
      hk.expand(&info_bytes, &mut okm)
        .map_err(|e| Error::new(Status::GenericFailure, format!("{:?}", e)))?;
    }
    _ => {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Unsupported HKDF digest: {}", digest),
      ));
    }
  }

  Ok(okm)
}

#[napi(js_name = "hkdfSync")]
pub fn hkdf_sync(
  digest: String,
  ikm: Either<String, Buffer>,
  salt: Either<String, Buffer>,
  info: Either<String, Buffer>,
  keylen: u32,
) -> Result<Buffer> {
  let res = compute_hkdf(&digest, ikm, salt, info, keylen)?;
  Ok(res.into())
}

pub struct HkdfTask {
  digest: String,
  ikm: Either<String, Buffer>,
  salt: Either<String, Buffer>,
  info: Either<String, Buffer>,
  keylen: u32,
}

#[napi]
impl Task for HkdfTask {
  type Output = Vec<u8>;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let ikm = std::mem::replace(&mut self.ikm, Either::A(String::new()));
    let salt = std::mem::replace(&mut self.salt, Either::A(String::new()));
    let info = std::mem::replace(&mut self.info, Either::A(String::new()));
    compute_hkdf(&self.digest, ikm, salt, info, self.keylen)
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output.into())
  }
}

#[napi(js_name = "hkdf")]
pub fn hkdf_async(
  digest: String,
  ikm: Either<String, Buffer>,
  salt: Either<String, Buffer>,
  info: Either<String, Buffer>,
  keylen: u32,
  signal: Option<AbortSignal>,
) -> AsyncTask<HkdfTask> {
  AsyncTask::with_optional_signal(
    HkdfTask {
      digest,
      ikm,
      salt,
      info,
      keylen,
    },
    signal,
  )
}

#[napi(object)]
pub struct ScryptOptions {
  pub cost: Option<u32>,
  pub block_size: Option<u32>,
  pub parallelization: Option<u32>,
  pub n: Option<u32>,
  pub r: Option<u32>,
  pub p: Option<u32>,
  pub maxmem: Option<u32>,
}

fn compute_scrypt(
  password: Either<String, Buffer>,
  salt: Either<String, Buffer>,
  keylen: u32,
  options: Option<ScryptOptions>,
) -> Result<Vec<u8>> {
  let pass_bytes = to_bytes(password);
  let salt_bytes = to_bytes(salt);
  let opts = options.unwrap_or(ScryptOptions {
    cost: None,
    block_size: None,
    parallelization: None,
    n: None,
    r: None,
    p: None,
    maxmem: None,
  });

  let n = opts.n.or(opts.cost).unwrap_or(16384);
  let r = opts.r.or(opts.block_size).unwrap_or(8);
  let p = opts.p.or(opts.parallelization).unwrap_or(1);

  let log_n = (n as f64).log2() as u8;

  let params = scrypt::Params::new(log_n, r, p)
    .map_err(|e| Error::new(Status::InvalidArg, format!("{:?}", e)))?;

  let mut output = vec![0u8; keylen as usize];
  scrypt::scrypt(&pass_bytes, &salt_bytes, &params, &mut output)
    .map_err(|e| Error::new(Status::GenericFailure, format!("{:?}", e)))?;

  Ok(output)
}

#[napi(js_name = "scryptSync")]
pub fn scrypt_sync(
  password: Either<String, Buffer>,
  salt: Either<String, Buffer>,
  keylen: u32,
  options: Option<ScryptOptions>,
) -> Result<Buffer> {
  let res = compute_scrypt(password, salt, keylen, options)?;
  Ok(res.into())
}

pub struct ScryptTask {
  password: Either<String, Buffer>,
  salt: Either<String, Buffer>,
  keylen: u32,
  options: Option<ScryptOptions>,
}

#[napi]
impl Task for ScryptTask {
  type Output = Vec<u8>;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let pass = std::mem::replace(&mut self.password, Either::A(String::new()));
    let salt = std::mem::replace(&mut self.salt, Either::A(String::new()));
    compute_scrypt(pass, salt, self.keylen, self.options.take())
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output.into())
  }
}

#[napi(js_name = "scrypt")]
pub fn scrypt_async(
  password: Either<String, Buffer>,
  salt: Either<String, Buffer>,
  keylen: u32,
  options: Option<ScryptOptions>,
  signal: Option<AbortSignal>,
) -> AsyncTask<ScryptTask> {
  AsyncTask::with_optional_signal(
    ScryptTask {
      password,
      salt,
      keylen,
      options,
    },
    signal,
  )
}
