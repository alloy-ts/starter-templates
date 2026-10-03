#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub enum TlsProvider {
  Ring,
  Openssl,
  Boring,
  Mbedtls,
}

#[napi]
pub struct Tls {
  provider: TlsProvider,
}

#[napi]
impl Tls {
  #[napi(constructor)]
  pub fn new(provider: Option<String>) -> Result<Self> {
    let prov = match provider.as_deref().unwrap_or("ring").to_lowercase().as_str() {
      "ring" => TlsProvider::Ring,
      "openssl" => TlsProvider::Openssl,
      "boring" | "boringssl" | "btls" => TlsProvider::Boring,
      "mbedtls" => TlsProvider::Mbedtls,
      p => {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Unknown TLS provider: {p}"),
        ));
      }
    };

    Ok(Tls { provider: prov })
  }

  #[napi(getter)]
  pub fn provider(&self) -> String {
    match self.provider {
      TlsProvider::Ring => "ring".to_string(),
      TlsProvider::Openssl => "openssl".to_string(),
      TlsProvider::Boring => "boringssl".to_string(),
      TlsProvider::Mbedtls => "mbedtls".to_string(),
    }
  }

  #[napi]
  pub fn is_supported(&self) -> bool {
    match self.provider {
      TlsProvider::Ring | TlsProvider::Openssl | TlsProvider::Boring => true,
      TlsProvider::Mbedtls => false,
    }
  }
}
