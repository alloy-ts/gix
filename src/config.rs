use napi::bindgen_prelude::*;
use napi_derive::napi;
use crate::repository::open_gix_repo;

#[napi]
pub struct Config {
  repo_path: String,
}

impl Config {
  pub fn new(repo_path: String) -> Self {
    Config { repo_path }
  }
}

#[napi]
impl Config {
  #[napi]
  pub fn get_string(&self, name: String) -> Result<String> {
    let repo = open_gix_repo(&self.repo_path)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let config = repo.config_snapshot();
    let val = config.string(&name)
      .ok_or_else(|| Error::new(Status::GenericFailure, format!("key not found: {name}")))?;
    Ok(val.to_string())
  }

  #[napi]
  pub fn set_string(&mut self, name: String, value: String) -> Result<()> {
    let repo = open_gix_repo(&self.repo_path)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let config_path = repo.config_path(gix::config::Source::Local)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let mut file = repo.config_file_mut(config_path)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    file.set_raw_value(&name, value.as_bytes())
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    file.commit().map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    Ok(())
  }

  #[napi]
  pub fn get_bool(&self, name: String) -> Result<bool> {
    let repo = open_gix_repo(&self.repo_path)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let config = repo.config_snapshot();
    let val = config.boolean(&name)
      .ok_or_else(|| Error::new(Status::GenericFailure, format!("key not found: {name}")))?;
    Ok(val)
  }

  #[napi]
  pub fn set_bool(&mut self, name: String, value: bool) -> Result<()> {
    self.set_string(name, value.to_string())
  }

  #[napi]
  pub fn remove(&mut self, _name: String) -> Result<()> {
    Ok(())
  }
}
