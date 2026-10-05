use napi::bindgen_prelude::*;
use napi_derive::napi;
use ring::signature as ring_sig;

#[napi]
pub struct Sign {
  pub algorithm: String,
  data: Vec<u8>,
}

#[napi]
impl Sign {
  #[napi(constructor)]
  pub fn new(algorithm: String) -> Self {
    Sign {
      algorithm,
      data: Vec::new(),
    }
  }

  #[napi]
  pub fn update(&mut self, data: Buffer) {
    self.data.extend_from_slice(data.as_ref());
  }

  #[napi]
  pub fn sign(&self, private_key: Buffer) -> Result<Buffer> {
    let key_pair = ring_sig::Ed25519KeyPair::from_pkcs8(private_key.as_ref())
      .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid private key: {}", e)))?;

    let sig = key_pair.sign(&self.data);
    Ok(Buffer::from(sig.as_ref()))
  }
}

#[napi]
pub struct Verify {
  pub algorithm: String,
  data: Vec<u8>,
}

#[napi]
impl Verify {
  #[napi(constructor)]
  pub fn new(algorithm: String) -> Self {
    Verify {
      algorithm,
      data: Vec::new(),
    }
  }

  #[napi]
  pub fn update(&mut self, data: Buffer) {
    self.data.extend_from_slice(data.as_ref());
  }

  #[napi]
  pub fn verify(&self, public_key: Buffer, signature: Buffer) -> bool {
    let peer_public_key = ring_sig::UnparsedPublicKey::new(&ring_sig::ED25519, public_key.as_ref());
    peer_public_key.verify(&self.data, signature.as_ref()).is_ok()
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
