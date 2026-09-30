use napi_derive::napi;
use sqids::Sqids as RustSqids;
use std::collections::HashSet;

#[napi(object)]
pub struct SqidsOptions {
  pub alphabet: Option<String>,
  pub min_length: Option<u8>,
  pub blocklist: Option<Vec<String>>,
}

#[napi]
pub struct NativeSqids {
  inner: RustSqids,
}

#[napi]
impl NativeSqids {
  #[napi(constructor)]
  pub fn new(options: Option<SqidsOptions>) -> napi::Result<Self> {
    let mut builder = RustSqids::builder();
    if let Some(opts) = options {
      if let Some(alphabet) = opts.alphabet {
        builder = builder.alphabet(alphabet.chars().collect());
      }
      if let Some(min_length) = opts.min_length {
        builder = builder.min_length(min_length);
      }
      if let Some(blocklist) = opts.blocklist {
        let set: HashSet<String> = blocklist.into_iter().collect();
        builder = builder.blocklist(set);
      }
    }
    let inner = builder
      .build()
      .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(Self { inner })
  }

  #[napi]
  pub fn encode(&self, numbers: Vec<f64>) -> napi::Result<String> {
    let u64_nums: Vec<u64> = numbers.into_iter().map(|n| n as u64).collect();
    self
      .inner
      .encode(&u64_nums)
      .map_err(|e| napi::Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn decode(&self, id: String) -> Vec<f64> {
    self
      .inner
      .decode(&id)
      .into_iter()
      .map(|n| n as f64)
      .collect()
  }
}
