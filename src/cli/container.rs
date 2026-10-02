use napi_derive::napi;
use std::collections::HashMap;
use std::path::Path;

use crate::{ContainerClient, ContainerStatus, ProcessIO};

#[napi(object)]
#[derive(Clone, Debug)]
pub struct ContainerStartOptions {
  pub attach: bool,
  pub interactive: bool,
  pub container_id: String,
  pub log_options: Option<String>,
}

#[napi]
pub struct ContainerStart {}

#[napi]
impl ContainerStart {
  #[napi]
  pub fn run(client: &ContainerClient, options: ContainerStartOptions) -> napi::Result<String> {
    let detach = !options.attach && !options.interactive;
    let container = client.get(options.container_id.clone())?;

    if container.status == ContainerStatus::Running {
      if !detach {
        return Err(napi::Error::from_reason(
          "ContainerizationError(invalidArgument): attach is currently unsupported on already running containers",
        ));
      }
      return Ok(container.id);
    }

    for mount in &container.configuration.mounts {
      if mount.is_virtiofs {
        if !Path::new(&mount.source).exists() {
          return Err(napi::Error::from_reason(format!(
            "ContainerizationError(invalidState): mount source path '{}' does not exist",
            mount.source
          )));
        }
      }
    }

    let mut io = ProcessIO::create(
      container.configuration.init_process.terminal,
      options.interactive,
      detach,
    )?;

    let mut env = HashMap::new();
    if let Ok(ssh_auth_sock) = std::env::var("SSH_AUTH_SOCK") {
      env.insert("SSH_AUTH_SOCK".to_string(), ssh_auth_sock);
    }

    match client.bootstrap(container.id.clone(), "stdio".to_string(), env) {
      Ok(mut process) => {
        if detach {
          process.start()?;
          io.close_after_start()?;
          Ok(container.id)
        } else {
          let _exit_code = io.handle_process()?;
          io.close()?;
          Ok(container.id)
        }
      }
      Err(err) => {
        let _ = client.stop(container.id.clone());
        Err(napi::Error::from_reason(format!(
          "ContainerizationError(internalError): failed to start container: {}",
          err
        )))
      }
    }
  }
}
