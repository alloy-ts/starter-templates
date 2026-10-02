use napi_derive::napi;

#[napi(object)]
#[derive(Clone, Debug)]
pub struct ComposeUpOptions {
  pub file: Option<String>,
  pub detach: Option<bool>,
  pub project_name: Option<String>,
}

#[napi]
pub struct ContainerCompose {}

#[napi]
impl ContainerCompose {
  #[napi]
  pub fn up(options: Option<ComposeUpOptions>) -> napi::Result<String> {
    let project = options
      .as_ref()
      .and_then(|o| o.project_name.clone())
      .unwrap_or_else(|| "default".to_string());
    Ok(format!("Compose project '{}' started", project))
  }

  #[napi]
  pub fn down(project_name: Option<String>) -> napi::Result<String> {
    let project = project_name.unwrap_or_else(|| "default".to_string());
    Ok(format!("Compose project '{}' stopped", project))
  }
}
