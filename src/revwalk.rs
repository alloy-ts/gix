use napi::bindgen_prelude::*;
use napi_derive::napi;
use crate::repository::open_gix_repo;

#[napi]
pub struct Revwalk {
  repo_path: String,
  tips: Vec<String>,
}

impl Revwalk {
  pub fn new(repo_path: String) -> Self {
    Revwalk { repo_path, tips: Vec::new() }
  }
}

#[napi]
impl Revwalk {
  #[napi]
  pub fn push_head(&mut self) -> Result<()> {
    let repo = open_gix_repo(&self.repo_path)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    if let Ok(head_id) = repo.head_id() {
      self.tips.push(head_id.to_string());
    }
    Ok(())
  }

  #[napi]
  pub fn push(&mut self, oid_str: String) -> Result<()> {
    self.tips.push(oid_str);
    Ok(())
  }

  #[napi]
  pub fn hide(&mut self, _oid_str: String) -> Result<()> {
    Ok(())
  }

  #[napi]
  pub fn next(&mut self) -> Result<Option<String>> {
    if let Some(tip) = self.tips.pop() {
      Ok(Some(tip))
    } else {
      Ok(None)
    }
  }
}
