use std::sync::Arc;
use napi::bindgen_prelude::*;
use napi_derive::napi;

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
pub struct Signature {
  name: Option<String>,
  email: Option<String>,
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
pub struct Tree {
  repo: Arc<gix::ThreadSafeRepository>,
  oid: gix::hash::ObjectId,
}

#[napi]
impl Tree {
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

#[napi]
pub struct Commit {
  repo: Arc<gix::ThreadSafeRepository>,
  oid: gix::hash::ObjectId,
}

#[napi]
impl Commit {
  #[napi]
  pub fn id(&self) -> String {
    self.oid.to_string()
  }

  #[napi]
  pub fn message(&self) -> Result<Option<String>> {
    let repo = self.repo.to_thread_local();
    let commit = repo.find_commit(self.oid).map_err(|e| Error::from_reason(e.to_string()))?;
    let msg = commit.message().map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Some(msg.title.to_string()))
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
  pub fn path(&self) -> String {
    let repo = self.inner.to_thread_local();
    repo.path().to_string_lossy().to_string()
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
  pub fn find_commit(&self, oid: String) -> Result<Commit> {
    let oid_parsed = gix::hash::ObjectId::from_hex(oid.as_bytes())
      .map_err(|e| Error::from_reason(e.to_string()))?;
    let repo = self.inner.to_thread_local();
    let _ = repo.find_commit(oid_parsed).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Commit {
      repo: self.inner.clone(),
      oid: oid_parsed,
    })
  }

  #[napi]
  pub fn find_tree(&self, oid: String) -> Result<Tree> {
    let oid_parsed = gix::hash::ObjectId::from_hex(oid.as_bytes())
      .map_err(|e| Error::from_reason(e.to_string()))?;
    let repo = self.inner.to_thread_local();
    let _ = repo.find_tree(oid_parsed).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Tree {
      repo: self.inner.clone(),
      oid: oid_parsed,
    })
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
