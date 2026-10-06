use napi::bindgen_prelude::*;
use napi_derive::napi;

fn decode_spkac(spkac: Either<Buffer, String>, encoding: Option<String>) -> Result<Vec<u8>> {
  let bytes = match spkac {
    Either::A(buf) => buf.to_vec(),
    Either::B(s) => {
      let trimmed = s.trim();
      let stripped = if let Some(rest) = trimmed.strip_prefix("SPKAC=") {
        rest.trim()
      } else {
        trimmed
      };

      let enc = encoding.unwrap_or_else(|| "utf8".to_string());
      match enc.as_str() {
        "base64" => base64::decode(stripped)
          .or_else(|_| base64::decode(stripped.replace(['\r', '\n', ' '], "")))
          .map_err(|e| Error::new(Status::InvalidArg, format!("Failed to decode base64 SPKAC: {}", e)))?,
        "hex" => hex::decode(stripped)
          .map_err(|e| Error::new(Status::InvalidArg, format!("Failed to decode hex SPKAC: {}", e)))?,
        _ => {
          if let Ok(decoded) = base64::decode(stripped.replace(['\r', '\n', ' '], "")) {
            decoded
          } else {
            stripped.as_bytes().to_vec()
          }
        }
      }
    }
  };
  Ok(bytes)
}

fn extract_challenge(der: &[u8]) -> Option<Vec<u8>> {
  if der.len() < 4 {
    return None;
  }

  let mut pos = 0;
  while pos < der.len() {
    let tag = der[pos];
    if pos + 1 >= der.len() {
      break;
    }
    let mut len = der[pos + 1] as usize;
    let mut len_bytes = 1;
    if len > 0x80 {
      let num_bytes = len & 0x7f;
      if pos + 1 + num_bytes >= der.len() {
        break;
      }
      len = 0;
      for i in 1..=num_bytes {
        len = (len << 8) | (der[pos + 1 + i] as usize);
      }
      len_bytes = 1 + num_bytes;
    }

    let val_start = pos + 1 + len_bytes;
    let val_end = val_start + len;

    if val_end > der.len() {
      break;
    }

    if tag == 0x16 || tag == 0x13 || tag == 0x0C {
      return Some(der[val_start..val_end].to_vec());
    }

    if tag == 0x30 {
      pos = val_start;
    } else {
      pos = val_end;
    }
  }

  None
}

fn extract_public_key(der: &[u8]) -> Option<Vec<u8>> {
  if der.len() < 4 || der[0] != 0x30 {
    return None;
  }

  let mut pos = 1;
  let len = der[pos] as usize;
  if len > 0x80 {
    let num_bytes = len & 0x7f;
    pos += 1 + num_bytes;
  } else {
    pos += 1;
  }

  if pos < der.len() && der[pos] == 0x30 {
    let mut inner_pos = pos + 1;
    let inner_len = der[inner_pos] as usize;
    if inner_len > 0x80 {
      let num_bytes = inner_len & 0x7f;
      inner_pos += 1 + num_bytes;
    } else {
      inner_pos += 1;
    }

    if inner_pos < der.len() && der[inner_pos] == 0x30 {
      let spki_start = inner_pos;
      let spki_pos = inner_pos + 1;
      let spki_len_byte = der[spki_pos];
      let mut spki_val_len = spki_len_byte as usize;
      let mut header_len = 2;
      if spki_val_len > 0x80 {
        let num_bytes = spki_val_len & 0x7f;
        spki_val_len = 0;
        for i in 1..=num_bytes {
          spki_val_len = (spki_val_len << 8) | (der[spki_pos + i] as usize);
        }
        header_len = 2 + num_bytes;
      }
      let spki_end = spki_start + header_len + spki_val_len;
      if spki_end <= der.len() {
        return Some(der[spki_start..spki_end].to_vec());
      }
    }
  }

  None
}

#[napi(js_name = "exportChallengeSpkac")]
pub fn export_challenge_spkac(
  spkac: Either<Buffer, String>,
  encoding: Option<String>,
) -> Result<Buffer> {
  let der = decode_spkac(spkac, encoding)?;
  if let Some(challenge) = extract_challenge(&der) {
    Ok(Buffer::from(challenge))
  } else {
    Ok(Buffer::from(Vec::new()))
  }
}

#[napi(js_name = "exportPublicKeySpkac")]
pub fn export_public_key_spkac(
  spkac: Either<Buffer, String>,
  encoding: Option<String>,
) -> Result<Buffer> {
  let der = decode_spkac(spkac, encoding)?;
  if let Some(pk) = extract_public_key(&der) {
    Ok(Buffer::from(pk))
  } else {
    Ok(Buffer::from(Vec::new()))
  }
}

#[napi(js_name = "verifySpkacData")]
pub fn verify_spkac_data(
  spkac: Either<Buffer, String>,
  encoding: Option<String>,
) -> Result<bool> {
  let der = decode_spkac(spkac, encoding);
  match der {
    Ok(data) => Ok(!data.is_empty() && extract_public_key(&data).is_some()),
    Err(_) => Ok(false),
  }
}

#[napi]
pub struct Certificate;

#[napi]
impl Certificate {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self
  }

  #[napi]
  pub fn export_challenge(
    &self,
    spkac: Either<Buffer, String>,
    encoding: Option<String>,
  ) -> Result<Buffer> {
    export_challenge_spkac(spkac, encoding)
  }

  #[napi]
  pub fn export_public_key(
    &self,
    spkac: Either<Buffer, String>,
    encoding: Option<String>,
  ) -> Result<Buffer> {
    export_public_key_spkac(spkac, encoding)
  }

  #[napi]
  pub fn verify_spkac(
    &self,
    spkac: Either<Buffer, String>,
    encoding: Option<String>,
  ) -> Result<bool> {
    verify_spkac_data(spkac, encoding)
  }
}
