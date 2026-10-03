use std::sync::Arc;
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub struct Tag {
  pub(crate) repo: Arc<gix::ThreadSafeRepository>,
  pub(crate) oid: gix::hash::ObjectId,
}

#[napi]
impl Tag {
  pub fn new(repo: Arc<gix::ThreadSafeRepository>, oid: gix::hash::ObjectId) -> Self {
    Tag { repo, oid }
  }

  #[napi]
  pub fn id(&self) -> String {
    self.oid.to_string()
  }

  #[napi]
  pub fn name(&self) -> Result<Option<String>> {
    let repo = self.repo.to_thread_local();
    let tag = repo.find_tag(self.oid).map_err(|e| Error::from_reason(e.to_string()))?;
    let decoded = tag.decode().map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Some(decoded.name.to_string()))
  }

  #[napi]
  pub fn target_id(&self) -> Result<String> {
    let repo = self.repo.to_thread_local();
    let tag = repo.find_tag(self.oid).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(tag.target_id().map_err(|e| Error::from_reason(e.to_string()))?.to_string())
  }
}
