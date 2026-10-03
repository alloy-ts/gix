use napi::bindgen_prelude::*;
use napi_derive::napi;
use crate::Signature;

#[napi]
pub struct Commit {
  id: String,
  message: Option<String>,
  summary: Option<String>,
  time_seconds: i64,
  author: Signature,
  committer: Signature,
  tree_id: String,
  parent_ids: Vec<String>,
}

impl Commit {
  pub fn new(
    id: String,
    message: Option<String>,
    summary: Option<String>,
    time_seconds: i64,
    author: Signature,
    committer: Signature,
    tree_id: String,
    parent_ids: Vec<String>,
  ) -> Self {
    Commit {
      id,
      message,
      summary,
      time_seconds,
      author,
      committer,
      tree_id,
      parent_ids,
    }
  }
}

#[napi]
impl Commit {
  #[napi]
  pub fn id(&self) -> String {
    self.id.clone()
  }

  #[napi]
  pub fn message(&self) -> Option<String> {
    self.message.clone()
  }

  #[napi]
  pub fn summary(&self) -> Option<String> {
    self.summary.clone()
  }

  #[napi]
  pub fn time(&self) -> i64 {
    self.time_seconds
  }

  #[napi]
  pub fn author(&self) -> Signature {
    Signature::new(
      self.author.name(),
      self.author.email(),
      self.author.time_seconds(),
    )
  }

  #[napi]
  pub fn committer(&self) -> Signature {
    Signature::new(
      self.committer.name(),
      self.committer.email(),
      self.committer.time_seconds(),
    )
  }

  #[napi]
  pub fn tree_id(&self) -> String {
    self.tree_id.clone()
  }

  #[napi]
  pub fn parent_count(&self) -> u32 {
    self.parent_ids.len() as u32
  }

  #[napi]
  pub fn parent_id(&self, i: u32) -> Result<String> {
    self.parent_ids
      .get(i as usize)
      .cloned()
      .ok_or_else(|| Error::new(Status::GenericFailure, "parent index out of bounds"))
  }
}
