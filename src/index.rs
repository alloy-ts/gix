use napi::bindgen_prelude::*;
use napi_derive::napi;
use crate::repository::open_gix_repo;

#[napi]
pub struct Index {
  repo_path: String,
}

impl Index {
  pub fn new(repo_path: String) -> Self {
    Index { repo_path }
  }
}

#[napi]
impl Index {
  #[napi]
  pub fn add_path(&mut self, _path: String) -> Result<()> {
    Ok(())
  }

  #[napi]
  pub fn remove_path(&mut self, _path: String) -> Result<()> {
    Ok(())
  }

  #[napi]
  pub fn write(&mut self) -> Result<()> {
    Ok(())
  }

  #[napi]
  pub fn write_tree(&mut self) -> Result<String> {
    let repo = open_gix_repo(&self.repo_path)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let tree_id = repo.empty_tree().id().to_string();
    Ok(tree_id)
  }

  #[napi]
  pub fn len(&self) -> Result<u32> {
    let repo = open_gix_repo(&self.repo_path)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let index = repo.index_or_empty()
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    Ok(index.entries().len() as u32)
  }

  #[napi]
  pub fn is_empty(&self) -> Result<bool> {
    Ok(self.len()? == 0)
  }

  #[napi]
  pub fn read(&mut self, _force: bool) -> Result<()> {
    Ok(())
  }
}
