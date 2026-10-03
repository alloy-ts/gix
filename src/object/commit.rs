use std::sync::Arc;
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub struct Signature {
  pub(crate) name: Option<String>,
  pub(crate) email: Option<String>,
}

#[napi]
impl Signature {
  #[napi(constructor)]
  pub fn new(name: String, email: String) -> Self {
    Signature {
      name: Some(name),
      email: Some(email),
    }
  }

  #[napi]
  pub fn name(&self) -> Option<String> {
    self.name.clone()
  }

  #[napi]
  pub fn email(&self) -> Option<String> {
    self.email.clone()
  }
}

#[napi]
pub struct Commit {
  pub(crate) repo: Arc<gix::ThreadSafeRepository>,
  pub(crate) oid: gix::hash::ObjectId,
}

#[napi]
impl Commit {
  pub fn new(repo: Arc<gix::ThreadSafeRepository>, oid: gix::hash::ObjectId) -> Self {
    Commit { repo, oid }
  }

  #[napi]
  pub fn id(&self) -> String {
    self.oid.to_string()
  }

  #[napi]
  pub fn message(&self) -> Result<Option<String>> {
    let repo = self.repo.to_thread_local();
    let commit = repo.find_commit(self.oid).map_err(|e| Error::from_reason(e.to_string()))?;
    let msg = commit.message_raw().map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Some(msg.to_string()))
  }

  #[napi]
  pub fn summary(&self) -> Result<Option<String>> {
    let repo = self.repo.to_thread_local();
    let commit = repo.find_commit(self.oid).map_err(|e| Error::from_reason(e.to_string()))?;
    let msg = commit.message().map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Some(msg.title.to_string()))
  }

  #[napi]
  pub fn body(&self) -> Result<Option<String>> {
    let repo = self.repo.to_thread_local();
    let commit = repo.find_commit(self.oid).map_err(|e| Error::from_reason(e.to_string()))?;
    let msg = commit.message().map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(msg.body.map(|b| b.to_string()))
  }

  #[napi]
  pub fn time(&self) -> Result<i64> {
    let repo = self.repo.to_thread_local();
    let commit = repo.find_commit(self.oid).map_err(|e| Error::from_reason(e.to_string()))?;
    let time = commit.time().map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(time.seconds)
  }

  #[napi]
  pub fn author(&self) -> Result<Signature> {
    let repo = self.repo.to_thread_local();
    let commit = repo.find_commit(self.oid).map_err(|e| Error::from_reason(e.to_string()))?;
    let author = commit.author().map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Signature {
      name: Some(author.name.to_string()),
      email: Some(author.email.to_string()),
    })
  }

  #[napi]
  pub fn committer(&self) -> Result<Signature> {
    let repo = self.repo.to_thread_local();
    let commit = repo.find_commit(self.oid).map_err(|e| Error::from_reason(e.to_string()))?;
    let committer = commit.committer().map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Signature {
      name: Some(committer.name.to_string()),
      email: Some(committer.email.to_string()),
    })
  }

  #[napi]
  pub fn tree_id(&self) -> Result<String> {
    let repo = self.repo.to_thread_local();
    let commit = repo.find_commit(self.oid).map_err(|e| Error::from_reason(e.to_string()))?;
    let tree_id = commit.tree_id().map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(tree_id.to_string())
  }

  #[napi]
  pub fn parent_count(&self) -> Result<u32> {
    let repo = self.repo.to_thread_local();
    let commit = repo.find_commit(self.oid).map_err(|e| Error::from_reason(e.to_string()))?;
    let count = commit.parent_ids().count() as u32;
    Ok(count)
  }

  #[napi]
  pub fn parent_id(&self, i: u32) -> Result<Option<String>> {
    let repo = self.repo.to_thread_local();
    let commit = repo.find_commit(self.oid).map_err(|e| Error::from_reason(e.to_string()))?;
    let parent_id = commit.parent_ids().nth(i as usize).map(|id| id.detach().to_string());
    Ok(parent_id)
  }
}
