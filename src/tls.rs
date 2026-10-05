#![deny(clippy::all)]

use napi_derive::napi;

#[napi]
pub enum TlsProvider {
  Ring,
  OpenSSL,
  BoringSSL,
  MbedTLS,
}

#[napi(js_name = "TLS")]
pub struct TLS {
  provider: TlsProvider,
}

#[napi]
impl TLS {
  #[napi(constructor)]
  pub fn new(provider: Option<TlsProvider>) -> Self {
    Self {
      provider: provider.unwrap_or(TlsProvider::Ring),
    }
  }

  #[napi]
  pub fn get_provider_name(&self) -> String {
    match self.provider {
      TlsProvider::Ring => "ring".to_string(),
      TlsProvider::OpenSSL => "openssl".to_string(),
      TlsProvider::BoringSSL => "boringssl".to_string(),
      TlsProvider::MbedTLS => "mbedtls".to_string(),
    }
  }

  #[napi]
  pub fn is_supported(&self) -> bool {
    true
  }
}
