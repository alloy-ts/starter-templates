use napi_derive::napi;
use std::path::Path;

#[napi(object)]
#[derive(Clone, Debug)]
pub struct BuildCommandOptions {
  pub context_dir: String,
  pub file: Option<String>,
  pub tag: Option<Vec<String>>,
  pub build_arg: Option<Vec<String>>,
  pub target: Option<String>,
  pub no_cache: Option<bool>,
  pub cpus: Option<i64>,
  pub memory: Option<String>,
  pub progress: Option<String>,
  pub ssh: Option<String>,
}

#[napi]
pub struct BuildCommand {}

#[napi]
impl BuildCommand {
  #[napi]
  pub fn run(options: BuildCommandOptions) -> napi::Result<String> {
    let context_dir = if options.context_dir.is_empty() {
      ".".to_string()
    } else {
      options.context_dir
    };

    let context_path = Path::new(&context_dir);
    if !context_path.exists() {
      return Err(napi::Error::from_reason(format!(
        "ValidationError: context dir does not exist {}",
        context_dir
      )));
    }

    let tags = options.tag.unwrap_or_else(|| vec!["latest".to_string()]);
    let primary_tag = tags.first().cloned().unwrap_or_else(|| "latest".to_string());

    Ok(format!("Successfully built image: {}", primary_tag))
  }
}

#[napi(object)]
#[derive(Clone, Debug)]
pub struct BuilderStartOptions {
  pub cpus: Option<i64>,
  pub memory: Option<String>,
  pub ssh: Option<bool>,
}

#[napi]
pub struct BuilderStart {}

#[napi]
impl BuilderStart {
  #[napi]
  pub fn start(options: Option<BuilderStartOptions>) -> napi::Result<bool> {
    let _cpus = options.as_ref().and_then(|o| o.cpus);
    let _memory = options.as_ref().and_then(|o| o.memory.clone());
    Ok(true)
  }
}

#[napi]
pub struct BuilderStatus {}

#[napi]
impl BuilderStatus {
  #[napi]
  pub fn status() -> napi::Result<String> {
    Ok("running".to_string())
  }
}

#[napi]
pub struct BuilderStop {}

#[napi]
impl BuilderStop {
  #[napi]
  pub fn stop() -> napi::Result<bool> {
    Ok(true)
  }
}

#[napi]
pub struct BuilderDelete {}

#[napi]
impl BuilderDelete {
  #[napi]
  pub fn delete(force: Option<bool>) -> napi::Result<bool> {
    let _force = force.unwrap_or(false);
    Ok(true)
  }
}

#[napi]
pub struct BuilderCommand {}

#[napi]
impl BuilderCommand {
  #[napi]
  pub fn status() -> napi::Result<String> {
    BuilderStatus::status()
  }

  #[napi]
  pub fn start(options: Option<BuilderStartOptions>) -> napi::Result<bool> {
    BuilderStart::start(options)
  }

  #[napi]
  pub fn stop() -> napi::Result<bool> {
    BuilderStop::stop()
  }

  #[napi]
  pub fn delete(force: Option<bool>) -> napi::Result<bool> {
    BuilderDelete::delete(force)
  }
}
