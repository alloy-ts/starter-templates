use napi::bindgen_prelude::*;
use napi_derive::napi;
use pkcs8::SecretDocument;

#[napi]
pub fn parse_pkcs8_pem(pem: String) -> Result<Buffer> {
  let (_, doc) = SecretDocument::from_pem(&pem)
    .map_err(|e| Error::new(Status::InvalidArg, format!("Failed to parse PKCS8 PEM: {}", e)))?;
  Ok(Buffer::from(doc.as_bytes().to_vec()))
}

#[napi]
pub fn export_pkcs8_pem(der: Buffer) -> Result<String> {
  let doc = SecretDocument::try_from(der.as_ref())
    .map_err(|e| Error::new(Status::InvalidArg, format!("Failed to create PKCS8 doc: {}", e)))?;
  let pem = doc
    .to_pem("PRIVATE KEY", pkcs8::LineEnding::LF)
    .map_err(|e| Error::new(Status::GenericFailure, format!("Failed to export PEM: {}", e)))?;
  Ok(pem.to_string())
}
