use std::sync::Arc;
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub struct Tree {
  pub(crate) repo: Arc<gix::ThreadSafeRepository>,
  pub(crate) oid: gix::hash::ObjectId,
}

#[napi]
impl Tree {
  pub fn new(repo: Arc<gix::ThreadSafeRepository>, oid: gix::hash::ObjectId) -> Self {
    Tree { repo, oid }
  }

  #[napi]
  pub fn id(&self) -> String {
    self.oid.to_string()
  }

  #[napi]
  pub fn len(&self) -> Result<u32> {
    let repo = self.repo.to_thread_local();
    let tree = repo.find_tree(self.oid).map_err(|e| Error::from_reason(e.to_string()))?;
    let decoded = tree.decode().map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(decoded.entries.len() as u32)
  }

  #[napi]
  pub fn is_empty(&self) -> Result<bool> {
    let repo = self.repo.to_thread_local();
    let tree = repo.find_tree(self.oid).map_err(|e| Error::from_reason(e.to_string()))?;
    let decoded = tree.decode().map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(decoded.entries.is_empty())
  }
}
