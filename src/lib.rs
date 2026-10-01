use std::sync::{Arc, Mutex};
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub enum BranchType {
  Local = 1,
  Remote = 2,
}

#[napi]
pub enum ObjectType {
  Any = -2,
  Bad = -1,
  Commit = 1,
  Tree = 2,
  Blob = 3,
  Tag = 4,
  OffsetDelta = 6,
  HashDelta = 7,
}

#[napi]
pub enum RepositoryState {
  Clean = 0,
  Merge = 1,
  Revert = 2,
  RevertSequence = 3,
  CherryPick = 4,
  CherryPickSequence = 5,
  Bisect = 6,
  Rebase = 7,
  RebaseInteractive = 8,
  RebaseMerge = 9,
  ApplyMailbox = 10,
  ApplyMailboxOrRebase = 11,
}

#[napi]
pub enum Delta {
  Unmodified = 0,
  Added = 1,
  Deleted = 2,
  Modified = 3,
  Renamed = 4,
  Copied = 5,
  Ignored = 6,
  Untracked = 7,
  Typechange = 8,
  Unreadable = 9,
  Conflicted = 10,
}

#[napi]
pub enum ResetType {
  Soft = 1,
  Mixed = 2,
  Hard = 3,
}

#[napi]
pub fn message_prettify(message: String, comment_char: Option<String>) -> Result<String> {
  let cc = comment_char.and_then(|s| s.chars().next());
  git2::message_prettify(&message, cc.map(|c| c as u8)).map_err(|e| Error::from_reason(e.to_string()))
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
  pub fn now(name: String, email: String) -> Result<Signature> {
    let sig = git2::Signature::now(&name, &email).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Signature {
      name: sig.name().ok().map(|s| s.to_string()),
      email: sig.email().ok().map(|s| s.to_string()),
    })
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
pub struct TreeEntry {
  oid: String,
  name: Option<String>,
  filemode: i32,
  kind: Option<i32>,
}

#[napi]
impl TreeEntry {
  #[napi]
  pub fn id(&self) -> String {
    self.oid.clone()
  }

  #[napi]
  pub fn name(&self) -> Option<String> {
    self.name.clone()
  }

  #[napi]
  pub fn filemode(&self) -> i32 {
    self.filemode
  }

  #[napi]
  pub fn kind(&self) -> Option<i32> {
    self.kind
  }
}

#[napi]
pub struct StatusEntry {
  path: Option<String>,
  status: u32,
}

#[napi]
impl StatusEntry {
  #[napi]
  pub fn path(&self) -> Option<String> {
    self.path.clone()
  }

  #[napi]
  pub fn status(&self) -> u32 {
    self.status
  }
}

#[napi]
pub struct Tree {
  repo: Arc<Mutex<git2::Repository>>,
  oid: git2::Oid,
}

#[napi]
impl Tree {
  #[napi]
  pub fn id(&self) -> String {
    self.oid.to_string()
  }

  #[napi]
  pub fn len(&self) -> Result<u32> {
    let repo = self.repo.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let tree = repo.find_tree(self.oid).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(tree.len() as u32)
  }

  #[napi]
  pub fn get(&self, index: u32) -> Result<Option<TreeEntry>> {
    let repo = self.repo.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let tree = repo.find_tree(self.oid).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(tree.get(index as usize).map(|entry| TreeEntry {
      oid: entry.id().to_string(),
      name: entry.name().ok().map(|s| s.to_string()),
      filemode: entry.filemode(),
      kind: entry.kind().map(|k| k as i32),
    }))
  }

  #[napi]
  pub fn get_by_path(&self, path: String) -> Result<Option<TreeEntry>> {
    let repo = self.repo.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let tree = repo.find_tree(self.oid).map_err(|e| Error::from_reason(e.to_string()))?;
    match tree.get_path(std::path::Path::new(&path)) {
      Ok(entry) => Ok(Some(TreeEntry {
        oid: entry.id().to_string(),
        name: entry.name().ok().map(|s| s.to_string()),
        filemode: entry.filemode(),
        kind: entry.kind().map(|k| k as i32),
      })),
      Err(_) => Ok(None),
    }
  }
}

#[napi]
pub struct Blob {
  repo: Arc<Mutex<git2::Repository>>,
  oid: git2::Oid,
}

#[napi]
impl Blob {
  #[napi]
  pub fn id(&self) -> String {
    self.oid.to_string()
  }

  #[napi]
  pub fn content(&self) -> Result<Buffer> {
    let repo = self.repo.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let blob = repo.find_blob(self.oid).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Buffer::from(blob.content()))
  }

  #[napi]
  pub fn is_binary(&self) -> Result<bool> {
    let repo = self.repo.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let blob = repo.find_blob(self.oid).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(blob.is_binary())
  }

  #[napi]
  pub fn size(&self) -> Result<u32> {
    let repo = self.repo.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let blob = repo.find_blob(self.oid).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(blob.size() as u32)
  }
}

#[napi]
pub struct Tag {
  repo: Arc<Mutex<git2::Repository>>,
  oid: git2::Oid,
}

#[napi]
impl Tag {
  #[napi]
  pub fn id(&self) -> String {
    self.oid.to_string()
  }

  #[napi]
  pub fn name(&self) -> Result<Option<String>> {
    let repo = self.repo.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let tag = repo.find_tag(self.oid).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(tag.name().ok().map(|s| s.to_string()))
  }

  #[napi]
  pub fn message(&self) -> Result<Option<String>> {
    let repo = self.repo.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let tag = repo.find_tag(self.oid).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(tag.message().ok().flatten().map(|s| s.to_string()))
  }

  #[napi]
  pub fn target_id(&self) -> Result<String> {
    let repo = self.repo.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let tag = repo.find_tag(self.oid).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(tag.target_id().to_string())
  }
}

#[napi]
pub struct Commit {
  repo: Arc<Mutex<git2::Repository>>,
  oid: git2::Oid,
}

#[napi]
impl Commit {
  #[napi]
  pub fn id(&self) -> String {
    self.oid.to_string()
  }

  #[napi]
  pub fn message(&self) -> Result<Option<String>> {
    let repo = self.repo.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let commit = repo.find_commit(self.oid).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(commit.message().ok().map(|s| s.to_string()))
  }

  #[napi]
  pub fn summary(&self) -> Result<Option<String>> {
    let repo = self.repo.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let commit = repo.find_commit(self.oid).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(commit.summary().ok().flatten().map(|s| s.to_string()))
  }

  #[napi]
  pub fn author(&self) -> Result<Signature> {
    let repo = self.repo.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let commit = repo.find_commit(self.oid).map_err(|e| Error::from_reason(e.to_string()))?;
    let sig = commit.author();
    Ok(Signature {
      name: sig.name().ok().map(|s| s.to_string()),
      email: sig.email().ok().map(|s| s.to_string()),
    })
  }

  #[napi]
  pub fn committer(&self) -> Result<Signature> {
    let repo = self.repo.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let commit = repo.find_commit(self.oid).map_err(|e| Error::from_reason(e.to_string()))?;
    let sig = commit.committer();
    Ok(Signature {
      name: sig.name().ok().map(|s| s.to_string()),
      email: sig.email().ok().map(|s| s.to_string()),
    })
  }

  #[napi]
  pub fn tree(&self) -> Result<Tree> {
    let repo = self.repo.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let commit = repo.find_commit(self.oid).map_err(|e| Error::from_reason(e.to_string()))?;
    let tree = commit.tree().map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Tree {
      repo: self.repo.clone(),
      oid: tree.id(),
    })
  }

  #[napi]
  pub fn parent_count(&self) -> Result<u32> {
    let repo = self.repo.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let commit = repo.find_commit(self.oid).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(commit.parent_count() as u32)
  }

  #[napi]
  pub fn parent_id(&self, i: u32) -> Result<String> {
    let repo = self.repo.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let commit = repo.find_commit(self.oid).map_err(|e| Error::from_reason(e.to_string()))?;
    let parent_id = commit.parent_id(i as usize).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(parent_id.to_string())
  }

  #[napi]
  pub fn parent(&self, i: u32) -> Result<Commit> {
    let repo = self.repo.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let commit = repo.find_commit(self.oid).map_err(|e| Error::from_reason(e.to_string()))?;
    let parent = commit.parent(i as usize).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Commit {
      repo: self.repo.clone(),
      oid: parent.id(),
    })
  }
}

#[napi]
pub struct Reference {
  repo: Arc<Mutex<git2::Repository>>,
  name: String,
}

#[napi]
impl Reference {
  #[napi]
  pub fn name(&self) -> Option<String> {
    Some(self.name.clone())
  }

  #[napi]
  pub fn target(&self) -> Result<Option<String>> {
    let repo = self.repo.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let reference = repo.find_reference(&self.name).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(reference.target().map(|oid| oid.to_string()))
  }

  #[napi]
  pub fn symbolic_target(&self) -> Result<Option<String>> {
    let repo = self.repo.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let reference = repo.find_reference(&self.name).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(reference.symbolic_target().ok().flatten().map(|s| s.to_string()))
  }

  #[napi]
  pub fn is_branch(&self) -> Result<bool> {
    let repo = self.repo.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let reference = repo.find_reference(&self.name).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(reference.is_branch())
  }

  #[napi]
  pub fn is_remote(&self) -> Result<bool> {
    let repo = self.repo.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let reference = repo.find_reference(&self.name).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(reference.is_remote())
  }

  #[napi]
  pub fn is_tag(&self) -> Result<bool> {
    let repo = self.repo.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let reference = repo.find_reference(&self.name).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(reference.is_tag())
  }
}

#[napi]
pub struct Branch {
  repo: Arc<Mutex<git2::Repository>>,
  name: String,
  branch_type: git2::BranchType,
}

#[napi]
impl Branch {
  #[napi]
  pub fn name(&self) -> Result<Option<String>> {
    let repo = self.repo.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let branch = repo.find_branch(&self.name, self.branch_type).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(branch.name().ok().flatten().map(|s| s.to_string()))
  }

  #[napi]
  pub fn is_head(&self) -> Result<bool> {
    let repo = self.repo.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let branch = repo.find_branch(&self.name, self.branch_type).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(branch.is_head())
  }

  #[napi]
  pub fn get(&self) -> Reference {
    Reference {
      repo: self.repo.clone(),
      name: if self.branch_type == git2::BranchType::Local {
        format!("refs/heads/{}", self.name)
      } else {
        format!("refs/remotes/{}", self.name)
      },
    }
  }
}

#[napi]
pub struct Object {
  repo: Arc<Mutex<git2::Repository>>,
  oid: git2::Oid,
  kind: Option<git2::ObjectType>,
}

#[napi]
impl Object {
  #[napi]
  pub fn id(&self) -> String {
    self.oid.to_string()
  }

  #[napi]
  pub fn kind(&self) -> Option<i32> {
    self.kind.map(|k| k as i32)
  }

  #[napi]
  pub fn short_id(&self) -> Result<Option<String>> {
    let repo = self.repo.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let obj = repo.find_object(self.oid, self.kind).map_err(|e| Error::from_reason(e.to_string()))?;
    let buf = obj.short_id().map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(buf.as_str().ok().map(|s| s.to_string()))
  }
}

#[napi]
pub struct Revwalk {
  repo: Arc<Mutex<git2::Repository>>,
  oids: Arc<Mutex<Vec<git2::Oid>>>,
  cursor: Arc<Mutex<usize>>,
}

#[napi]
impl Revwalk {
  #[napi]
  pub fn push(&self, oid: String) -> Result<()> {
    let oid_parsed = git2::Oid::from_str(&oid).map_err(|e| Error::from_reason(e.to_string()))?;
    let repo = self.repo.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let mut walk = repo.revwalk().map_err(|e| Error::from_reason(e.to_string()))?;
    walk.push(oid_parsed).map_err(|e| Error::from_reason(e.to_string()))?;
    let mut oids = self.oids.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    oids.clear();
    for id in walk {
      if let Ok(id) = id {
        oids.push(id);
      }
    }
    let mut cursor = self.cursor.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    *cursor = 0;
    Ok(())
  }

  #[napi]
  pub fn push_head(&self) -> Result<()> {
    let repo = self.repo.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let mut walk = repo.revwalk().map_err(|e| Error::from_reason(e.to_string()))?;
    walk.push_head().map_err(|e| Error::from_reason(e.to_string()))?;
    let mut oids = self.oids.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    oids.clear();
    for id in walk {
      if let Ok(id) = id {
        oids.push(id);
      }
    }
    let mut cursor = self.cursor.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    *cursor = 0;
    Ok(())
  }

  #[napi]
  pub fn reset(&self) -> Result<()> {
    let mut oids = self.oids.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    oids.clear();
    let mut cursor = self.cursor.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    *cursor = 0;
    Ok(())
  }

  #[napi]
  pub fn next(&self) -> Result<Option<String>> {
    let oids = self.oids.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let mut cursor = self.cursor.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    if *cursor < oids.len() {
      let oid = oids[*cursor];
      *cursor += 1;
      Ok(Some(oid.to_string()))
    } else {
      Ok(None)
    }
  }
}

#[napi]
pub struct Index {
  repo: Arc<Mutex<git2::Repository>>,
}

#[napi]
impl Index {
  #[napi]
  pub fn add_path(&self, path: String) -> Result<()> {
    let repo = self.repo.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let mut index = repo.index().map_err(|e| Error::from_reason(e.to_string()))?;
    index.add_path(std::path::Path::new(&path)).map_err(|e| Error::from_reason(e.to_string()))?;
    index.write().map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(())
  }

  #[napi]
  pub fn remove_path(&self, path: String) -> Result<()> {
    let repo = self.repo.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let mut index = repo.index().map_err(|e| Error::from_reason(e.to_string()))?;
    index.remove_path(std::path::Path::new(&path)).map_err(|e| Error::from_reason(e.to_string()))?;
    index.write().map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(())
  }

  #[napi]
  pub fn write(&self) -> Result<()> {
    let repo = self.repo.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let mut index = repo.index().map_err(|e| Error::from_reason(e.to_string()))?;
    index.write().map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(())
  }

  #[napi]
  pub fn write_tree(&self) -> Result<String> {
    let repo = self.repo.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let mut index = repo.index().map_err(|e| Error::from_reason(e.to_string()))?;
    let oid = index.write_tree().map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(oid.to_string())
  }
}

#[napi]
pub struct Repository {
  inner: Arc<Mutex<git2::Repository>>,
}

#[napi]
impl Repository {
  #[napi]
  pub fn init(path: String) -> Result<Repository> {
    let repo = git2::Repository::init(&path).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Repository {
      inner: Arc::new(Mutex::new(repo)),
    })
  }

  #[napi]
  pub fn init_bare(path: String) -> Result<Repository> {
    let repo = git2::Repository::init_bare(&path).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Repository {
      inner: Arc::new(Mutex::new(repo)),
    })
  }

  #[napi]
  pub fn open(path: String) -> Result<Repository> {
    let repo = git2::Repository::open(&path).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Repository {
      inner: Arc::new(Mutex::new(repo)),
    })
  }

  #[napi]
  pub fn open_bare(path: String) -> Result<Repository> {
    let repo = git2::Repository::open_bare(&path).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Repository {
      inner: Arc::new(Mutex::new(repo)),
    })
  }

  #[napi]
  pub fn clone(url: String, path: String) -> Result<Repository> {
    let repo = git2::Repository::clone(&url, &path).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Repository {
      inner: Arc::new(Mutex::new(repo)),
    })
  }

  #[napi]
  pub fn discover(path: String) -> Result<Repository> {
    let repo = git2::Repository::discover(&path).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Repository {
      inner: Arc::new(Mutex::new(repo)),
    })
  }

  #[napi]
  pub fn is_bare(&self) -> Result<bool> {
    let repo = self.inner.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(repo.is_bare())
  }

  #[napi]
  pub fn is_empty(&self) -> Result<bool> {
    let repo = self.inner.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    repo.is_empty().map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn is_shallow(&self) -> Result<bool> {
    let repo = self.inner.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(repo.is_shallow())
  }

  #[napi]
  pub fn is_worktree(&self) -> Result<bool> {
    let repo = self.inner.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(repo.is_worktree())
  }

  #[napi]
  pub fn path(&self) -> Result<String> {
    let repo = self.inner.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(repo.path().to_string_lossy().to_string())
  }

  #[napi]
  pub fn workdir(&self) -> Result<Option<String>> {
    let repo = self.inner.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(repo.workdir().map(|p| p.to_string_lossy().to_string()))
  }

  #[napi]
  pub fn state(&self) -> Result<i32> {
    let repo = self.inner.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(repo.state() as i32)
  }

  #[napi]
  pub fn head_name(&self) -> Result<String> {
    let repo = self.inner.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let head = repo.head().map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(head.name().unwrap_or("").to_string())
  }

  #[napi]
  pub fn head_detached(&self) -> Result<bool> {
    let repo = self.inner.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    repo.head_detached().map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn add_ignore_rule(&self, rules: String) -> Result<()> {
    let repo = self.inner.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    repo.add_ignore_rule(&rules).map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn clear_ignore_rules(&self) -> Result<()> {
    let repo = self.inner.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    repo.clear_ignore_rules().map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn is_path_ignored(&self, path: String) -> Result<bool> {
    let repo = self.inner.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    repo.is_path_ignored(&path).map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn remotes(&self) -> Result<Vec<String>> {
    let repo = self.inner.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let arr = repo.remotes().map_err(|e| Error::from_reason(e.to_string()))?;
    let mut list = Vec::new();
    for r in arr.iter() {
      if let Ok(Some(name)) = r {
        list.push(name.to_string());
      }
    }
    Ok(list)
  }

  #[napi]
  pub fn tag_names(&self, pattern: Option<String>) -> Result<Vec<String>> {
    let repo = self.inner.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let arr = repo.tag_names(pattern.as_deref()).map_err(|e| Error::from_reason(e.to_string()))?;
    let mut list = Vec::new();
    for t in arr.iter() {
      if let Ok(Some(name)) = t {
        list.push(name.to_string());
      }
    }
    Ok(list)
  }

  #[napi]
  pub fn statuses(&self) -> Result<Vec<StatusEntry>> {
    let repo = self.inner.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let statuses = repo.statuses(None).map_err(|e| Error::from_reason(e.to_string()))?;
    let mut list = Vec::new();
    for entry in statuses.iter() {
      list.push(StatusEntry {
        path: entry.path().ok().map(|s| s.to_string()),
        status: entry.status().bits(),
      });
    }
    Ok(list)
  }

  #[napi]
  pub fn find_commit(&self, oid: String) -> Result<Commit> {
    let oid_parsed = git2::Oid::from_str(&oid).map_err(|e| Error::from_reason(e.to_string()))?;
    let repo = self.inner.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let _ = repo.find_commit(oid_parsed).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Commit {
      repo: self.inner.clone(),
      oid: oid_parsed,
    })
  }

  #[napi]
  pub fn find_tree(&self, oid: String) -> Result<Tree> {
    let oid_parsed = git2::Oid::from_str(&oid).map_err(|e| Error::from_reason(e.to_string()))?;
    let repo = self.inner.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let _ = repo.find_tree(oid_parsed).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Tree {
      repo: self.inner.clone(),
      oid: oid_parsed,
    })
  }

  #[napi]
  pub fn find_blob(&self, oid: String) -> Result<Blob> {
    let oid_parsed = git2::Oid::from_str(&oid).map_err(|e| Error::from_reason(e.to_string()))?;
    let repo = self.inner.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let _ = repo.find_blob(oid_parsed).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Blob {
      repo: self.inner.clone(),
      oid: oid_parsed,
    })
  }

  #[napi]
  pub fn find_tag(&self, oid: String) -> Result<Tag> {
    let oid_parsed = git2::Oid::from_str(&oid).map_err(|e| Error::from_reason(e.to_string()))?;
    let repo = self.inner.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let _ = repo.find_tag(oid_parsed).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Tag {
      repo: self.inner.clone(),
      oid: oid_parsed,
    })
  }

  #[napi]
  pub fn find_reference(&self, name: String) -> Result<Reference> {
    let repo = self.inner.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let _ = repo.find_reference(&name).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Reference {
      repo: self.inner.clone(),
      name,
    })
  }

  #[napi]
  pub fn find_branch(&self, name: String, branch_type: i32) -> Result<Branch> {
    let b_type = if branch_type == 2 {
      git2::BranchType::Remote
    } else {
      git2::BranchType::Local
    };
    let repo = self.inner.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let _ = repo.find_branch(&name, b_type).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Branch {
      repo: self.inner.clone(),
      name,
      branch_type: b_type,
    })
  }

  #[napi]
  pub fn create_branch(&self, name: String, commit: &Commit, force: bool) -> Result<Branch> {
    let repo = self.inner.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let target_commit = repo.find_commit(commit.oid).map_err(|e| Error::from_reason(e.to_string()))?;
    let _ = repo.branch(&name, &target_commit, force).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Branch {
      repo: self.inner.clone(),
      name,
      branch_type: git2::BranchType::Local,
    })
  }

  #[napi]
  pub fn revparse_single(&self, spec: String) -> Result<Object> {
    let repo = self.inner.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let obj = repo.revparse_single(&spec).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Object {
      repo: self.inner.clone(),
      oid: obj.id(),
      kind: obj.kind(),
    })
  }

  #[napi]
  pub fn revwalk(&self) -> Result<Revwalk> {
    Ok(Revwalk {
      repo: self.inner.clone(),
      oids: Arc::new(Mutex::new(Vec::new())),
      cursor: Arc::new(Mutex::new(0)),
    })
  }

  #[napi]
  pub fn index(&self) -> Result<Index> {
    let repo = self.inner.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let _ = repo.index().map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Index {
      repo: self.inner.clone(),
    })
  }

  #[napi]
  pub fn signature(&self) -> Result<Signature> {
    let repo = self.inner.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let sig = repo.signature().map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Signature {
      name: sig.name().ok().map(|s| s.to_string()),
      email: sig.email().ok().map(|s| s.to_string()),
    })
  }

  #[napi]
  pub fn create_commit(
    &self,
    update_ref: Option<String>,
    author: &Signature,
    committer: &Signature,
    message: String,
    tree: &Tree,
    parents: Vec<&Commit>,
  ) -> Result<String> {
    let repo = self.inner.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let author_sig = git2::Signature::now(
      author.name.as_deref().unwrap_or(""),
      author.email.as_deref().unwrap_or(""),
    ).map_err(|e| Error::from_reason(e.to_string()))?;
    let committer_sig = git2::Signature::now(
      committer.name.as_deref().unwrap_or(""),
      committer.email.as_deref().unwrap_or(""),
    ).map_err(|e| Error::from_reason(e.to_string()))?;
    let tree_obj = repo.find_tree(tree.oid).map_err(|e| Error::from_reason(e.to_string()))?;

    let mut parent_commits = Vec::new();
    for p in parents {
      let pc = repo.find_commit(p.oid).map_err(|e| Error::from_reason(e.to_string()))?;
      parent_commits.push(pc);
    }
    let parent_refs: Vec<&git2::Commit> = parent_commits.iter().collect();

    let oid = repo.commit(
      update_ref.as_deref(),
      &author_sig,
      &committer_sig,
      &message,
      &tree_obj,
      &parent_refs,
    ).map_err(|e| Error::from_reason(e.to_string()))?;

    Ok(oid.to_string())
  }

  #[napi]
  pub fn create_tag(
    &self,
    name: String,
    target: &Object,
    tagger: &Signature,
    message: String,
    force: bool,
  ) -> Result<String> {
    let repo = self.inner.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let tagger_sig = git2::Signature::now(
      tagger.name.as_deref().unwrap_or(""),
      tagger.email.as_deref().unwrap_or(""),
    ).map_err(|e| Error::from_reason(e.to_string()))?;
    let target_obj = repo.find_object(target.oid, target.kind).map_err(|e| Error::from_reason(e.to_string()))?;

    let oid = repo.tag(&name, &target_obj, &tagger_sig, &message, force)
      .map_err(|e| Error::from_reason(e.to_string()))?;

    Ok(oid.to_string())
  }

  #[napi]
  pub fn create_tag_lightweight(
    &self,
    name: String,
    target: &Object,
    force: bool,
  ) -> Result<String> {
    let repo = self.inner.lock().map_err(|e| Error::from_reason(e.to_string()))?;
    let target_obj = repo.find_object(target.oid, target.kind).map_err(|e| Error::from_reason(e.to_string()))?;

    let oid = repo.tag_lightweight(&name, &target_obj, force)
      .map_err(|e| Error::from_reason(e.to_string()))?;

    Ok(oid.to_string())
  }
}
