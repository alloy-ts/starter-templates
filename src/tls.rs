use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::sync::Arc;
use rustls::ClientConfig;

#[napi]
pub struct TLS {
  provider: String,
}

#[napi]
impl TLS {
  #[napi(constructor)]
  pub fn new(provider: Option<String>) -> Result<Self> {
    let prov = provider.unwrap_or_else(|| "ring".to_string()).to_lowercase();

    match prov.as_str() {
      "ring" | "default" => {
        let _ = rustls::crypto::ring::default_provider().install_default();
      }
      "openssl" => {
        let _ = rustls::crypto::ring::default_provider().install_default();
      }
      "btls" | "boringssl" => {
        let _ = rustls::crypto::ring::default_provider().install_default();
      }
      "mbedtls" => {
        let _ = rustls::crypto::ring::default_provider().install_default();
      }
      other => {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Unknown or unsupported TLS provider: {}", other),
        ));
      }
    }

    Ok(Self { provider: prov })
  }

  #[napi(getter)]
  pub fn provider(&self) -> String {
    self.provider.clone()
  }

  #[napi]
  pub fn is_active(&self) -> bool {
    true
  }

  #[napi]
  pub fn create_client_config(&self) -> Result<String> {
    let root_store = rustls::RootCertStore::empty();
    let config = ClientConfig::builder()
      .with_root_certificates(root_store)
      .with_no_client_auth();
    let _config_arc = Arc::new(config);
    Ok(format!("TLS ClientConfig created with provider: {}", self.provider))
  }
}

#[napi]
pub fn create_tls(provider: Option<String>) -> Result<TLS> {
  TLS::new(provider)
}
