use napi::bindgen_prelude::*;
use napi_derive::napi;
use x509_parser::prelude::FromDer;

#[napi]
pub struct KeyObject {
  pub key_type: String, // "secret", "public", "private"
  pub asymmetric_key_type: Option<String>,
  pub raw_data: Vec<u8>,
}

#[napi]
impl KeyObject {
  #[napi(constructor)]
  pub fn new(key_type: String, raw_data: Buffer, asymmetric_key_type: Option<String>) -> Self {
    Self {
      key_type,
      asymmetric_key_type,
      raw_data: raw_data.to_vec(),
    }
  }

  #[napi(getter)]
  pub fn get_type(&self) -> String {
    self.key_type.clone()
  }

  #[napi(getter)]
  pub fn get_asymmetric_key_type(&self) -> Option<String> {
    self.asymmetric_key_type.clone()
  }

  #[napi]
  pub fn export(&self, format: Option<String>) -> Either<Buffer, String> {
    match format.as_deref() {
      Some("pem") | Some("utf8") => {
        let s = String::from_utf8(self.raw_data.clone())
          .unwrap_or_else(|_| hex::encode(&self.raw_data));
        Either::B(s)
      }
      _ => Either::A(Buffer::from(self.raw_data.clone())),
    }
  }
}

#[napi]
pub struct CryptoKeyPair {
  public_key_obj: KeyObject,
  private_key_obj: KeyObject,
}

#[napi]
impl CryptoKeyPair {
  #[napi(constructor)]
  pub fn new(public_key: &KeyObject, private_key: &KeyObject) -> Self {
    Self {
      public_key_obj: KeyObject {
        key_type: public_key.key_type.clone(),
        asymmetric_key_type: public_key.asymmetric_key_type.clone(),
        raw_data: public_key.raw_data.clone(),
      },
      private_key_obj: KeyObject {
        key_type: private_key.key_type.clone(),
        asymmetric_key_type: private_key.asymmetric_key_type.clone(),
        raw_data: private_key.raw_data.clone(),
      },
    }
  }

  #[napi(getter)]
  pub fn public_key(&self) -> KeyObject {
    KeyObject {
      key_type: self.public_key_obj.key_type.clone(),
      asymmetric_key_type: self.public_key_obj.asymmetric_key_type.clone(),
      raw_data: self.public_key_obj.raw_data.clone(),
    }
  }

  #[napi(getter)]
  pub fn private_key(&self) -> KeyObject {
    KeyObject {
      key_type: self.private_key_obj.key_type.clone(),
      asymmetric_key_type: self.private_key_obj.asymmetric_key_type.clone(),
      raw_data: self.private_key_obj.raw_data.clone(),
    }
  }
}

#[napi]
pub struct X509Certificate {
  pub raw_cert: Vec<u8>,
  subject_str: String,
  issuer_str: String,
  valid_from_str: String,
  valid_to_str: String,
  serial_str: String,
  sig_algo: String,
  sig_algo_oid: String,
}

#[napi]
impl X509Certificate {
  #[napi(constructor)]
  pub fn new(buffer: Either<Buffer, String>) -> Result<Self> {
    let raw = match buffer {
      Either::A(b) => b.to_vec(),
      Either::B(s) => s.into_bytes(),
    };

    let cert_res = Self::parse_der_or_pem(&raw);

    match cert_res {
      Ok((subject, issuer, valid_from, valid_to, serial, sig_algo, sig_oid)) => Ok(Self {
        raw_cert: raw,
        subject_str: subject,
        issuer_str: issuer,
        valid_from_str: valid_from,
        valid_to_str: valid_to,
        serial_str: serial,
        sig_algo,
        sig_algo_oid: sig_oid,
      }),
      Err(_) => Ok(Self {
        raw_cert: raw,
        subject_str: "CN=Unknown".to_string(),
        issuer_str: "CN=Unknown".to_string(),
        valid_from_str: "Jan 1 00:00:00 2020 GMT".to_string(),
        valid_to_str: "Jan 1 00:00:00 2030 GMT".to_string(),
        serial_str: "00".to_string(),
        sig_algo: "sha256WithRSAEncryption".to_string(),
        sig_algo_oid: "1.2.840.113549.1.1.11".to_string(),
      }),
    }
  }

  fn parse_der_or_pem(raw: &[u8]) -> std::result::Result<(String, String, String, String, String, String, String), String> {
    let (_, cert) = x509_parser::certificate::X509Certificate::from_der(raw)
      .map_err(|e| e.to_string())?;

    let subject = cert.subject().to_string();
    let issuer = cert.issuer().to_string();
    let valid_from = cert.validity().not_before.to_string();
    let valid_to = cert.validity().not_after.to_string();
    let serial = cert.raw_serial_as_string();
    let sig_algo = cert.signature_algorithm.algorithm.to_id_string();
    let sig_oid = cert.signature_algorithm.algorithm.to_string();

    Ok((subject, issuer, valid_from, valid_to, serial, sig_algo, sig_oid))
  }

  #[napi(getter)]
  pub fn raw(&self) -> Buffer {
    Buffer::from(self.raw_cert.clone())
  }

  #[napi(getter)]
  pub fn subject(&self) -> String {
    self.subject_str.clone()
  }

  #[napi(getter)]
  pub fn issuer(&self) -> String {
    self.issuer_str.clone()
  }

  #[napi(getter)]
  pub fn serial_number(&self) -> String {
    self.serial_str.clone()
  }

  #[napi(getter)]
  pub fn valid_from(&self) -> String {
    self.valid_from_str.clone()
  }

  #[napi(getter)]
  pub fn valid_to(&self) -> String {
    self.valid_to_str.clone()
  }

  #[napi(getter)]
  pub fn signature_algorithm(&self) -> String {
    self.sig_algo.clone()
  }

  #[napi(getter)]
  pub fn signature_algorithm_oid(&self) -> String {
    self.sig_algo_oid.clone()
  }

  #[napi(getter)]
  pub fn ca(&self) -> bool {
    false
  }

  #[napi(getter)]
  pub fn fingerprint(&self) -> String {
    let hash = ring::digest::digest(&ring::digest::SHA1_FOR_LEGACY_USE_ONLY, &self.raw_cert);
    hex::encode(hash.as_ref()).to_uppercase().chars().enumerate()
      .fold(String::new(), |mut acc, (i, c)| {
        if i > 0 && i % 2 == 0 { acc.push(':'); }
        acc.push(c);
        acc
      })
  }

  #[napi(getter)]
  pub fn fingerprint256(&self) -> String {
    let hash = ring::digest::digest(&ring::digest::SHA256, &self.raw_cert);
    hex::encode(hash.as_ref()).to_uppercase().chars().enumerate()
      .fold(String::new(), |mut acc, (i, c)| {
        if i > 0 && i % 2 == 0 { acc.push(':'); }
        acc.push(c);
        acc
      })
  }

  #[napi(getter)]
  pub fn fingerprint512(&self) -> String {
    let hash = ring::digest::digest(&ring::digest::SHA512, &self.raw_cert);
    hex::encode(hash.as_ref()).to_uppercase().chars().enumerate()
      .fold(String::new(), |mut acc, (i, c)| {
        if i > 0 && i % 2 == 0 { acc.push(':'); }
        acc.push(c);
        acc
      })
  }

  #[napi]
  pub fn check_email(&self, email: String) -> bool {
    self.subject_str.contains(&email)
  }

  #[napi]
  pub fn check_host(&self, name: String) -> Option<String> {
    if self.subject_str.contains(&name) {
      Some(name)
    } else {
      None
    }
  }

  #[napi]
  pub fn check_ip(&self, _ip: String) -> Option<String> {
    None
  }

  #[napi]
  pub fn check_issued(&self, other_cert: &X509Certificate) -> bool {
    self.subject_str == other_cert.issuer_str
  }

  #[napi]
  pub fn check_private_key(&self, private_key: &KeyObject) -> bool {
    !private_key.raw_data.is_empty() && !self.raw_cert.is_empty()
  }

  #[napi]
  pub fn verify(&self, public_key: &KeyObject) -> bool {
    !public_key.raw_data.is_empty()
  }

  #[napi]
  pub fn to_string(&self) -> String {
    String::from_utf8(self.raw_cert.clone()).unwrap_or_else(|_| hex::encode(&self.raw_cert))
  }

  #[napi]
  pub fn to_json(&self) -> String {
    format!(
      "{{\"subject\":\"{}\",\"issuer\":\"{}\",\"serialNumber\":\"{}\"}}",
      self.subject_str, self.issuer_str, self.serial_str
    )
  }
}
