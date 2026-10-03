use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::sync::Arc;

#[napi]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TlsProvider {
  Ring = 0,
  OpenSSL = 1,
  Btls = 2,
  MbedTLS = 3,
}

#[napi]
pub struct TLS {
  provider: TlsProvider,
  crypto_provider: Arc<rustls::crypto::CryptoProvider>,
}

#[napi]
impl TLS {
  #[napi(constructor)]
  pub fn new(provider: Option<TlsProvider>) -> Result<Self> {
    let requested_provider = provider.unwrap_or(TlsProvider::Ring);
    let (effective_provider, crypto_provider) = match requested_provider {
      TlsProvider::Ring => (
        TlsProvider::Ring,
        Arc::new(rustls::crypto::ring::default_provider()),
      ),
      TlsProvider::OpenSSL => (
        TlsProvider::Ring,
        Arc::new(rustls::crypto::ring::default_provider()),
      ),
      TlsProvider::Btls => match std::panic::catch_unwind(|| boring_rustls_provider::provider()) {
        Ok(prov) => (TlsProvider::Btls, Arc::new(prov)),
        Err(_) => (
          TlsProvider::Ring,
          Arc::new(rustls::crypto::ring::default_provider()),
        ),
      },
      TlsProvider::MbedTLS => match std::panic::catch_unwind(|| rustls_mbedcrypto_provider::mbedtls_crypto_provider()) {
        Ok(prov) => (TlsProvider::MbedTLS, Arc::new(prov)),
        Err(_) => (
          TlsProvider::Ring,
          Arc::new(rustls::crypto::ring::default_provider()),
        ),
      },
    };

    Ok(Self {
      provider: effective_provider,
      crypto_provider,
    })
  }

  #[napi(getter)]
  pub fn provider(&self) -> String {
    match self.provider {
      TlsProvider::Ring => "ring".to_string(),
      TlsProvider::OpenSSL => "openssl".to_string(),
      TlsProvider::Btls => "btls".to_string(),
      TlsProvider::MbedTLS => "mbedtls".to_string(),
    }
  }

  #[napi]
  pub fn is_supported(&self) -> bool {
    true
  }

  #[napi]
  pub fn get_cipher_suites(&self) -> Vec<String> {
    self
      .crypto_provider
      .cipher_suites
      .iter()
      .map(|cs| format!("{:?}", cs.suite()))
      .collect()
  }
}
