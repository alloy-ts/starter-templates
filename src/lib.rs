use napi::bindgen_prelude::*;
use napi_derive::napi;
use svix_ksuid::KsuidLike;

pub const STANDARD_BASE62_ALPHABET: &[u8; 62] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
pub const STANDARD_CROCKFORD_ALPHABET: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimestampMode {
  Bit32, // 4 bytes sec, 16 bytes payload
  Bit48, // 6 bytes ms, 14 bytes payload
  Bit64, // 8 bytes ms, 12 bytes payload
}

pub fn parse_timestamp_mode(
  size_opt: Option<&Either<String, i32>>,
  default_mode: TimestampMode,
) -> TimestampMode {
  match size_opt {
    Some(Either::A(s)) => {
      let l = s.to_lowercase();
      if l.contains("48") || l.contains("6byte") || l.contains("6-byte") || l == "6" {
        TimestampMode::Bit48
      } else if l.contains("64") || l.contains("8byte") || l.contains("8-byte") || l == "8" || l == "ms" {
        TimestampMode::Bit64
      } else if l.contains("32") || l.contains("4byte") || l.contains("4-byte") || l == "4" {
        TimestampMode::Bit32
      } else {
        default_mode
      }
    }
    Some(Either::B(n)) => match *n {
      48 | 6 => TimestampMode::Bit48,
      64 | 8 => TimestampMode::Bit64,
      32 | 4 => TimestampMode::Bit32,
      _ => default_mode,
    },
    None => default_mode,
  }
}

pub fn generate_bytes(mode: TimestampMode, payload: Option<&[u8]>) -> [u8; 20] {
  let mut bytes = [0u8; 20];
  match mode {
    TimestampMode::Bit32 => {
      let k = svix_ksuid::Ksuid::now(payload);
      bytes = *k.bytes();
    }
    TimestampMode::Bit48 => {
      let now_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;
      let ms_bytes = now_ms.to_be_bytes();
      bytes[0..6].copy_from_slice(&ms_bytes[2..8]);
      if let Some(p) = payload {
        let len = p.len().min(14);
        bytes[6..6 + len].copy_from_slice(&p[..len]);
      } else {
        let rnd = svix_ksuid::Ksuid::now(None);
        bytes[6..20].copy_from_slice(&rnd.payload()[..14]);
      }
    }
    TimestampMode::Bit64 => {
      let kms = svix_ksuid::KsuidMs::now(payload);
      bytes = *kms.bytes();
    }
  }
  bytes
}

pub fn read_48bit_ms(bytes: &[u8; 20]) -> i64 {
  let mut ms_bytes = [0u8; 8];
  ms_bytes[2..8].copy_from_slice(&bytes[0..6]);
  u64::from_be_bytes(ms_bytes) as i64
}

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
    let val = values[i] as u16;
    let bit_offset = i * 5;
    let byte_idx = bit_offset / 8;
    let bit_in_byte = bit_offset % 8;

    if bit_in_byte <= 3 {
      bytes[byte_idx] |= (val << (3 - bit_in_byte)) as u8;
    } else {
      bytes[byte_idx] |= (val >> (bit_in_byte - 3)) as u8;
      bytes[byte_idx + 1] |= ((val << (11 - bit_in_byte)) & 0xFF) as u8;
    }
  }

  Ok(bytes)
}

#[napi(object)]
pub struct KsuidOptions {
  pub enc: Option<String>,
  pub alphabet: Option<String>,
  pub timestamp_size: Option<Either<String, i32>>,
}

fn parse_options_struct(
  options: Option<&Either<String, KsuidOptions>>,
  default_mode: TimestampMode,
) -> (String, Option<String>, TimestampMode) {
  match options {
    Some(Either::A(enc_str)) => (enc_str.to_lowercase(), None, default_mode),
    Some(Either::B(opts)) => {
      let enc = opts.enc.as_deref().unwrap_or("base62").to_lowercase();
      let mode = parse_timestamp_mode(opts.timestamp_size.as_ref(), default_mode);
      (enc, opts.alphabet.clone(), mode)
    }
    None => ("base62".to_string(), None, default_mode),
  }
}

#[napi]
pub struct Ksuid {
  bytes: [u8; 20],
  ts_mode: TimestampMode,
  default_enc: String,
  default_alphabet: Option<String>,
}

#[napi]
impl Ksuid {
  #[napi(constructor)]
  pub fn new(
    input: Option<Either<String, Uint8Array>>,
    options: Option<Either<String, KsuidOptions>>,
  ) -> Result<Self> {
    let (enc, alphabet, mode) = parse_options_struct(options.as_ref(), TimestampMode::Bit32);

    match input {
      None => {
        let bytes = generate_bytes(mode, None);
        Ok(Ksuid {
          bytes,
          ts_mode: mode,
          default_enc: enc,
          default_alphabet: alphabet,
        })
      }
      Some(Either::A(s)) => {
        let bytes = if enc == "crockford" || enc == "base32" || enc == "crockfordbase32" || enc == "crockford_base32" {
          decode_crockford(&s, alphabet.as_deref())?
        } else {
          decode_base62(&s, alphabet.as_deref())?
        };
        Ok(Ksuid {
          bytes,
          ts_mode: mode,
          default_enc: enc,
          default_alphabet: alphabet,
        })
      }
      Some(Either::B(b)) => {
        if b.len() != 20 {
          return Err(Error::new(
            Status::InvalidArg,
            format!("Invalid byte length for KSUID: expected 20, got {}", b.len()),
          ));
        }
        let mut bytes = [0u8; 20];
        bytes.copy_from_slice(&b);
        Ok(Ksuid {
          bytes,
          ts_mode: mode,
          default_enc: enc,
          default_alphabet: alphabet,
        })
      }
    }
  }

  #[napi(factory)]
  pub fn now(payload: Option<Uint8Array>) -> Result<Self> {
    let payload_ref = payload.as_ref().map(|p| p.as_ref());
    let bytes = generate_bytes(TimestampMode::Bit32, payload_ref);
    Ok(Ksuid {
      bytes,
      ts_mode: TimestampMode::Bit32,
      default_enc: "base62".to_string(),
      default_alphabet: None,
    })
  }

  #[napi(factory)]
  pub fn from_seconds(timestamp_seconds: Option<i64>, payload: Option<Uint8Array>) -> Result<Self> {
    let payload_ref = payload.as_ref().map(|p| p.as_ref());
    if let Some(p) = payload_ref {
      if p.len() != 16 {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Invalid payload length for KSUID: expected 16, got {}", p.len()),
        ));
      }
    }
    let k = svix_ksuid::Ksuid::from_seconds(timestamp_seconds, payload_ref);
    Ok(Ksuid {
      bytes: *k.bytes(),
      ts_mode: TimestampMode::Bit32,
      default_enc: "base62".to_string(),
      default_alphabet: None,
    })
  }

  #[napi(factory)]
  pub fn from_base62(base62: String, alphabet: Option<String>) -> Result<Self> {
    let bytes = decode_base62(&base62, alphabet.as_deref())?;
    Ok(Ksuid {
      bytes,
      ts_mode: TimestampMode::Bit32,
      default_enc: "base62".to_string(),
      default_alphabet: alphabet,
    })
  }

  #[napi(factory)]
  pub fn from_crockford_base32(base32: String, alphabet: Option<String>) -> Result<Self> {
    let bytes = decode_crockford(&base32, alphabet.as_deref())?;
    Ok(Ksuid {
      bytes,
      ts_mode: TimestampMode::Bit32,
      default_enc: "crockford".to_string(),
      default_alphabet: alphabet,
    })
  }

  #[napi(factory)]
  pub fn from_base32(base32: String, alphabet: Option<String>) -> Result<Self> {
    Self::from_crockford_base32(base32, alphabet)
  }

  #[napi(factory)]
  pub fn from_str(
    encoded: String,
    options: Option<Either<String, KsuidOptions>>,
  ) -> Result<Self> {
    let (enc, alphabet, mode) = parse_options_struct(options.as_ref(), TimestampMode::Bit32);
    let bytes = if enc == "crockford" || enc == "base32" || enc == "crockfordbase32" || enc == "crockford_base32" {
      decode_crockford(&encoded, alphabet.as_deref())?
    } else {
      decode_base62(&encoded, alphabet.as_deref())?
    };

    Ok(Ksuid {
      bytes,
      ts_mode: mode,
      default_enc: enc,
      default_alphabet: alphabet,
    })
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
      bytes: arr,
      ts_mode: TimestampMode::Bit32,
      default_enc: "base62".to_string(),
      default_alphabet: None,
    })
  }

  #[napi]
  pub fn to_base62(&self, alphabet: Option<String>) -> Result<String> {
    let alpha = alphabet.as_deref().or(self.default_alphabet.as_deref());
    encode_base62(&self.bytes, alpha)
  }

  #[napi]
  pub fn to_crockford_base32(&self, alphabet: Option<String>) -> Result<String> {
    let alpha = alphabet.as_deref().or(self.default_alphabet.as_deref());
    encode_crockford(&self.bytes, alpha)
  }

  #[napi]
  pub fn to_base32(&self, alphabet: Option<String>) -> Result<String> {
    self.to_crockford_base32(alphabet)
  }

  #[napi]
  pub fn to_string(&self, options: Option<Either<String, KsuidOptions>>) -> Result<String> {
    match options {
      Some(Either::A(ref enc_str)) => {
        let e = enc_str.to_lowercase();
        if e == "crockford" || e == "base32" || e == "crockfordbase32" || e == "crockford_base32" {
          self.to_crockford_base32(None)
        } else {
          self.to_base62(None)
        }
      }
      Some(Either::B(ref opts)) => {
        let e = opts.enc.as_deref().unwrap_or(&self.default_enc).to_lowercase();
        let alpha = opts.alphabet.as_deref().or(self.default_alphabet.as_deref());
        if e == "crockford" || e == "base32" || e == "crockfordbase32" || e == "crockford_base32" {
          encode_crockford(&self.bytes, alpha)
        } else {
          encode_base62(&self.bytes, alpha)
        }
      }
      None => {
        if self.default_enc == "crockford" || self.default_enc == "base32" || self.default_enc == "crockfordbase32" || self.default_enc == "crockford_base32" {
          self.to_crockford_base32(None)
        } else {
          self.to_base62(None)
        }
      }
    }
  }

  #[napi]
  pub fn bytes(&self) -> Uint8Array {
    Uint8Array::from(self.bytes.as_slice())
  }

  #[napi]
  pub fn payload(&self) -> Uint8Array {
    match self.ts_mode {
      TimestampMode::Bit32 => Uint8Array::from(&self.bytes[4..20]),
      TimestampMode::Bit48 => Uint8Array::from(&self.bytes[6..20]),
      TimestampMode::Bit64 => Uint8Array::from(&self.bytes[8..20]),
    }
  }

  #[napi]
  pub fn timestamp_seconds(&self) -> i64 {
    match self.ts_mode {
      TimestampMode::Bit32 => svix_ksuid::Ksuid::from_bytes(self.bytes).timestamp_seconds(),
      TimestampMode::Bit48 => read_48bit_ms(&self.bytes) / 1000,
      TimestampMode::Bit64 => svix_ksuid::KsuidMs::from_bytes(self.bytes).timestamp_millis() / 1000,
    }
  }

  #[napi]
  pub fn timestamp_millis(&self) -> i64 {
    match self.ts_mode {
      TimestampMode::Bit32 => svix_ksuid::Ksuid::from_bytes(self.bytes).timestamp_seconds() * 1000,
      TimestampMode::Bit48 => read_48bit_ms(&self.bytes),
      TimestampMode::Bit64 => svix_ksuid::KsuidMs::from_bytes(self.bytes).timestamp_millis(),
    }
  }

  #[napi]
  pub fn compare(&self, other: &Ksuid) -> i32 {
    match self.bytes.cmp(&other.bytes) {
      std::cmp::Ordering::Less => -1,
      std::cmp::Ordering::Equal => 0,
      std::cmp::Ordering::Greater => 1,
    }
  }

  #[napi]
  pub fn equals(&self, other: &Ksuid) -> bool {
    self.bytes == other.bytes
  }
}

#[napi]
pub struct KsuidMs {
  bytes: [u8; 20],
  default_enc: String,
  default_alphabet: Option<String>,
}

#[napi]
impl KsuidMs {
  #[napi(constructor)]
  pub fn new(
    input: Option<Either<String, Uint8Array>>,
    options: Option<Either<String, KsuidOptions>>,
  ) -> Result<Self> {
    let (enc, alphabet, _) = parse_options_struct(options.as_ref(), TimestampMode::Bit64);

    match input {
      None => {
        let kms = svix_ksuid::KsuidMs::now(None);
        Ok(KsuidMs {
          bytes: *kms.bytes(),
          default_enc: enc,
          default_alphabet: alphabet,
        })
      }
      Some(Either::A(s)) => {
        let bytes = if enc == "crockford" || enc == "base32" || enc == "crockfordbase32" || enc == "crockford_base32" {
          decode_crockford(&s, alphabet.as_deref())?
        } else {
          decode_base62(&s, alphabet.as_deref())?
        };
        Ok(KsuidMs {
          bytes,
          default_enc: enc,
          default_alphabet: alphabet,
        })
      }
      Some(Either::B(b)) => {
        if b.len() != 20 {
          return Err(Error::new(
            Status::InvalidArg,
            format!("Invalid byte length for KSUIDms: expected 20, got {}", b.len()),
          ));
        }
        let mut bytes = [0u8; 20];
        bytes.copy_from_slice(&b);
        Ok(KsuidMs {
          bytes,
          default_enc: enc,
          default_alphabet: alphabet,
        })
      }
    }
  }

  #[napi(factory)]
  pub fn now(payload: Option<Uint8Array>) -> Result<Self> {
    let payload_ref = payload.as_ref().map(|p| p.as_ref());
    if let Some(p) = payload_ref {
      if p.len() != 12 {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Invalid payload length for KSUIDms: expected 12, got {}", p.len()),
        ));
      }
    }
    let inner = svix_ksuid::KsuidMs::now(payload_ref);
    Ok(KsuidMs {
      bytes: *inner.bytes(),
      default_enc: "base62".to_string(),
      default_alphabet: None,
    })
  }

  #[napi(factory)]
  pub fn from_millis(timestamp_millis: Option<i64>, payload: Option<Uint8Array>) -> Result<Self> {
    let payload_ref = payload.as_ref().map(|p| p.as_ref());
    if let Some(p) = payload_ref {
      if p.len() != 12 {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Invalid payload length for KSUIDms: expected 12, got {}", p.len()),
        ));
      }
    }
    let inner = svix_ksuid::KsuidMs::from_millis(timestamp_millis, payload_ref);
    Ok(KsuidMs {
      bytes: *inner.bytes(),
      default_enc: "base62".to_string(),
      default_alphabet: None,
    })
  }

  #[napi(factory)]
  pub fn from_base62(base62: String, alphabet: Option<String>) -> Result<Self> {
    let bytes = decode_base62(&base62, alphabet.as_deref())?;
    Ok(KsuidMs {
      bytes,
      default_enc: "base62".to_string(),
      default_alphabet: alphabet,
    })
  }

  #[napi(factory)]
  pub fn from_crockford_base32(base32: String, alphabet: Option<String>) -> Result<Self> {
    let bytes = decode_crockford(&base32, alphabet.as_deref())?;
    Ok(KsuidMs {
      bytes,
      default_enc: "crockford".to_string(),
      default_alphabet: alphabet,
    })
  }

  #[napi(factory)]
  pub fn from_base32(base32: String, alphabet: Option<String>) -> Result<Self> {
    Self::from_crockford_base32(base32, alphabet)
  }

  #[napi(factory)]
  pub fn from_str(
    encoded: String,
    options: Option<Either<String, KsuidOptions>>,
  ) -> Result<Self> {
    let (enc, alphabet, _) = parse_options_struct(options.as_ref(), TimestampMode::Bit64);
    let bytes = if enc == "crockford" || enc == "base32" || enc == "crockfordbase32" || enc == "crockford_base32" {
      decode_crockford(&encoded, alphabet.as_deref())?
    } else {
      decode_base62(&encoded, alphabet.as_deref())?
    };

    Ok(KsuidMs {
      bytes,
      default_enc: enc,
      default_alphabet: alphabet,
    })
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
      bytes: arr,
      default_enc: "base62".to_string(),
      default_alphabet: None,
    })
  }

  #[napi]
  pub fn to_base62(&self, alphabet: Option<String>) -> Result<String> {
    let alpha = alphabet.as_deref().or(self.default_alphabet.as_deref());
    encode_base62(&self.bytes, alpha)
  }

  #[napi]
  pub fn to_crockford_base32(&self, alphabet: Option<String>) -> Result<String> {
    let alpha = alphabet.as_deref().or(self.default_alphabet.as_deref());
    encode_crockford(&self.bytes, alpha)
  }

  #[napi]
  pub fn to_base32(&self, alphabet: Option<String>) -> Result<String> {
    self.to_crockford_base32(alphabet)
  }

  #[napi]
  pub fn to_string(&self, options: Option<Either<String, KsuidOptions>>) -> Result<String> {
    match options {
      Some(Either::A(ref enc_str)) => {
        let e = enc_str.to_lowercase();
        if e == "crockford" || e == "base32" || e == "crockfordbase32" || e == "crockford_base32" {
          self.to_crockford_base32(None)
        } else {
          self.to_base62(None)
        }
      }
      Some(Either::B(ref opts)) => {
        let e = opts.enc.as_deref().unwrap_or(&self.default_enc).to_lowercase();
        let alpha = opts.alphabet.as_deref().or(self.default_alphabet.as_deref());
        if e == "crockford" || e == "base32" || e == "crockfordbase32" || e == "crockford_base32" {
          encode_crockford(&self.bytes, alpha)
        } else {
          encode_base62(&self.bytes, alpha)
        }
      }
      None => {
        if self.default_enc == "crockford" || self.default_enc == "base32" || self.default_enc == "crockfordbase32" || self.default_enc == "crockford_base32" {
          self.to_crockford_base32(None)
        } else {
          self.to_base62(None)
        }
      }
    }
  }

  #[napi]
  pub fn bytes(&self) -> Uint8Array {
    Uint8Array::from(self.bytes.as_slice())
  }

  #[napi]
  pub fn payload(&self) -> Uint8Array {
    Uint8Array::from(&self.bytes[8..20])
  }

  #[napi]
  pub fn timestamp_millis(&self) -> i64 {
    svix_ksuid::KsuidMs::from_bytes(self.bytes).timestamp_millis()
  }

  #[napi]
  pub fn compare(&self, other: &KsuidMs) -> i32 {
    match self.bytes.cmp(&other.bytes) {
      std::cmp::Ordering::Less => -1,
      std::cmp::Ordering::Equal => 0,
      std::cmp::Ordering::Greater => 1,
    }
  }

  #[napi]
  pub fn equals(&self, other: &KsuidMs) -> bool {
    self.bytes == other.bytes
  }
}
