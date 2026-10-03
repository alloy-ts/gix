use std::sync::Arc;
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub struct Blob {
  pub(crate) repo: Arc<gix::ThreadSafeRepository>,
  pub(crate) oid: gix::hash::ObjectId,
}

#[napi]
impl Blob {
  pub fn new(repo: Arc<gix::ThreadSafeRepository>, oid: gix::hash::ObjectId) -> Self {
    Blob { repo, oid }
  }

  #[napi]
  pub fn id(&self) -> String {
    self.oid.to_string()
  }

  #[napi]
  pub fn content(&self) -> Result<Buffer> {
    let repo = self.repo.to_thread_local();
    let blob = repo.find_blob(self.oid).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Buffer::from(blob.data.as_slice()))
  }

  #[napi]
  pub fn size(&self) -> Result<u32> {
    let repo = self.repo.to_thread_local();
    let blob = repo.find_blob(self.oid).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(blob.data.len() as u32)
  }
}
