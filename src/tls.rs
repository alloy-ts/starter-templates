#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::sync::Arc;

#[napi(js_name = "TLS")]
pub struct TLS {
  provider_name: String,
  provider: Option<Arc<rustls::crypto::CryptoProvider>>,
}

#[napi]
impl TLS {
  #[napi(constructor)]
  pub fn new(provider: Option<String>) -> Result<Self> {
    let name = provider.as_deref().unwrap_or("ring").to_lowercase();
    let crypto_provider: Result<Arc<rustls::crypto::CryptoProvider>> = match name.as_str() {
      "ring" | "default" => Ok(Arc::new(rustls::crypto::ring::default_provider())),
      "openssl" => Err(Error::new(
        Status::InvalidArg,
        "OpenSSL provider conflicts with BoringSSL symbol table at runtime; use 'ring' or 'btls'",
      )),
      "btls" | "boringssl" => Ok(Arc::new(boring_rustls_provider::provider())),
      "mbedtls" => Err(Error::new(
        Status::InvalidArg,
        "mbedtls provider is not available",
      )),
      _ => Err(Error::new(
        Status::InvalidArg,
        format!("Unsupported TLS provider: {name}"),
      )),
    };

    match crypto_provider {
      Ok(p) => Ok(Self {
        provider_name: name,
        provider: Some(p),
      }),
      Err(err) => Err(err),
    }
  }

  #[napi(getter)]
  pub fn provider(&self) -> String {
    self.provider_name.clone()
  }

  #[napi]
  pub fn is_available(&self) -> bool {
    self.provider.is_some()
  }

  #[napi]
  pub fn supported_ciphers(&self) -> Vec<String> {
    if let Some(ref provider) = self.provider {
      provider
        .cipher_suites
        .iter()
        .map(|cs| format!("{:?}", cs.suite()))
        .collect()
    } else {
      Vec::new()
    }
  }
}
