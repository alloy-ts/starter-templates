use napi_derive::napi;
use std::collections::HashMap;

pub mod cli;
pub use cli::*;

#[napi]
pub fn add(left: i32, right: i32) -> i32 {
  left + right
}

#[napi]
#[derive(Clone, Debug, PartialEq)]
pub enum PacketType {
  ImageTransfer = 0,
  BuildTransfer = 1,
  IO = 2,
  Unknown = 3,
}

#[napi]
#[derive(Clone, Debug)]
pub struct IO {
  pub stdio: String,
}

#[napi]
impl IO {
  #[napi(constructor)]
  pub fn new(stdio: String) -> Self {
    Self { stdio }
  }
}

#[napi]
#[derive(Clone, Debug)]
pub struct Platform {
  pub os: String,
  pub arch: String,
  pub variant: Option<String>,
}

#[napi]
impl Platform {
  #[napi(constructor)]
  pub fn new(os: String, arch: String, variant: Option<String>) -> Self {
    Self { os, arch, variant }
  }

  #[napi(factory)]
  pub fn parse(input: String) -> napi::Result<Self> {
    let parts: Vec<&str> = input.split('/').collect();
    if parts.is_empty() || parts[0].is_empty() {
      return Err(napi::Error::from_reason("invalid platform string"));
    }
    let os = parts[0].to_string();
    let arch = if parts.len() > 1 && !parts[1].is_empty() {
      parts[1].to_string()
    } else {
      "amd64".to_string()
    };
    let variant = if parts.len() > 2 && !parts[2].is_empty() {
      Some(parts[2].to_string())
    } else {
      None
    };
    Ok(Self { os, arch, variant })
  }
}

#[napi]
#[derive(Clone, Debug)]
pub struct BuildTransfer {
  metadata: HashMap<String, String>,
}

#[napi]
impl BuildTransfer {
  #[napi(constructor)]
  pub fn new(metadata: Option<HashMap<String, String>>) -> Self {
    Self {
      metadata: metadata.unwrap_or_default(),
    }
  }

  #[napi]
  pub fn get_metadata(&self) -> HashMap<String, String> {
    self.metadata.clone()
  }

  #[napi]
  pub fn stage(&self) -> Option<String> {
    self
      .metadata
      .get("stage")
      .and_then(|s| if s.is_empty() { None } else { Some(s.clone()) })
  }

  #[napi]
  pub fn method(&self) -> Option<String> {
    self
      .metadata
      .get("method")
      .and_then(|s| if s.is_empty() { None } else { Some(s.clone()) })
  }

  #[napi]
  pub fn include_patterns(&self) -> Option<Vec<String>> {
    self.metadata.get("include-patterns").and_then(|s| {
      if s.is_empty() {
        None
      } else {
        Some(s.split(',').map(|p| p.trim().to_string()).collect())
      }
    })
  }

  #[napi]
  pub fn follow_paths(&self) -> Option<Vec<String>> {
    self.metadata.get("followpaths").and_then(|s| {
      if s.is_empty() {
        None
      } else {
        Some(s.split(',').map(|p| p.trim().to_string()).collect())
      }
    })
  }

  #[napi]
  pub fn mode(&self) -> Option<String> {
    self.metadata.get("mode").cloned()
  }

  #[napi]
  pub fn size(&self) -> Option<i64> {
    self
      .metadata
      .get("size")
      .and_then(|s| if s.is_empty() { None } else { s.parse::<i64>().ok() })
  }

  #[napi]
  pub fn offset(&self) -> Option<f64> {
    self
      .metadata
      .get("offset")
      .and_then(|s| if s.is_empty() { None } else { s.parse::<u64>().ok().map(|v| v as f64) })
  }

  #[napi]
  pub fn len(&self) -> Option<i64> {
    self
      .metadata
      .get("length")
      .and_then(|s| if s.is_empty() { None } else { s.parse::<i64>().ok() })
  }
}

#[napi]
#[derive(Clone, Debug)]
pub struct ImageTransfer {
  metadata: HashMap<String, String>,
}

#[napi]
impl ImageTransfer {
  #[napi(constructor)]
  pub fn new(metadata: Option<HashMap<String, String>>) -> Self {
    Self {
      metadata: metadata.unwrap_or_default(),
    }
  }

  #[napi]
  pub fn get_metadata(&self) -> HashMap<String, String> {
    self.metadata.clone()
  }

  #[napi]
  pub fn stage(&self) -> Option<String> {
    self.metadata.get("stage").cloned()
  }

  #[napi]
  pub fn method(&self) -> Option<String> {
    self.metadata.get("method").cloned()
  }

  #[napi]
  pub fn ref_name(&self) -> Option<String> {
    self.metadata.get("ref").cloned()
  }

  #[napi]
  pub fn platform(&self) -> napi::Result<Option<Platform>> {
    match self.metadata.get("platform") {
      Some(p_str) if !p_str.is_empty() => {
        let p = Platform::parse(p_str.clone())?;
        Ok(Some(p))
      }
      _ => Ok(None),
    }
  }

  #[napi]
  pub fn mode(&self) -> Option<String> {
    self.metadata.get("mode").cloned()
  }

  #[napi]
  pub fn size(&self) -> Option<i64> {
    self
      .metadata
      .get("size")
      .and_then(|s| s.parse::<i64>().ok())
  }

  #[napi]
  pub fn len(&self) -> Option<i64> {
    self
      .metadata
      .get("length")
      .and_then(|s| s.parse::<i64>().ok())
  }

  #[napi]
  pub fn offset(&self) -> Option<f64> {
    self
      .metadata
      .get("offset")
      .and_then(|s| s.parse::<u64>().ok().map(|v| v as f64))
  }
}

#[napi]
#[derive(Clone, Debug)]
pub struct ServerStream {
  packet_type: PacketType,
  image_transfer: Option<ImageTransfer>,
  build_transfer: Option<BuildTransfer>,
  io: Option<IO>,
}

#[napi]
impl ServerStream {
  #[napi(constructor)]
  pub fn new(
    packet_type: PacketType,
    image_transfer: Option<&ImageTransfer>,
    build_transfer: Option<&BuildTransfer>,
    io: Option<&IO>,
  ) -> Self {
    Self {
      packet_type,
      image_transfer: image_transfer.cloned(),
      build_transfer: build_transfer.cloned(),
      io: io.cloned(),
    }
  }

  #[napi]
  pub fn get_packet_type(&self) -> PacketType {
    self.packet_type.clone()
  }

  #[napi]
  pub fn get_image_transfer(&self) -> Option<ImageTransfer> {
    if self.packet_type == PacketType::ImageTransfer {
      self.image_transfer.clone()
    } else {
      None
    }
  }

  #[napi]
  pub fn get_build_transfer(&self) -> Option<BuildTransfer> {
    if self.packet_type == PacketType::BuildTransfer {
      self.build_transfer.clone()
    } else {
      None
    }
  }

  #[napi]
  pub fn get_io(&self) -> Option<IO> {
    if self.packet_type == PacketType::IO {
      self.io.clone()
    } else {
      None
    }
  }
}

#[napi]
#[derive(Clone, Debug, PartialEq)]
pub enum ContainerStatus {
  Created = 0,
  Running = 1,
  Stopped = 2,
  Paused = 3,
}

#[napi(object)]
#[derive(Clone, Debug)]
pub struct Mount {
  pub source: String,
  pub target: String,
  pub is_virtiofs: bool,
}

#[napi(object)]
#[derive(Clone, Debug)]
pub struct InitProcess {
  pub terminal: bool,
}

#[napi(object)]
#[derive(Clone, Debug)]
pub struct ContainerConfiguration {
  pub mounts: Vec<Mount>,
  pub init_process: InitProcess,
}

#[napi(object)]
#[derive(Clone, Debug)]
pub struct Container {
  pub id: String,
  pub status: ContainerStatus,
  pub configuration: ContainerConfiguration,
}

#[napi]
#[derive(Clone, Debug)]
pub struct ProcessIO {
  pub tty: bool,
  pub interactive: bool,
  pub detach: bool,
  pub closed: bool,
}

#[napi]
impl ProcessIO {
  #[napi(factory)]
  pub fn create(tty: bool, interactive: bool, detach: bool) -> napi::Result<Self> {
    Ok(Self {
      tty,
      interactive,
      detach,
      closed: false,
    })
  }

  #[napi]
  pub fn close(&mut self) -> napi::Result<()> {
    self.closed = true;
    Ok(())
  }

  #[napi]
  pub fn close_after_start(&mut self) -> napi::Result<()> {
    self.closed = true;
    Ok(())
  }

  #[napi]
  pub fn handle_process(&mut self) -> napi::Result<i32> {
    Ok(0)
  }
}

#[napi]
#[derive(Clone, Debug)]
pub struct ProcessHandle {
  pub id: String,
  pub running: bool,
}

#[napi]
impl ProcessHandle {
  #[napi]
  pub fn start(&mut self) -> napi::Result<()> {
    self.running = true;
    Ok(())
  }
}

#[napi]
pub struct ContainerClient {
  containers: std::sync::Arc<std::sync::Mutex<HashMap<String, Container>>>,
}

#[napi]
impl ContainerClient {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self {
      containers: std::sync::Arc::new(std::sync::Mutex::new(HashMap::new())),
    }
  }

  #[napi]
  pub fn register_container(&self, container: Container) -> napi::Result<()> {
    let mut map = self
      .containers
      .lock()
      .map_err(|_| napi::Error::from_reason("mutex poisoned"))?;
    map.insert(container.id.clone(), container);
    Ok(())
  }

  #[napi]
  pub fn get(&self, id: String) -> napi::Result<Container> {
    let map = self
      .containers
      .lock()
      .map_err(|_| napi::Error::from_reason("mutex poisoned"))?;
    match map.get(&id) {
      Some(c) => Ok(c.clone()),
      None => Err(napi::Error::from_reason(format!("container '{}' not found", id))),
    }
  }

  #[napi]
  pub fn bootstrap(
    &self,
    id: String,
    _stdio: String,
    _dynamic_env: HashMap<String, String>,
  ) -> napi::Result<ProcessHandle> {
    let mut map = self
      .containers
      .lock()
      .map_err(|_| napi::Error::from_reason("mutex poisoned"))?;
    if let Some(c) = map.get_mut(&id) {
      c.status = ContainerStatus::Running;
    }
    Ok(ProcessHandle { id, running: true })
  }

  #[napi]
  pub fn stop(&self, id: String) -> napi::Result<()> {
    let mut map = self
      .containers
      .lock()
      .map_err(|_| napi::Error::from_reason("mutex poisoned"))?;
    if let Some(c) = map.get_mut(&id) {
      c.status = ContainerStatus::Stopped;
    }
    Ok(())
  }
}

#[napi]
pub struct VirtcontainersRuntime {
  pub active: bool,
}

#[napi]
impl VirtcontainersRuntime {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self { active: true }
  }

  #[napi]
  pub fn version(&self) -> String {
    "runtime-rs/0.1.0".to_string()
  }

  #[napi]
  pub fn create_sandbox(&self, id: String) -> napi::Result<String> {
    Ok(format!("sandbox-{}", id))
  }

  #[napi]
  pub fn start_sandbox(&self, _id: String) -> napi::Result<bool> {
    Ok(true)
  }

  #[napi]
  pub fn stop_sandbox(&self, _id: String) -> napi::Result<bool> {
    Ok(true)
  }
}
