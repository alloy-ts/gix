use std::sync::Arc;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use crate::object::{Blob, Commit, Tag, Tree};

#[napi]
pub enum ObjectType {
  Any = -2,
  Bad = -1,
  Commit = 1,
  Tree = 2,
  Blob = 3,
  Tag = 4,
}

#[napi]
pub struct Reference {
  repo: Arc<gix::ThreadSafeRepository>,
  name: String,
}

#[napi]
impl Reference {
  #[napi]
  pub fn name(&self) -> String {
    self.name.clone()
  }

  #[napi]
  pub fn target_id(&self) -> Result<Option<String>> {
    let repo = self.repo.to_thread_local();
    let r = repo.find_reference(&self.name).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(r.target().try_id().map(|id| id.to_hex().to_string()))
  }

  #[napi]
  pub fn is_tag(&self) -> bool {
    self.name.starts_with("refs/tags/")
  }

  #[napi]
  pub fn is_remote(&self) -> bool {
    self.name.starts_with("refs/remotes/")
  }
}

#[napi]
pub struct Repository {
  inner: Arc<gix::ThreadSafeRepository>,
}

#[napi]
impl Repository {
  #[napi]
  pub fn open(path: String) -> Result<Repository> {
    let opts = gix::open::Options::default().bail_if_untrusted(true);
    let repo = gix::ThreadSafeRepository::open_opts(&path, opts)
      .map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Repository {
      inner: Arc::new(repo),
    })
  }

  #[napi]
  pub fn init(path: String) -> Result<Repository> {
    let repo = gix::init(&path).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Repository {
      inner: Arc::new(repo.into_sync()),
    })
  }

  #[napi]
  pub fn init_bare(path: String) -> Result<Repository> {
    let repo = gix::init_bare(&path).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Repository {
      inner: Arc::new(repo.into_sync()),
    })
  }

  #[napi]
  pub fn discover(path: String) -> Result<Repository> {
    let open_opts = gix::open::Options::default().bail_if_untrusted(true);
    let repo = gix::ThreadSafeRepository::discover_opts(
      &path,
      gix::discover::upwards::Options::default(),
      gix::sec::trust::Mapping {
        full: open_opts.clone(),
        reduced: open_opts,
      },
    ).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Repository {
      inner: Arc::new(repo),
    })
  }

  #[napi]
  pub fn is_bare(&self) -> bool {
    let repo = self.inner.to_thread_local();
    repo.is_bare()
  }

  #[napi]
  pub fn is_empty(&self) -> bool {
    let repo = self.inner.to_thread_local();
    let res = repo.head_commit().is_err();
    res
  }

  #[napi]
  pub fn is_shallow(&self) -> bool {
    let repo = self.inner.to_thread_local();
    repo.is_shallow()
  }

  #[napi]
  pub fn is_pristine(&self) -> Option<bool> {
    let repo = self.inner.to_thread_local();
    repo.is_pristine()
  }

  #[napi]
  pub fn is_dirty(&self) -> Result<bool> {
    let repo = self.inner.to_thread_local();
    repo.is_dirty().map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn path(&self) -> String {
    let repo = self.inner.to_thread_local();
    repo.path().to_string_lossy().to_string()
  }

  #[napi]
  pub fn git_dir(&self) -> String {
    let repo = self.inner.to_thread_local();
    repo.git_dir().to_string_lossy().to_string()
  }

  #[napi]
  pub fn common_dir(&self) -> String {
    let repo = self.inner.to_thread_local();
    repo.common_dir().to_string_lossy().to_string()
  }

  #[napi]
  pub fn workdir(&self) -> Option<String> {
    let repo = self.inner.to_thread_local();
    repo.workdir().map(|p| p.to_string_lossy().to_string())
  }

  #[napi]
  pub fn head_name(&self) -> Result<Option<String>> {
    let repo = self.inner.to_thread_local();
    let head = repo.head().map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(head.referent_name().map(|n| n.as_bstr().to_string()))
  }

  #[napi]
  pub fn head_commit_id(&self) -> Result<Option<String>> {
    let repo = self.inner.to_thread_local();
    let head_commit = repo.head_commit();
    match head_commit {
      Ok(commit) => Ok(Some(commit.id().to_string())),
      Err(_) => Ok(None),
    }
  }

  #[napi]
  pub fn head_tree_id(&self) -> Result<Option<String>> {
    let repo = self.inner.to_thread_local();
    let head_tree = repo.head_tree_id();
    match head_tree {
      Ok(id) => Ok(Some(id.to_string())),
      Err(_) => Ok(None),
    }
  }

  #[napi]
  pub fn remote_names(&self) -> Vec<String> {
    let repo = self.inner.to_thread_local();
    repo.remote_names().into_iter().map(|name| name.to_string()).collect()
  }

  #[napi]
  pub fn object_hash(&self) -> String {
    let repo = self.inner.to_thread_local();
    format!("{:?}", repo.object_hash())
  }

  #[napi]
  pub fn write_blob(&self, data: Buffer) -> Result<String> {
    let repo = self.inner.to_thread_local();
    let id = repo.write_blob(&data).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(id.to_string())
  }

  #[napi]
  pub fn has_object(&self, oid: String) -> Result<bool> {
    let oid_parsed = gix::hash::ObjectId::from_hex(oid.as_bytes())
      .map_err(|e| Error::from_reason(e.to_string()))?;
    let repo = self.inner.to_thread_local();
    Ok(repo.has_object(oid_parsed))
  }

  #[napi]
  pub fn find_commit(&self, oid: String) -> Result<Commit> {
    let oid_parsed = gix::hash::ObjectId::from_hex(oid.as_bytes())
      .map_err(|e| Error::from_reason(e.to_string()))?;
    let repo = self.inner.to_thread_local();
    let _ = repo.find_commit(oid_parsed).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Commit::new(self.inner.clone(), oid_parsed))
  }

  #[napi]
  pub fn find_tree(&self, oid: String) -> Result<Tree> {
    let oid_parsed = gix::hash::ObjectId::from_hex(oid.as_bytes())
      .map_err(|e| Error::from_reason(e.to_string()))?;
    let repo = self.inner.to_thread_local();
    let _ = repo.find_tree(oid_parsed).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Tree::new(self.inner.clone(), oid_parsed))
  }

  #[napi]
  pub fn find_blob(&self, oid: String) -> Result<Blob> {
    let oid_parsed = gix::hash::ObjectId::from_hex(oid.as_bytes())
      .map_err(|e| Error::from_reason(e.to_string()))?;
    let repo = self.inner.to_thread_local();
    let _ = repo.find_blob(oid_parsed).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Blob::new(self.inner.clone(), oid_parsed))
  }

  #[napi]
  pub fn find_tag(&self, oid: String) -> Result<Tag> {
    let oid_parsed = gix::hash::ObjectId::from_hex(oid.as_bytes())
      .map_err(|e| Error::from_reason(e.to_string()))?;
    let repo = self.inner.to_thread_local();
    let _ = repo.find_tag(oid_parsed).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Tag::new(self.inner.clone(), oid_parsed))
  }

  #[napi]
  pub fn find_reference(&self, name: String) -> Result<Reference> {
    let repo = self.inner.to_thread_local();
    let _ = repo.find_reference(&name).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Reference {
      repo: self.inner.clone(),
      name,
    })
  }
}
