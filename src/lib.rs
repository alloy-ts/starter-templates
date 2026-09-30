use std::collections::HashMap;
use std::sync::Arc;

use napi::bindgen_prelude::*;
use napi_derive::napi;
use object_store::local::LocalFileSystem;
use object_store::memory::InMemory;
use object_store::path::Path;
use object_store::{ObjectStoreExt, PutPayload};
use url::Url;

#[napi(object)]
pub struct ObjectMeta {
  pub location: String,
  pub last_modified: String,
  pub size: i64,
  pub e_tag: Option<String>,
  pub version: Option<String>,
}

impl From<object_store::ObjectMeta> for ObjectMeta {
  fn from(meta: object_store::ObjectMeta) -> Self {
    Self {
      location: meta.location.to_string(),
      last_modified: meta.last_modified.to_rfc3339(),
      size: meta.size as i64,
      e_tag: meta.e_tag,
      version: meta.version,
    }
  }
}

#[napi(object)]
pub struct GetResult {
  pub meta: ObjectMeta,
  pub bytes: Buffer,
}

#[napi(object)]
pub struct PutResult {
  pub e_tag: Option<String>,
  pub version: Option<String>,
}

#[napi(object)]
pub struct ByteRange {
  pub start: i64,
  pub end: i64,
}

#[napi]
pub struct ObjectStore {
  inner: Arc<dyn object_store::ObjectStore>,
}

#[napi]
impl ObjectStore {
  #[napi(factory)]
  pub fn memory() -> Self {
    Self {
      inner: Arc::new(InMemory::new()),
    }
  }

  #[napi(factory)]
  pub fn local(path: String) -> Result<Self> {
    let local = LocalFileSystem::new_with_prefix(&path)
      .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
    Ok(Self {
      inner: Arc::new(local),
    })
  }

  #[napi(factory)]
  pub fn parse_url(url: String, options: Option<HashMap<String, String>>) -> Result<Self> {
    let parsed_url =
      Url::parse(&url).map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;
    let opts = options.unwrap_or_default();
    let (store, _) = object_store::parse_url_opts(&parsed_url, opts)
      .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
    Ok(Self {
      inner: Arc::from(store),
    })
  }

  #[napi]
  pub async fn put(&self, path: String, bytes: Buffer) -> Result<PutResult> {
    let p = Path::from(path);
    let payload = PutPayload::from(bytes.as_ref().to_vec());
    let res = self
      .inner
      .put(&p, payload)
      .await
      .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
    Ok(PutResult {
      e_tag: res.e_tag,
      version: res.version,
    })
  }

  #[napi]
  pub async fn get(&self, path: String) -> Result<GetResult> {
    let p = Path::from(path);
    let res = self
      .inner
      .get(&p)
      .await
      .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
    let meta = ObjectMeta::from(res.meta.clone());
    let bytes = res
      .bytes()
      .await
      .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
    Ok(GetResult {
      meta,
      bytes: Buffer::from(bytes.as_ref()),
    })
  }

  #[napi]
  pub async fn get_bytes(&self, path: String) -> Result<Buffer> {
    let p = Path::from(path);
    let res = self
      .inner
      .get(&p)
      .await
      .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
    let bytes = res
      .bytes()
      .await
      .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
    Ok(Buffer::from(bytes.as_ref()))
  }

  #[napi]
  pub async fn head(&self, path: String) -> Result<ObjectMeta> {
    let p = Path::from(path);
    let meta = self
      .inner
      .head(&p)
      .await
      .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
    Ok(ObjectMeta::from(meta))
  }

  #[napi]
  pub async fn delete(&self, path: String) -> Result<()> {
    let p = Path::from(path);
    self
      .inner
      .delete(&p)
      .await
      .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
    Ok(())
  }

  #[napi]
  pub async fn list(&self, prefix: Option<String>) -> Result<Vec<ObjectMeta>> {
    use futures::StreamExt;
    let p = prefix.map(Path::from);
    let mut stream = self.inner.list(p.as_ref());
    let mut results = Vec::new();
    while let Some(meta_res) = stream.next().await {
      let meta = meta_res.map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
      results.push(ObjectMeta::from(meta));
    }
    Ok(results)
  }

  #[napi]
  pub async fn copy(&self, from: String, to: String) -> Result<()> {
    let from_p = Path::from(from);
    let to_p = Path::from(to);
    self
      .inner
      .copy(&from_p, &to_p)
      .await
      .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
    Ok(())
  }

  #[napi]
  pub async fn rename(&self, from: String, to: String) -> Result<()> {
    let from_p = Path::from(from);
    let to_p = Path::from(to);
    self
      .inner
      .rename(&from_p, &to_p)
      .await
      .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
    Ok(())
  }

  #[napi]
  pub async fn get_ranges(&self, path: String, ranges: Vec<ByteRange>) -> Result<Vec<Buffer>> {
    let p = Path::from(path);
    let r_vec: Vec<std::ops::Range<u64>> = ranges
      .into_iter()
      .map(|r| r.start as u64..r.end as u64)
      .collect();
    let res = self
      .inner
      .get_ranges(&p, &r_vec)
      .await
      .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
    Ok(
      res
        .into_iter()
        .map(|b| Buffer::from(b.as_ref()))
        .collect(),
    )
  }
}
