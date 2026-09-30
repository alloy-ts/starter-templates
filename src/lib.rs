use napi::bindgen_prelude::*;
use napi_derive::napi;
use svix_ksuid::KsuidLike;

pub const STANDARD_BASE62_ALPHABET: &[u8; 62] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
pub const STANDARD_CROCKFORD_ALPHABET: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

pub fn encode_base62(bytes: &[u8; 20], custom_alphabet: Option<&str>) -> Result<String> {
  match custom_alphabet {
    None => Ok(svix_ksuid::Ksuid::from_bytes(*bytes).to_base62()),
    Some(a) => {
      let a_bytes = a.as_bytes();
      if a_bytes.len() != 62 {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Base62 alphabet must be exactly 62 characters, got {}", a_bytes.len()),
        ));
      }
      let std_str = svix_ksuid::Ksuid::from_bytes(*bytes).to_base62();
      let mut out = String::with_capacity(std_str.len());
      for ch in std_str.bytes() {
        let pos = STANDARD_BASE62_ALPHABET.iter().position(|&b| b == ch).unwrap();
        out.push(a_bytes[pos] as char);
      }
      Ok(out)
    }
  }
}

pub fn decode_base62(input: &str, custom_alphabet: Option<&str>) -> Result<[u8; 20]> {
  match custom_alphabet {
    None => {
      let k = svix_ksuid::Ksuid::from_base62(input)
        .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid base62 KSUID: {e}")))?;
      Ok(*k.bytes())
    }
    Some(a) => {
      let a_bytes = a.as_bytes();
      if a_bytes.len() != 62 {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Base62 alphabet must be exactly 62 characters, got {}", a_bytes.len()),
        ));
      }
      let mut std_str = String::with_capacity(input.len());
      for ch in input.bytes() {
        let pos = a_bytes.iter().position(|&b| b == ch).ok_or_else(|| {
          Error::new(
            Status::InvalidArg,
            format!("Character '{}' not in custom alphabet", ch as char),
          )
        })?;
        std_str.push(STANDARD_BASE62_ALPHABET[pos] as char);
      }
      let k = svix_ksuid::Ksuid::from_base62(&std_str)
        .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid base62 KSUID: {e}")))?;
      Ok(*k.bytes())
    }
  }
}

pub fn encode_crockford(bytes: &[u8; 20], custom_alphabet: Option<&str>) -> Result<String> {
  let alphabet_bytes = match custom_alphabet {
    Some(a) => {
      let b = a.as_bytes();
      if b.len() != 32 {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Base32 alphabet must be exactly 32 characters, got {}", b.len()),
        ));
      }
      b
    }
    None => STANDARD_CROCKFORD_ALPHABET.as_slice(),
  };

  let mut out = Vec::with_capacity(32);
  for i in 0..32 {
    let bit_offset = i * 5;
    let byte_idx = bit_offset / 8;
    let bit_in_byte = bit_offset % 8;

    let val = if bit_in_byte <= 3 {
      (bytes[byte_idx] >> (3 - bit_in_byte)) & 0x1F
    } else {
      let top = (bytes[byte_idx] << (bit_in_byte - 3)) & 0x1F;
      let bottom = bytes[byte_idx + 1] >> (11 - bit_in_byte);
      top | bottom
    };
    out.push(alphabet_bytes[val as usize]);
  }
  Ok(unsafe { String::from_utf8_unchecked(out) })
}

pub fn decode_crockford(input: &str, custom_alphabet: Option<&str>) -> Result<[u8; 20]> {
  let clean: String = input.chars().filter(|&c| c != '-').collect();
  if clean.len() != 32 {
    return Err(Error::new(
      Status::InvalidArg,
      format!("Base32 string must be 32 characters (excluding hyphens), got {}", clean.len()),
    ));
  }

  let mut values = [0u8; 32];
  if let Some(a) = custom_alphabet {
    let a_bytes = a.as_bytes();
    if a_bytes.len() != 32 {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Base32 alphabet must be exactly 32 characters, got {}", a_bytes.len()),
      ));
    }
    for (idx, ch) in clean.bytes().enumerate() {
      let pos = a_bytes.iter().position(|&b| b == ch).ok_or_else(|| {
        Error::new(
          Status::InvalidArg,
          format!("Character '{}' not found in custom alphabet", ch as char),
        )
      })?;
      values[idx] = pos as u8;
    }
  } else {
    for (idx, ch) in clean.bytes().enumerate() {
      let val = match ch {
        b'0' | b'O' | b'o' => 0,
        b'1' | b'I' | b'i' | b'L' | b'l' => 1,
        b'2'..=b'9' => ch - b'2' + 2,
        b'A' | b'a' => 10,
        b'B' | b'b' => 11,
        b'C' | b'c' => 12,
        b'D' | b'd' => 13,
        b'E' | b'e' => 14,
        b'F' | b'f' => 15,
        b'G' | b'g' => 16,
        b'H' | b'h' => 17,
        b'J' | b'j' => 18,
        b'K' | b'k' => 19,
        b'M' | b'm' => 20,
        b'N' | b'n' => 21,
        b'P' | b'p' => 22,
        b'Q' | b'q' => 23,
        b'R' | b'r' => 24,
        b'S' | b's' => 25,
        b'T' | b't' => 26,
        b'V' | b'v' => 27,
        b'W' | b'w' => 28,
        b'X' | b'x' => 29,
        b'Y' | b'y' => 30,
        b'Z' | b'z' => 31,
        _ => {
          return Err(Error::new(
            Status::InvalidArg,
            format!("Invalid Crockford Base32 character: '{}'", ch as char),
          ));
        }
      };
      values[idx] = val;
    }
  }

  let mut bytes = [0u8; 20];
  for i in 0..32 {
    let val = values[i];
    let bit_offset = i * 5;
    let byte_idx = bit_offset / 8;
    let bit_in_byte = bit_offset % 8;

    if bit_in_byte <= 3 {
      bytes[byte_idx] |= val << (3 - bit_in_byte);
    } else {
      bytes[byte_idx] |= val >> (bit_in_byte - 3);
      bytes[byte_idx + 1] |= (val << (11 - bit_in_byte)) & 0xFF;
    }
  }

  Ok(bytes)
}

#[napi(object)]
pub struct EncodingOptions {
  pub enc: Option<String>,
  pub alphabet: Option<String>,
}

fn parse_encoding_options(options: Option<Either<String, EncodingOptions>>) -> (String, Option<String>) {
  match options {
    Some(Either::A(enc_str)) => (enc_str.to_lowercase(), None),
    Some(Either::B(opts)) => {
      let enc = opts.enc.unwrap_or_else(|| "base62".to_string()).to_lowercase();
      (enc, opts.alphabet)
    }
    None => ("base62".to_string(), None),
  }
}

#[napi]
pub struct Ksuid {
  inner: svix_ksuid::Ksuid,
}

#[napi]
impl Ksuid {
  #[napi(constructor)]
  pub fn new(
    input: Option<Either<String, Uint8Array>>,
    options: Option<Either<String, EncodingOptions>>,
  ) -> Result<Self> {
    match input {
      None => Ok(Ksuid {
        inner: svix_ksuid::Ksuid::now(None),
      }),
      Some(Either::A(s)) => Self::from_str(s, options),
      Some(Either::B(b)) => Self::from_bytes(b),
    }
  }

  #[napi(factory)]
  pub fn now(payload: Option<Uint8Array>) -> Result<Self> {
    let payload_ref = payload.as_ref().map(|p| p.as_ref());
    if let Some(p) = payload_ref {
      if p.len() != svix_ksuid::Ksuid::PAYLOAD_BYTES {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Invalid payload length for KSUID: expected {}, got {}", svix_ksuid::Ksuid::PAYLOAD_BYTES, p.len()),
        ));
      }
    }
    let inner = svix_ksuid::Ksuid::now(payload_ref);
    Ok(Ksuid { inner })
  }

  #[napi(factory)]
  pub fn from_seconds(timestamp_seconds: Option<i64>, payload: Option<Uint8Array>) -> Result<Self> {
    let payload_ref = payload.as_ref().map(|p| p.as_ref());
    if let Some(p) = payload_ref {
      if p.len() != svix_ksuid::Ksuid::PAYLOAD_BYTES {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Invalid payload length for KSUID: expected {}, got {}", svix_ksuid::Ksuid::PAYLOAD_BYTES, p.len()),
        ));
      }
    }
    let inner = svix_ksuid::Ksuid::from_seconds(timestamp_seconds, payload_ref);
    Ok(Ksuid { inner })
  }

  #[napi(factory)]
  pub fn from_base62(base62: String, alphabet: Option<String>) -> Result<Self> {
    let bytes = decode_base62(&base62, alphabet.as_deref())?;
    Ok(Ksuid {
      inner: svix_ksuid::Ksuid::from_bytes(bytes),
    })
  }

  #[napi(factory)]
  pub fn from_crockford_base32(base32: String, alphabet: Option<String>) -> Result<Self> {
    let bytes = decode_crockford(&base32, alphabet.as_deref())?;
    Ok(Ksuid {
      inner: svix_ksuid::Ksuid::from_bytes(bytes),
    })
  }

  #[napi(factory)]
  pub fn from_base32(base32: String, alphabet: Option<String>) -> Result<Self> {
    Self::from_crockford_base32(base32, alphabet)
  }

  #[napi(factory)]
  pub fn from_str(
    encoded: String,
    options: Option<Either<String, EncodingOptions>>,
  ) -> Result<Self> {
    let (enc, alphabet) = parse_encoding_options(options);
    if enc == "crockford" || enc == "base32" || enc == "crockfordbase32" || enc == "crockford_base32" {
      Self::from_crockford_base32(encoded, alphabet)
    } else {
      Self::from_base62(encoded, alphabet)
    }
  }

  #[napi(factory)]
  pub fn from_bytes(bytes: Uint8Array) -> Result<Self> {
    if bytes.len() != 20 {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Invalid byte length for KSUID: expected 20, got {}", bytes.len()),
      ));
    }
    let mut arr = [0u8; 20];
    arr.copy_from_slice(&bytes);
    Ok(Ksuid {
      inner: svix_ksuid::Ksuid::from_bytes(arr),
    })
  }

  #[napi]
  pub fn to_base62(&self, alphabet: Option<String>) -> Result<String> {
    encode_base62(self.inner.bytes(), alphabet.as_deref())
  }

  #[napi]
  pub fn to_crockford_base32(&self, alphabet: Option<String>) -> Result<String> {
    encode_crockford(self.inner.bytes(), alphabet.as_deref())
  }

  #[napi]
  pub fn to_base32(&self, alphabet: Option<String>) -> Result<String> {
    self.to_crockford_base32(alphabet)
  }

  #[napi]
  pub fn to_string(&self, options: Option<Either<String, EncodingOptions>>) -> Result<String> {
    let (enc, alphabet) = parse_encoding_options(options);
    if enc == "crockford" || enc == "base32" || enc == "crockfordbase32" || enc == "crockford_base32" {
      self.to_crockford_base32(alphabet)
    } else {
      self.to_base62(alphabet)
    }
  }

  #[napi]
  pub fn bytes(&self) -> Uint8Array {
    Uint8Array::from(self.inner.bytes().as_slice())
  }

  #[napi]
  pub fn payload(&self) -> Uint8Array {
    Uint8Array::from(self.inner.payload())
  }

  #[napi]
  pub fn timestamp_seconds(&self) -> i64 {
    self.inner.timestamp_seconds()
  }

  #[napi]
  pub fn compare(&self, other: &Ksuid) -> i32 {
    match self.inner.cmp(&other.inner) {
      std::cmp::Ordering::Less => -1,
      std::cmp::Ordering::Equal => 0,
      std::cmp::Ordering::Greater => 1,
    }
  }

  #[napi]
  pub fn equals(&self, other: &Ksuid) -> bool {
    self.inner == other.inner
  }
}

#[napi]
pub struct KsuidMs {
  inner: svix_ksuid::KsuidMs,
}

#[napi]
impl KsuidMs {
  #[napi(constructor)]
  pub fn new(
    input: Option<Either<String, Uint8Array>>,
    options: Option<Either<String, EncodingOptions>>,
  ) -> Result<Self> {
    match input {
      None => Ok(KsuidMs {
        inner: svix_ksuid::KsuidMs::now(None),
      }),
      Some(Either::A(s)) => Self::from_str(s, options),
      Some(Either::B(b)) => Self::from_bytes(b),
    }
  }

  #[napi(factory)]
  pub fn now(payload: Option<Uint8Array>) -> Result<Self> {
    let payload_ref = payload.as_ref().map(|p| p.as_ref());
    if let Some(p) = payload_ref {
      if p.len() != svix_ksuid::KsuidMs::PAYLOAD_BYTES {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Invalid payload length for KSUIDms: expected {}, got {}", svix_ksuid::KsuidMs::PAYLOAD_BYTES, p.len()),
        ));
      }
    }
    let inner = svix_ksuid::KsuidMs::now(payload_ref);
    Ok(KsuidMs { inner })
  }

  #[napi(factory)]
  pub fn from_millis(timestamp_millis: Option<i64>, payload: Option<Uint8Array>) -> Result<Self> {
    let payload_ref = payload.as_ref().map(|p| p.as_ref());
    if let Some(p) = payload_ref {
      if p.len() != svix_ksuid::KsuidMs::PAYLOAD_BYTES {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Invalid payload length for KSUIDms: expected {}, got {}", svix_ksuid::KsuidMs::PAYLOAD_BYTES, p.len()),
        ));
      }
    }
    let inner = svix_ksuid::KsuidMs::from_millis(timestamp_millis, payload_ref);
    Ok(KsuidMs { inner })
  }

  #[napi(factory)]
  pub fn from_base62(base62: String, alphabet: Option<String>) -> Result<Self> {
    let bytes = decode_base62(&base62, alphabet.as_deref())?;
    Ok(KsuidMs {
      inner: svix_ksuid::KsuidMs::from_bytes(bytes),
    })
  }

  #[napi(factory)]
  pub fn from_crockford_base32(base32: String, alphabet: Option<String>) -> Result<Self> {
    let bytes = decode_crockford(&base32, alphabet.as_deref())?;
    Ok(KsuidMs {
      inner: svix_ksuid::KsuidMs::from_bytes(bytes),
    })
  }

  #[napi(factory)]
  pub fn from_base32(base32: String, alphabet: Option<String>) -> Result<Self> {
    Self::from_crockford_base32(base32, alphabet)
  }

  #[napi(factory)]
  pub fn from_str(
    encoded: String,
    options: Option<Either<String, EncodingOptions>>,
  ) -> Result<Self> {
    let (enc, alphabet) = parse_encoding_options(options);
    if enc == "crockford" || enc == "base32" || enc == "crockfordbase32" || enc == "crockford_base32" {
      Self::from_crockford_base32(encoded, alphabet)
    } else {
      Self::from_base62(encoded, alphabet)
    }
  }

  #[napi(factory)]
  pub fn from_bytes(bytes: Uint8Array) -> Result<Self> {
    if bytes.len() != 20 {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Invalid byte length for KSUIDms: expected 20, got {}", bytes.len()),
      ));
    }
    let mut arr = [0u8; 20];
    arr.copy_from_slice(&bytes);
    Ok(KsuidMs {
      inner: svix_ksuid::KsuidMs::from_bytes(arr),
    })
  }

  #[napi]
  pub fn to_base62(&self, alphabet: Option<String>) -> Result<String> {
    encode_base62(self.inner.bytes(), alphabet.as_deref())
  }

  #[napi]
  pub fn to_crockford_base32(&self, alphabet: Option<String>) -> Result<String> {
    encode_crockford(self.inner.bytes(), alphabet.as_deref())
  }

  #[napi]
  pub fn to_base32(&self, alphabet: Option<String>) -> Result<String> {
    self.to_crockford_base32(alphabet)
  }

  #[napi]
  pub fn to_string(&self, options: Option<Either<String, EncodingOptions>>) -> Result<String> {
    let (enc, alphabet) = parse_encoding_options(options);
    if enc == "crockford" || enc == "base32" || enc == "crockfordbase32" || enc == "crockford_base32" {
      self.to_crockford_base32(alphabet)
    } else {
      self.to_base62(alphabet)
    }
  }

  #[napi]
  pub fn bytes(&self) -> Uint8Array {
    Uint8Array::from(self.inner.bytes().as_slice())
  }

  #[napi]
  pub fn payload(&self) -> Uint8Array {
    Uint8Array::from(self.inner.payload())
  }

  #[napi]
  pub fn timestamp_millis(&self) -> i64 {
    self.inner.timestamp_millis()
  }

  #[napi]
  pub fn compare(&self, other: &KsuidMs) -> i32 {
    match self.inner.cmp(&other.inner) {
      std::cmp::Ordering::Less => -1,
      std::cmp::Ordering::Equal => 0,
      std::cmp::Ordering::Greater => 1,
    }
  }

  #[napi]
  pub fn equals(&self, other: &KsuidMs) -> bool {
    self.inner == other.inner
  }
}
