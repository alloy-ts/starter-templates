use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub struct Tls {
  pub provider: String,
  pub is_connected: bool,
}

#[napi]
impl Tls {
  #[napi(constructor)]
  pub fn new(provider: Option<String>) -> Self {
    let p = provider.unwrap_or_else(|| "ring".to_string());
    Tls {
      provider: p,
      is_connected: false,
    }
  }

  #[napi]
  pub fn connect(&mut self, host: String) -> Result<bool> {
    if host.is_empty() {
      return Err(Error::new(Status::InvalidArg, "Host cannot be empty"));
    }
    self.is_connected = true;
    Ok(true)
  }

  #[napi]
  pub fn disconnect(&mut self) -> bool {
    self.is_connected = false;
    true
  }
}
