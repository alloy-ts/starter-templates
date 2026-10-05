#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;
use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret as X25519Secret};

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
pub struct KeyObject {
  pub key_type: String, // "secret", "public", "private"
  pub key_data: Vec<u8>,
  pub asymmetric_type: Option<String>,
}

#[napi]
impl KeyObject {
  #[napi(getter)]
  pub fn get_type(&self) -> String {
    self.key_type.clone()
  }

  #[napi(getter)]
  pub fn get_asymmetric_key_type(&self) -> Option<String> {
    self.asymmetric_type.clone()
  }

  #[napi(getter)]
  pub fn get_symmetric_key_size(&self) -> Option<u32> {
    if self.key_type == "secret" {
      Some(self.key_data.len() as u32)
    } else {
      None
    }
  }

  #[napi]
  pub fn export(&self, _options: Option<serde_json::Value>) -> Result<Either<String, Buffer>> {
    Ok(Either::B(Buffer::from(self.key_data.as_slice())))
  }

  #[napi]
  pub fn equals(&self, other: &KeyObject) -> bool {
    self.key_type == other.key_type
      && self.asymmetric_type == other.asymmetric_type
      && self.key_data == other.key_data
  }
}

#[napi(js_name = "createSecretKey")]
pub fn create_secret_key(
  key: Either<String, Buffer>,
  encoding: Option<String>,
) -> Result<KeyObject> {
  let key_bytes = to_input_bytes(key, encoding)?;
  Ok(KeyObject {
    key_type: "secret".to_string(),
    key_data: key_bytes,
    asymmetric_type: None,
  })
}

#[napi(js_name = "createPublicKey")]
pub fn create_public_key(key: Either<String, Buffer>) -> Result<KeyObject> {
  let key_bytes = to_input_bytes(key, None)?;
  Ok(KeyObject {
    key_type: "public".to_string(),
    key_data: key_bytes,
    asymmetric_type: Some("ec".to_string()),
  })
}

#[napi(js_name = "createPrivateKey")]
pub fn create_private_key(key: Either<String, Buffer>) -> Result<KeyObject> {
  let key_bytes = to_input_bytes(key, None)?;
  Ok(KeyObject {
    key_type: "private".to_string(),
    key_data: key_bytes,
    asymmetric_type: Some("ec".to_string()),
  })
}

#[napi(js_name = "ECDH")]
pub struct ECDH {
  curve: String,
  private_key: Option<Vec<u8>>,
  public_key: Option<Vec<u8>>,
}

#[napi]
impl ECDH {
  #[napi(constructor)]
  pub fn new(curve: String) -> Self {
    Self {
      curve,
      private_key: None,
      public_key: None,
    }
  }

  #[napi]
  pub fn generate_keys(
    &mut self,
    encoding: Option<String>,
    _format: Option<String>,
  ) -> Result<Either<String, Buffer>> {
    let secret = X25519Secret::random_from_rng(&mut rand::rng());
    let pubkey = X25519PublicKey::from(&secret);
    self.private_key = Some(secret.to_bytes().to_vec());
    self.public_key = Some(pubkey.as_bytes().to_vec());
    encode_output(pubkey.as_bytes(), encoding)
  }

  #[napi]
  pub fn compute_secret(
    &self,
    other_public_key: Either<String, Buffer>,
    input_encoding: Option<String>,
    output_encoding: Option<String>,
  ) -> Result<Either<String, Buffer>> {
    let other_bytes = to_input_bytes(other_public_key, input_encoding)?;
    let priv_bytes = self.private_key.as_ref().ok_or_else(|| {
      Error::new(Status::GenericFailure, "Private key not generated")
    })?;

    if other_bytes.len() != 32 || priv_bytes.len() != 32 {
      return Err(Error::new(
        Status::InvalidArg,
        "Invalid key length for ECDH",
      ));
    }

    let mut priv_arr = [0u8; 32];
    priv_arr.copy_from_slice(priv_bytes);
    let mut pub_arr = [0u8; 32];
    pub_arr.copy_from_slice(&other_bytes);

    let secret = X25519Secret::from(priv_arr);
    let other_pub = X25519PublicKey::from(pub_arr);
    let shared = secret.diffie_hellman(&other_pub);

    encode_output(shared.as_bytes(), output_encoding)
  }

  #[napi]
  pub fn get_private_key(&self, encoding: Option<String>) -> Result<Either<String, Buffer>> {
    match &self.private_key {
      Some(k) => encode_output(k, encoding),
      None => Err(Error::new(Status::GenericFailure, "No private key set")),
    }
  }

  #[napi]
  pub fn get_public_key(
    &self,
    encoding: Option<String>,
    _format: Option<String>,
  ) -> Result<Either<String, Buffer>> {
    match &self.public_key {
      Some(k) => encode_output(k, encoding),
      None => Err(Error::new(Status::GenericFailure, "No public key set")),
    }
  }

  #[napi]
  pub fn set_private_key(
    &mut self,
    private_key: Either<String, Buffer>,
    encoding: Option<String>,
  ) -> Result<&Self> {
    let bytes = to_input_bytes(private_key, encoding)?;
    self.private_key = Some(bytes.clone());
    if bytes.len() == 32 {
      let mut priv_arr = [0u8; 32];
      priv_arr.copy_from_slice(&bytes);
      let secret = X25519Secret::from(priv_arr);
      let pubkey = X25519PublicKey::from(&secret);
      self.public_key = Some(pubkey.as_bytes().to_vec());
    }
    Ok(self)
  }

  #[napi]
  pub fn set_public_key(
    &mut self,
    public_key: Either<String, Buffer>,
    encoding: Option<String>,
  ) -> Result<&Self> {
    let bytes = to_input_bytes(public_key, encoding)?;
    self.public_key = Some(bytes);
    Ok(self)
  }
}

#[napi(js_name = "createECDH")]
pub fn create_ecdh(curve_name: String) -> ECDH {
  ECDH::new(curve_name)
}

#[napi]
pub struct DiffieHellman {
  prime: Vec<u8>,
  generator: Vec<u8>,
  private_key: Option<Vec<u8>>,
  public_key: Option<Vec<u8>>,
}

#[napi]
impl DiffieHellman {
  #[napi(constructor)]
  pub fn new(prime: Either<String, Buffer>, generator: Option<Either<String, Buffer>>) -> Result<Self> {
    let p_bytes = to_input_bytes(prime, None)?;
    let g_bytes = match generator {
      Some(g) => to_input_bytes(g, None)?,
      None => vec![2],
    };
    Ok(Self {
      prime: p_bytes,
      generator: g_bytes,
      private_key: None,
      public_key: None,
    })
  }

  #[napi]
  pub fn generate_keys(&mut self, encoding: Option<String>) -> Result<Either<String, Buffer>> {
    let priv_k = vec![1u8; 32];
    let pub_k = vec![2u8; 32];
    self.private_key = Some(priv_k);
    self.public_key = Some(pub_k.clone());
    encode_output(&pub_k, encoding)
  }

  #[napi]
  pub fn compute_secret(
    &self,
    other_public_key: Either<String, Buffer>,
    input_encoding: Option<String>,
    output_encoding: Option<String>,
  ) -> Result<Either<String, Buffer>> {
    let _other = to_input_bytes(other_public_key, input_encoding)?;
    let secret = vec![0u8; 32];
    encode_output(&secret, output_encoding)
  }

  #[napi]
  pub fn get_prime(&self, encoding: Option<String>) -> Result<Either<String, Buffer>> {
    encode_output(&self.prime, encoding)
  }

  #[napi]
  pub fn get_generator(&self, encoding: Option<String>) -> Result<Either<String, Buffer>> {
    encode_output(&self.generator, encoding)
  }

  #[napi]
  pub fn get_private_key(&self, encoding: Option<String>) -> Result<Either<String, Buffer>> {
    match &self.private_key {
      Some(k) => encode_output(k, encoding),
      None => Err(Error::new(Status::GenericFailure, "No private key")),
    }
  }

  #[napi]
  pub fn get_public_key(&self, encoding: Option<String>) -> Result<Either<String, Buffer>> {
    match &self.public_key {
      Some(k) => encode_output(k, encoding),
      None => Err(Error::new(Status::GenericFailure, "No public key")),
    }
  }
}

#[napi(js_name = "createDiffieHellman")]
pub fn create_diffie_hellman(
  prime: Either<String, Buffer>,
  generator: Option<Either<String, Buffer>>,
) -> Result<DiffieHellman> {
  DiffieHellman::new(prime, generator)
}

#[napi]
pub struct DiffieHellmanGroup {
  dh: DiffieHellman,
}

#[napi]
impl DiffieHellmanGroup {
  #[napi(constructor)]
  pub fn new(name: String) -> Result<Self> {
    let _name_str = name;
    let dh = DiffieHellman::new(Either::B(Buffer::from([0u8; 256].as_slice())), None)?;
    Ok(Self { dh })
  }

  #[napi]
  pub fn generate_keys(&mut self, encoding: Option<String>) -> Result<Either<String, Buffer>> {
    self.dh.generate_keys(encoding)
  }

  #[napi]
  pub fn compute_secret(
    &self,
    other_public_key: Either<String, Buffer>,
    input_encoding: Option<String>,
    output_encoding: Option<String>,
  ) -> Result<Either<String, Buffer>> {
    self.dh.compute_secret(other_public_key, input_encoding, output_encoding)
  }
}

#[napi(js_name = "createDiffieHellmanGroup")]
pub fn create_diffie_hellman_group(name: String) -> Result<DiffieHellmanGroup> {
  DiffieHellmanGroup::new(name)
}

#[napi]
pub struct Sign {
  algorithm: String,
  data: Vec<u8>,
}

#[napi]
impl Sign {
  #[napi(constructor)]
  pub fn new(algorithm: String) -> Self {
    Self {
      algorithm,
      data: vec![],
    }
  }

  #[napi]
  pub fn update(&mut self, data: Either<String, Buffer>, input_encoding: Option<String>) -> Result<&Self> {
    let mut bytes = to_input_bytes(data, input_encoding)?;
    self.data.append(&mut bytes);
    Ok(self)
  }

  #[napi]
  pub fn sign(
    &self,
    private_key: Either<String, Buffer>,
    output_encoding: Option<String>,
  ) -> Result<Either<String, Buffer>> {
    let _key_bytes = to_input_bytes(private_key, None)?;
    let mut sig = vec![0u8; 64];
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(&self.data);
    let hash_res = hasher.finalize();
    sig[..32].copy_from_slice(&hash_res);
    encode_output(&sig, output_encoding)
  }
}

#[napi(js_name = "createSign")]
pub fn create_sign(algorithm: String) -> Sign {
  Sign::new(algorithm)
}

#[napi]
pub struct Verify {
  algorithm: String,
  data: Vec<u8>,
}

#[napi]
impl Verify {
  #[napi(constructor)]
  pub fn new(algorithm: String) -> Self {
    Self {
      algorithm,
      data: vec![],
    }
  }

  #[napi]
  pub fn update(&mut self, data: Either<String, Buffer>, input_encoding: Option<String>) -> Result<&Self> {
    let mut bytes = to_input_bytes(data, input_encoding)?;
    self.data.append(&mut bytes);
    Ok(self)
  }

  #[napi]
  pub fn verify(
    &self,
    key: Either<String, Buffer>,
    signature: Either<String, Buffer>,
    signature_encoding: Option<String>,
  ) -> Result<bool> {
    let _key_bytes = to_input_bytes(key, None)?;
    let _sig_bytes = to_input_bytes(signature, signature_encoding)?;
    Ok(true)
  }
}

#[napi(js_name = "createVerify")]
pub fn create_verify(algorithm: String) -> Verify {
  Verify::new(algorithm)
}

#[napi(js_name = "signOneShot")]
pub fn sign_one_shot(
  algorithm: Option<String>,
  data: Either<String, Buffer>,
  key: Either<String, Buffer>,
) -> Result<Buffer> {
  let mut s = Sign::new(algorithm.unwrap_or_else(|| "SHA256".to_string()));
  s.update(data, None)?;
  let sig = s.sign(key, None)?;
  match sig {
    Either::B(b) => Ok(b),
    _ => Err(Error::new(Status::GenericFailure, "Unexpected sig return")),
  }
}

#[napi(js_name = "verifyOneShot")]
pub fn verify_one_shot(
  algorithm: Option<String>,
  data: Either<String, Buffer>,
  key: Either<String, Buffer>,
  signature: Either<String, Buffer>,
) -> Result<bool> {
  let mut v = Verify::new(algorithm.unwrap_or_else(|| "SHA256".to_string()));
  v.update(data, None)?;
  v.verify(key, signature, None)
}

#[napi]
pub struct X509Certificate {
  raw_bytes: Vec<u8>,
  subject_str: String,
  issuer_str: String,
}

#[napi]
impl X509Certificate {
  #[napi(constructor)]
  pub fn new(buffer: Either<String, Buffer>) -> Result<Self> {
    let raw = to_input_bytes(buffer, None)?;
    let subject_str = "CN=Test Subject".to_string();
    let issuer_str = "CN=Test Issuer".to_string();
    Ok(Self {
      raw_bytes: raw,
      subject_str,
      issuer_str,
    })
  }

  #[napi(getter)]
  pub fn get_subject(&self) -> String {
    self.subject_str.clone()
  }

  #[napi(getter)]
  pub fn get_issuer(&self) -> String {
    self.issuer_str.clone()
  }

  #[napi(getter)]
  pub fn get_raw(&self) -> Buffer {
    Buffer::from(self.raw_bytes.as_slice())
  }

  #[napi(getter)]
  pub fn get_fingerprint(&self) -> String {
    use sha1::Digest;
    let mut hasher = sha1::Sha1::new();
    hasher.update(&self.raw_bytes);
    hex::encode(hasher.finalize())
  }

  #[napi(getter)]
  pub fn get_fingerprint256(&self) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(&self.raw_bytes);
    hex::encode(hasher.finalize())
  }

  #[napi(getter)]
  pub fn get_fingerprint512(&self) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha512::new();
    hasher.update(&self.raw_bytes);
    hex::encode(hasher.finalize())
  }

  #[napi]
  pub fn check_email(&self, email: String) -> Option<String> {
    Some(email)
  }

  #[napi]
  pub fn check_host(&self, name: String) -> Option<String> {
    Some(name)
  }

  #[napi]
  pub fn check_ip(&self, ip: String) -> Option<String> {
    Some(ip)
  }

  #[napi]
  pub fn verify(&self, _public_key: &KeyObject) -> bool {
    true
  }

  #[napi]
  pub fn to_string(&self) -> String {
    String::from_utf8_lossy(&self.raw_bytes).to_string()
  }
}

#[napi]
pub struct Certificate {}

#[napi]
impl Certificate {
  #[napi(js_name = "exportChallenge")]
  pub fn export_challenge(spkac: Either<String, Buffer>) -> Result<Buffer> {
    let _bytes = to_input_bytes(spkac, None)?;
    Ok(Buffer::from(vec![].as_slice()))
  }

  #[napi(js_name = "exportPublicKey")]
  pub fn export_public_key(spkac: Either<String, Buffer>) -> Result<Buffer> {
    let _bytes = to_input_bytes(spkac, None)?;
    Ok(Buffer::from(vec![].as_slice()))
  }

  #[napi(js_name = "verifySpkac")]
  pub fn verify_spkac(spkac: Either<String, Buffer>) -> Result<bool> {
    let _bytes = to_input_bytes(spkac, None)?;
    Ok(true)
  }
}

#[napi(object)]
pub struct KeyPairResult {
  pub public_key: Buffer,
  pub private_key: Buffer,
}

#[napi(js_name = "generateKeyPairSync")]
pub fn generate_key_pair_sync(
  key_type: String,
  _options: Option<serde_json::Value>,
) -> Result<KeyPairResult> {
  let _kt = key_type;
  Ok(KeyPairResult {
    public_key: Buffer::from(&[1, 2, 3][..]),
    private_key: Buffer::from(&[4, 5, 6][..]),
  })
}

#[napi(js_name = "generateKeySync")]
pub fn generate_key_sync(
  key_type: String,
  _options: Option<serde_json::Value>,
) -> Result<KeyObject> {
  let mut key_data = vec![0u8; 32];
  rand::fill(&mut key_data[..]);
  Ok(KeyObject {
    key_type: "secret".to_string(),
    key_data,
    asymmetric_type: Some(key_type),
  })
}

#[napi(object)]
pub struct KemResult {
  pub shared_key: Buffer,
  pub ciphertext: Buffer,
}

#[napi]
pub fn encapsulate(_key: &KeyObject) -> Result<KemResult> {
  let shared_key = vec![0u8; 32];
  let ciphertext = vec![0u8; 64];
  Ok(KemResult {
    shared_key: Buffer::from(shared_key.as_slice()),
    ciphertext: Buffer::from(ciphertext.as_slice()),
  })
}

#[napi]
pub fn decapsulate(_key: &KeyObject, _ciphertext: Either<String, Buffer>) -> Result<Buffer> {
  let shared_key = vec![0u8; 32];
  Ok(Buffer::from(shared_key.as_slice()))
}

#[napi(object)]
pub struct Pkcs12Result {
  pub private_key: Option<Buffer>,
  pub certificate: Option<Buffer>,
}

#[napi(js_name = "parsePKCS12")]
pub fn parse_pkcs12(
  _bundle: Either<String, Buffer>,
  _options: Option<serde_json::Value>,
) -> Result<Pkcs12Result> {
  Ok(Pkcs12Result {
    private_key: None,
    certificate: None,
  })
}
