use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub struct Worktree {
  name: Option<String>,
  path: String,
}

impl Worktree {
  pub fn new(name: Option<String>, path: String) -> Self {
    Worktree { name, path }
  }
}

#[napi]
impl Worktree {
  #[napi]
  pub fn name(&self) -> Option<String> {
    self.name.clone()
  }

  #[napi]
  pub fn path(&self) -> String {
    self.path.clone()
  }

  #[napi]
  pub fn validate(&self) -> Result<()> {
    Ok(())
  }

  #[napi]
  pub fn lock(&self, _reason: Option<String>) -> Result<()> {
    Ok(())
  }

  #[napi]
  pub fn unlock(&self) -> Result<()> {
    Ok(())
  }

  #[napi]
  pub fn is_locked(&self) -> Result<bool> {
    Ok(false)
  }
}
