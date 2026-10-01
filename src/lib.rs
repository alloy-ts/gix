use napi::bindgen_prelude::*;
use napi_derive::napi;

// -----------------------------------------------------------------------------
// Enums
// -----------------------------------------------------------------------------

#[napi]
pub enum RepositoryState {
  Clean,
  Merge,
  Revert,
  RevertSequence,
  CherryPick,
  CherryPickSequence,
  Bisect,
  Rebase,
  RebaseInteractive,
  RebaseMerge,
  ApplyMailbox,
  ApplyMailboxOrRebase,
}

impl From<git2::RepositoryState> for RepositoryState {
  fn from(state: git2::RepositoryState) -> Self {
    match state {
      git2::RepositoryState::Clean => RepositoryState::Clean,
      git2::RepositoryState::Merge => RepositoryState::Merge,
      git2::RepositoryState::Revert => RepositoryState::Revert,
      git2::RepositoryState::RevertSequence => RepositoryState::RevertSequence,
      git2::RepositoryState::CherryPick => RepositoryState::CherryPick,
      git2::RepositoryState::CherryPickSequence => RepositoryState::CherryPickSequence,
      git2::RepositoryState::Bisect => RepositoryState::Bisect,
      git2::RepositoryState::Rebase => RepositoryState::Rebase,
      git2::RepositoryState::RebaseInteractive => RepositoryState::RebaseInteractive,
      git2::RepositoryState::RebaseMerge => RepositoryState::RebaseMerge,
      git2::RepositoryState::ApplyMailbox => RepositoryState::ApplyMailbox,
      git2::RepositoryState::ApplyMailboxOrRebase => RepositoryState::ApplyMailboxOrRebase,
    }
  }
}

#[napi]
pub enum BranchType {
  Local,
  Remote,
}

impl From<BranchType> for git2::BranchType {
  fn from(bt: BranchType) -> Self {
    match bt {
      BranchType::Local => git2::BranchType::Local,
      BranchType::Remote => git2::BranchType::Remote,
    }
  }
}

impl From<git2::BranchType> for BranchType {
  fn from(bt: git2::BranchType) -> Self {
    match bt {
      git2::BranchType::Local => BranchType::Local,
      git2::BranchType::Remote => BranchType::Remote,
    }
  }
}

#[napi]
pub enum ObjectType {
  Any,
  Commit,
  Tree,
  Blob,
  Tag,
}

impl From<ObjectType> for git2::ObjectType {
  fn from(ot: ObjectType) -> Self {
    match ot {
      ObjectType::Any => git2::ObjectType::Any,
      ObjectType::Commit => git2::ObjectType::Commit,
      ObjectType::Tree => git2::ObjectType::Tree,
      ObjectType::Blob => git2::ObjectType::Blob,
      ObjectType::Tag => git2::ObjectType::Tag,
    }
  }
}

impl From<git2::ObjectType> for ObjectType {
  fn from(ot: git2::ObjectType) -> Self {
    match ot {
      git2::ObjectType::Any => ObjectType::Any,
      git2::ObjectType::Commit => ObjectType::Commit,
      git2::ObjectType::Tree => ObjectType::Tree,
      git2::ObjectType::Blob => ObjectType::Blob,
      git2::ObjectType::Tag => ObjectType::Tag,
    }
  }
}

#[napi]
pub enum ResetType {
  Soft,
  Mixed,
  Hard,
}

impl From<ResetType> for git2::ResetType {
  fn from(rt: ResetType) -> Self {
    match rt {
      ResetType::Soft => git2::ResetType::Soft,
      ResetType::Mixed => git2::ResetType::Mixed,
      ResetType::Hard => git2::ResetType::Hard,
    }
  }
}

#[derive(Clone, Copy)]
#[napi]
pub enum Delta {
  Unmodified,
  Added,
  Deleted,
  Modified,
  Renamed,
  Copied,
  Ignored,
  Untracked,
  Typechange,
  Unreadable,
  Conflicted,
}

impl From<git2::Delta> for Delta {
  fn from(d: git2::Delta) -> Self {
    match d {
      git2::Delta::Unmodified => Delta::Unmodified,
      git2::Delta::Added => Delta::Added,
      git2::Delta::Deleted => Delta::Deleted,
      git2::Delta::Modified => Delta::Modified,
      git2::Delta::Renamed => Delta::Renamed,
      git2::Delta::Copied => Delta::Copied,
      git2::Delta::Ignored => Delta::Ignored,
      git2::Delta::Untracked => Delta::Untracked,
      git2::Delta::Typechange => Delta::Typechange,
      git2::Delta::Unreadable => Delta::Unreadable,
      git2::Delta::Conflicted => Delta::Conflicted,
    }
  }
}

// -----------------------------------------------------------------------------
// Struct Wrappers
// -----------------------------------------------------------------------------

#[napi]
pub struct Signature {
  name: String,
  email: String,
  time_seconds: i64,
}

#[napi]
impl Signature {
  #[napi(factory)]
  pub fn now(name: String, email: String) -> Result<Signature> {
    let sig = git2::Signature::now(&name, &email)
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to create signature: {e}")))?;
    Ok(Signature {
      name: sig.name().unwrap_or("").to_string(),
      email: sig.email().unwrap_or("").to_string(),
      time_seconds: sig.when().seconds(),
    })
  }

  #[napi(getter)]
  pub fn name(&self) -> String {
    self.name.clone()
  }

  #[napi(getter)]
  pub fn email(&self) -> String {
    self.email.clone()
  }

  #[napi(getter)]
  pub fn time_seconds(&self) -> i64 {
    self.time_seconds
  }
}

#[napi]
pub struct Oid {
  inner: git2::Oid,
}

#[napi]
impl Oid {
  #[napi(factory)]
  pub fn from_str(s: String) -> Result<Oid> {
    let oid = git2::Oid::from_str(&s)
      .map_err(|e| Error::new(Status::GenericFailure, format!("invalid oid: {e}")))?;
    Ok(Oid { inner: oid })
  }

  #[napi]
  pub fn to_string(&self) -> String {
    self.inner.to_string()
  }
}

#[napi]
pub struct Commit {
  inner: git2::Commit<'static>,
}

#[napi]
impl Commit {
  #[napi]
  pub fn id(&self) -> String {
    self.inner.id().to_string()
  }

  #[napi]
  pub fn message(&self) -> Option<String> {
    self.inner.message().ok().map(|s| s.to_string())
  }

  #[napi]
  pub fn summary(&self) -> Option<String> {
    self.inner.summary().ok().flatten().map(|s| s.to_string())
  }

  #[napi]
  pub fn time(&self) -> i64 {
    self.inner.time().seconds()
  }

  #[napi]
  pub fn author(&self) -> Signature {
    let sig = self.inner.author();
    Signature {
      name: sig.name().unwrap_or("").to_string(),
      email: sig.email().unwrap_or("").to_string(),
      time_seconds: sig.when().seconds(),
    }
  }

  #[napi]
  pub fn committer(&self) -> Signature {
    let sig = self.inner.committer();
    Signature {
      name: sig.name().unwrap_or("").to_string(),
      email: sig.email().unwrap_or("").to_string(),
      time_seconds: sig.when().seconds(),
    }
  }

  #[napi]
  pub fn tree_id(&self) -> String {
    self.inner.tree_id().to_string()
  }

  #[napi]
  pub fn parent_count(&self) -> u32 {
    self.inner.parent_count() as u32
  }

  #[napi]
  pub fn parent_id(&self, i: u32) -> Result<String> {
    self.inner
      .parent_id(i as usize)
      .map(|oid| oid.to_string())
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to get parent_id: {e}")))
  }
}

#[napi]
pub struct Tree {
  inner: git2::Tree<'static>,
}

#[napi]
impl Tree {
  #[napi]
  pub fn id(&self) -> String {
    self.inner.id().to_string()
  }

  #[napi]
  pub fn len(&self) -> u32 {
    self.inner.len() as u32
  }

  #[napi]
  pub fn is_empty(&self) -> bool {
    self.inner.is_empty()
  }

  #[napi]
  pub fn get_name(&self, filename: String) -> Option<TreeEntry> {
    self.inner.get_name(&filename).map(|e| TreeEntry {
      id: e.id().to_string(),
      name: e.name().ok().map(|s| s.to_string()),
      filemode: e.filemode(),
    })
  }
}

#[napi]
pub struct TreeEntry {
  id: String,
  name: Option<String>,
  filemode: i32,
}

#[napi]
impl TreeEntry {
  #[napi(getter)]
  pub fn id(&self) -> String {
    self.id.clone()
  }

  #[napi(getter)]
  pub fn name(&self) -> Option<String> {
    self.name.clone()
  }

  #[napi(getter)]
  pub fn filemode(&self) -> i32 {
    self.filemode
  }
}

#[napi]
pub struct Blob {
  inner: git2::Blob<'static>,
}

#[napi]
impl Blob {
  #[napi]
  pub fn id(&self) -> String {
    self.inner.id().to_string()
  }

  #[napi]
  pub fn content(&self) -> Buffer {
    Buffer::from(self.inner.content())
  }

  #[napi]
  pub fn is_binary(&self) -> bool {
    self.inner.is_binary()
  }

  #[napi]
  pub fn size(&self) -> u32 {
    self.inner.size() as u32
  }
}

#[napi]
pub struct Tag {
  inner: git2::Tag<'static>,
}

#[napi]
impl Tag {
  #[napi]
  pub fn id(&self) -> String {
    self.inner.id().to_string()
  }

  #[napi]
  pub fn name(&self) -> Option<String> {
    self.inner.name().ok().map(|s| s.to_string())
  }

  #[napi]
  pub fn message(&self) -> Option<String> {
    self.inner.message().ok().flatten().map(|s| s.to_string())
  }

  #[napi]
  pub fn target_id(&self) -> String {
    self.inner.target_id().to_string()
  }
}

#[napi]
pub struct Branch {
  inner: git2::Branch<'static>,
}

#[napi]
impl Branch {
  #[napi]
  pub fn name(&self) -> Result<Option<String>> {
    self.inner
      .name()
      .map(|opt| opt.map(|s| s.to_string()))
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to get branch name: {e}")))
  }

  #[napi]
  pub fn is_head(&self) -> bool {
    self.inner.is_head()
  }

  #[napi]
  pub fn get_reference(&self) -> Reference {
    let r = self.inner.get();
    unsafe {
      Reference {
        inner: std::mem::transmute::<git2::Reference<'_>, git2::Reference<'static>>(r.clone()),
      }
    }
  }
}

#[napi]
pub struct Reference {
  inner: git2::Reference<'static>,
}

#[napi]
impl Reference {
  #[napi]
  pub fn is_valid_name(refname: String) -> bool {
    git2::Reference::is_valid_name(&refname)
  }

  #[napi]
  pub fn name(&self) -> Option<String> {
    self.inner.name().ok().map(|s| s.to_string())
  }

  #[napi]
  pub fn shorthand(&self) -> Option<String> {
    self.inner.shorthand().ok().map(|s| s.to_string())
  }

  #[napi]
  pub fn target(&self) -> Option<String> {
    self.inner.target().map(|oid| oid.to_string())
  }

  #[napi]
  pub fn target_peel(&self) -> Option<String> {
    self.inner.target_peel().map(|oid| oid.to_string())
  }

  #[napi]
  pub fn symbolic_target(&self) -> Option<String> {
    self.inner.symbolic_target().ok().flatten().map(|s| s.to_string())
  }

  #[napi]
  pub fn resolve(&self) -> Result<Reference> {
    let resolved = self.inner.resolve()
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to resolve reference: {e}")))?;
    unsafe {
      Ok(Reference {
        inner: std::mem::transmute::<git2::Reference<'_>, git2::Reference<'static>>(resolved),
      })
    }
  }

  #[napi]
  pub fn rename(&mut self, new_name: String, force: bool, log_message: String) -> Result<Reference> {
    let renamed = self.inner.rename(&new_name, force, &log_message)
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to rename reference: {e}")))?;
    unsafe {
      Ok(Reference {
        inner: std::mem::transmute::<git2::Reference<'_>, git2::Reference<'static>>(renamed),
      })
    }
  }

  #[napi]
  pub fn delete(&mut self) -> Result<()> {
    self.inner.delete()
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to delete reference: {e}")))
  }

  #[napi]
  pub fn set_target(&mut self, target_oid_str: String, log_message: String) -> Result<Reference> {
    let oid = git2::Oid::from_str(&target_oid_str)
      .map_err(|e| Error::new(Status::GenericFailure, format!("invalid oid: {e}")))?;
    let updated = self.inner.set_target(oid, &log_message)
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to set_target: {e}")))?;
    unsafe {
      Ok(Reference {
        inner: std::mem::transmute::<git2::Reference<'_>, git2::Reference<'static>>(updated),
      })
    }
  }

  #[napi]
  pub fn symbolic_set_target(&mut self, target: String, log_message: String) -> Result<Reference> {
    let updated = self.inner.symbolic_set_target(&target, &log_message)
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to symbolic_set_target: {e}")))?;
    unsafe {
      Ok(Reference {
        inner: std::mem::transmute::<git2::Reference<'_>, git2::Reference<'static>>(updated),
      })
    }
  }

  #[napi]
  pub fn is_branch(&self) -> bool {
    self.inner.is_branch()
  }

  #[napi]
  pub fn is_remote(&self) -> bool {
    self.inner.is_remote()
  }

  #[napi]
  pub fn is_tag(&self) -> bool {
    self.inner.is_tag()
  }

  #[napi]
  pub fn is_note(&self) -> bool {
    self.inner.is_note()
  }
}

#[napi]
pub struct Index {
  inner: git2::Index,
}

#[napi]
impl Index {
  #[napi]
  pub fn add_path(&mut self, path: String) -> Result<()> {
    self.inner
      .add_path(std::path::Path::new(&path))
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to add_path: {e}")))
  }

  #[napi]
  pub fn remove_path(&mut self, path: String) -> Result<()> {
    self.inner
      .remove_path(std::path::Path::new(&path))
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to remove_path: {e}")))
  }

  #[napi]
  pub fn write(&mut self) -> Result<()> {
    self.inner
      .write()
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to write index: {e}")))
  }

  #[napi]
  pub fn write_tree(&mut self) -> Result<String> {
    self.inner
      .write_tree()
      .map(|oid| oid.to_string())
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to write_tree: {e}")))
  }

  #[napi]
  pub fn len(&self) -> u32 {
    self.inner.len() as u32
  }

  #[napi]
  pub fn is_empty(&self) -> bool {
    self.inner.is_empty()
  }

  #[napi]
  pub fn read(&mut self, force: bool) -> Result<()> {
    self.inner
      .read(force)
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to read index: {e}")))
  }
}

#[napi]
pub struct StatusEntry {
  path: Option<String>,
  status_bits: u32,
}

#[napi]
impl StatusEntry {
  #[napi(getter)]
  pub fn path(&self) -> Option<String> {
    self.path.clone()
  }

  #[napi(getter)]
  pub fn status(&self) -> u32 {
    self.status_bits
  }
}

#[napi]
pub struct DiffFile {
  path: Option<String>,
  id: String,
  size: u32,
}

#[napi]
impl DiffFile {
  #[napi(getter)]
  pub fn path(&self) -> Option<String> {
    self.path.clone()
  }

  #[napi(getter)]
  pub fn id(&self) -> String {
    self.id.clone()
  }

  #[napi(getter)]
  pub fn size(&self) -> u32 {
    self.size
  }
}

#[napi]
pub struct DiffDelta {
  status: Delta,
  old_file: DiffFile,
  new_file: DiffFile,
}

#[napi]
impl DiffDelta {
  #[napi(getter)]
  pub fn status(&self) -> Delta {
    self.status
  }

  #[napi(getter)]
  pub fn old_file(&self) -> DiffFile {
    DiffFile {
      path: self.old_file.path.clone(),
      id: self.old_file.id.clone(),
      size: self.old_file.size,
    }
  }

  #[napi(getter)]
  pub fn new_file(&self) -> DiffFile {
    DiffFile {
      path: self.new_file.path.clone(),
      id: self.new_file.id.clone(),
      size: self.new_file.size,
    }
  }
}

#[napi]
pub struct Diff {
  inner: git2::Diff<'static>,
}

#[napi]
impl Diff {
  #[napi]
  pub fn deltas_len(&self) -> u32 {
    self.inner.deltas().count() as u32
  }

  #[napi]
  pub fn get_delta(&self, idx: u32) -> Option<DiffDelta> {
    self.inner.deltas().nth(idx as usize).map(|d| DiffDelta {
      status: Delta::from(d.status()),
      old_file: DiffFile {
        path: d.old_file().path().map(|p| p.to_string_lossy().into_owned()),
        id: d.old_file().id().to_string(),
        size: d.old_file().size() as u32,
      },
      new_file: DiffFile {
        path: d.new_file().path().map(|p| p.to_string_lossy().into_owned()),
        id: d.new_file().id().to_string(),
        size: d.new_file().size() as u32,
      },
    })
  }
}

#[napi]
pub struct Revwalk {
  inner: git2::Revwalk<'static>,
}

#[napi]
impl Revwalk {
  #[napi]
  pub fn push_head(&mut self) -> Result<()> {
    self.inner
      .push_head()
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to push_head: {e}")))
  }

  #[napi]
  pub fn push(&mut self, oid_str: String) -> Result<()> {
    let oid = git2::Oid::from_str(&oid_str)
      .map_err(|e| Error::new(Status::GenericFailure, format!("invalid oid: {e}")))?;
    self.inner
      .push(oid)
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to push oid: {e}")))
  }

  #[napi]
  pub fn hide(&mut self, oid_str: String) -> Result<()> {
    let oid = git2::Oid::from_str(&oid_str)
      .map_err(|e| Error::new(Status::GenericFailure, format!("invalid oid: {e}")))?;
    self.inner
      .hide(oid)
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to hide oid: {e}")))
  }

  #[napi]
  pub fn next(&mut self) -> Result<Option<String>> {
    match self.inner.next() {
      Some(Ok(oid)) => Ok(Some(oid.to_string())),
      Some(Err(e)) => Err(Error::new(Status::GenericFailure, format!("revwalk error: {e}"))),
      None => Ok(None),
    }
  }
}

#[napi]
pub struct Worktree {
  inner: git2::Worktree,
}

#[napi]
impl Worktree {
  #[napi]
  pub fn name(&self) -> Option<String> {
    self.inner.name().ok().flatten().map(|s| s.to_string())
  }

  #[napi]
  pub fn path(&self) -> String {
    self.inner.path().to_string_lossy().into_owned()
  }

  #[napi]
  pub fn validate(&self) -> Result<()> {
    self.inner
      .validate()
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to validate worktree: {e}")))
  }

  #[napi]
  pub fn lock(&self, reason: Option<String>) -> Result<()> {
    self.inner
      .lock(reason.as_deref())
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to lock worktree: {e}")))
  }

  #[napi]
  pub fn unlock(&self) -> Result<()> {
    self.inner
      .unlock()
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to unlock worktree: {e}")))
  }

  #[napi]
  pub fn is_locked(&self) -> Result<bool> {
    self.inner
      .is_locked()
      .map(|s| match s {
        git2::WorktreeLockStatus::Locked(_) => true,
        git2::WorktreeLockStatus::Unlocked => false,
      })
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to check is_locked: {e}")))
  }
}

#[napi]
pub struct Config {
  inner: git2::Config,
}

#[napi]
impl Config {
  #[napi]
  pub fn get_string(&self, name: String) -> Result<String> {
    self.inner
      .get_string(&name)
      .map_err(|e| Error::new(Status::GenericFailure, format!("config key not found: {e}")))
  }

  #[napi]
  pub fn set_string(&mut self, name: String, value: String) -> Result<()> {
    self.inner
      .set_str(&name, &value)
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to set config string: {e}")))
  }

  #[napi]
  pub fn get_bool(&self, name: String) -> Result<bool> {
    self.inner
      .get_bool(&name)
      .map_err(|e| Error::new(Status::GenericFailure, format!("config key not found: {e}")))
  }

  #[napi]
  pub fn set_bool(&mut self, name: String, value: bool) -> Result<()> {
    self.inner
      .set_bool(&name, value)
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to set config bool: {e}")))
  }

  #[napi]
  pub fn remove(&mut self, name: String) -> Result<()> {
    self.inner
      .remove(&name)
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to remove config key: {e}")))
  }
}

// -----------------------------------------------------------------------------
// Repository Wrapper
// -----------------------------------------------------------------------------

#[napi]
pub struct Repository {
  inner: git2::Repository,
}

#[napi]
impl Repository {
  #[napi(factory)]
  pub fn init(path: String) -> Result<Repository> {
    let repo = git2::Repository::init(&path)
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to init: {e}")))?;
    Ok(Repository { inner: repo })
  }

  #[napi(factory)]
  pub fn init_bare(path: String) -> Result<Repository> {
    let repo = git2::Repository::init_bare(&path)
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to init_bare: {e}")))?;
    Ok(Repository { inner: repo })
  }

  #[napi(factory)]
  pub fn open(path: String) -> Result<Repository> {
    let repo = git2::Repository::open(&path)
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to open: {e}")))?;
    Ok(Repository { inner: repo })
  }

  #[napi(factory)]
  pub fn open_bare(path: String) -> Result<Repository> {
    let repo = git2::Repository::open_bare(&path)
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to open_bare: {e}")))?;
    Ok(Repository { inner: repo })
  }

  #[napi(factory)]
  pub fn discover(path: String) -> Result<Repository> {
    let repo = git2::Repository::discover(&path)
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to discover: {e}")))?;
    Ok(Repository { inner: repo })
  }

  #[napi(factory)]
  pub fn clone(url: String, path: String) -> Result<Repository> {
    let repo = git2::Repository::clone(&url, &path)
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to clone: {e}")))?;
    Ok(Repository { inner: repo })
  }

  #[napi]
  pub fn is_bare(&self) -> bool {
    self.inner.is_bare()
  }

  #[napi]
  pub fn is_empty(&self) -> Result<bool> {
    self.inner
      .is_empty()
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to check is_empty: {e}")))
  }

  #[napi]
  pub fn path(&self) -> String {
    self.inner.path().to_string_lossy().into_owned()
  }

  #[napi]
  pub fn workdir(&self) -> Option<String> {
    self.inner.workdir().map(|p| p.to_string_lossy().into_owned())
  }

  #[napi]
  pub fn state(&self) -> RepositoryState {
    RepositoryState::from(self.inner.state())
  }

  #[napi]
  pub fn head(&self) -> Result<Reference> {
    let ref_ = self.inner.head()
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to get head: {e}")))?;
    unsafe {
      Ok(Reference {
        inner: std::mem::transmute::<git2::Reference<'_>, git2::Reference<'static>>(ref_),
      })
    }
  }

  #[napi]
  pub fn set_head(&self, refname: String) -> Result<()> {
    self.inner
      .set_head(&refname)
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to set_head: {e}")))
  }

  #[napi]
  pub fn head_detached(&self) -> Result<bool> {
    self.inner
      .head_detached()
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to check head_detached: {e}")))
  }

  #[napi]
  pub fn references(&self) -> Result<Vec<String>> {
    let refs = self.inner.references()
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to list references: {e}")))?;
    let mut names = Vec::new();
    for r in refs {
      if let Ok(r) = r {
        if let Ok(name) = r.name() {
          names.push(name.to_string());
        }
      }
    }
    Ok(names)
  }

  #[napi]
  pub fn references_glob(&self, glob: String) -> Result<Vec<String>> {
    let refs = self.inner.references_glob(&glob)
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to list references glob: {e}")))?;
    let mut names = Vec::new();
    for r in refs {
      if let Ok(r) = r {
        if let Ok(name) = r.name() {
          names.push(name.to_string());
        }
      }
    }
    Ok(names)
  }

  #[napi]
  pub fn reference(&self, name: String, target_oid_str: String, force: bool, log_message: String) -> Result<Reference> {
    let oid = git2::Oid::from_str(&target_oid_str)
      .map_err(|e| Error::new(Status::GenericFailure, format!("invalid oid: {e}")))?;
    let r = self.inner.reference(&name, oid, force, &log_message)
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to create reference: {e}")))?;
    unsafe {
      Ok(Reference {
        inner: std::mem::transmute::<git2::Reference<'_>, git2::Reference<'static>>(r),
      })
    }
  }

  #[napi]
  pub fn reference_symbolic(&self, name: String, target: String, force: bool, log_message: String) -> Result<Reference> {
    let r = self.inner.reference_symbolic(&name, &target, force, &log_message)
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to create symbolic reference: {e}")))?;
    unsafe {
      Ok(Reference {
        inner: std::mem::transmute::<git2::Reference<'_>, git2::Reference<'static>>(r),
      })
    }
  }

  #[napi]
  pub fn refname_to_id(&self, name: String) -> Result<String> {
    self.inner.refname_to_id(&name)
      .map(|oid| oid.to_string())
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed refname_to_id: {e}")))
  }

  #[napi]
  pub fn config(&self) -> Result<Config> {
    self.inner
      .config()
      .map(|cfg| Config { inner: cfg })
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to get config: {e}")))
  }

  #[napi]
  pub fn index(&self) -> Result<Index> {
    self.inner
      .index()
      .map(|idx| Index { inner: idx })
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to get index: {e}")))
  }

  #[napi]
  pub fn revwalk(&self) -> Result<Revwalk> {
    let rw = self.inner.revwalk()
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to create revwalk: {e}")))?;
    unsafe {
      Ok(Revwalk {
        inner: std::mem::transmute::<git2::Revwalk<'_>, git2::Revwalk<'static>>(rw),
      })
    }
  }

  #[napi]
  pub fn find_commit(&self, oid_str: String) -> Result<Commit> {
    let oid = git2::Oid::from_str(&oid_str)
      .map_err(|e| Error::new(Status::GenericFailure, format!("invalid oid: {e}")))?;
    let c = self.inner.find_commit(oid)
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to find_commit: {e}")))?;
    unsafe {
      Ok(Commit {
        inner: std::mem::transmute::<git2::Commit<'_>, git2::Commit<'static>>(c),
      })
    }
  }

  #[napi]
  pub fn find_tree(&self, oid_str: String) -> Result<Tree> {
    let oid = git2::Oid::from_str(&oid_str)
      .map_err(|e| Error::new(Status::GenericFailure, format!("invalid oid: {e}")))?;
    let t = self.inner.find_tree(oid)
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to find_tree: {e}")))?;
    unsafe {
      Ok(Tree {
        inner: std::mem::transmute::<git2::Tree<'_>, git2::Tree<'static>>(t),
      })
    }
  }

  #[napi]
  pub fn find_blob(&self, oid_str: String) -> Result<Blob> {
    let oid = git2::Oid::from_str(&oid_str)
      .map_err(|e| Error::new(Status::GenericFailure, format!("invalid oid: {e}")))?;
    let b = self.inner.find_blob(oid)
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to find_blob: {e}")))?;
    unsafe {
      Ok(Blob {
        inner: std::mem::transmute::<git2::Blob<'_>, git2::Blob<'static>>(b),
      })
    }
  }

  #[napi]
  pub fn find_tag(&self, oid_str: String) -> Result<Tag> {
    let oid = git2::Oid::from_str(&oid_str)
      .map_err(|e| Error::new(Status::GenericFailure, format!("invalid oid: {e}")))?;
    let t = self.inner.find_tag(oid)
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to find_tag: {e}")))?;
    unsafe {
      Ok(Tag {
        inner: std::mem::transmute::<git2::Tag<'_>, git2::Tag<'static>>(t),
      })
    }
  }

  #[napi]
  pub fn find_branch(&self, name: String, branch_type: BranchType) -> Result<Branch> {
    let b = self.inner.find_branch(&name, branch_type.into())
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to find_branch: {e}")))?;
    unsafe {
      Ok(Branch {
        inner: std::mem::transmute::<git2::Branch<'_>, git2::Branch<'static>>(b),
      })
    }
  }

  #[napi]
  pub fn find_reference(&self, name: String) -> Result<Reference> {
    let r = self.inner.find_reference(&name)
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to find_reference: {e}")))?;
    unsafe {
      Ok(Reference {
        inner: std::mem::transmute::<git2::Reference<'_>, git2::Reference<'static>>(r),
      })
    }
  }

  #[napi]
  pub fn blob(&self, data: Buffer) -> Result<String> {
    self.inner
      .blob(&data)
      .map(|oid| oid.to_string())
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to create blob: {e}")))
  }

  #[napi]
  pub fn branch(&self, name: String, target_commit_id: String, force: bool) -> Result<Branch> {
    let oid = git2::Oid::from_str(&target_commit_id)
      .map_err(|e| Error::new(Status::GenericFailure, format!("invalid oid: {e}")))?;
    let commit = self.inner.find_commit(oid)
      .map_err(|e| Error::new(Status::GenericFailure, format!("commit not found: {e}")))?;
    let b = self.inner.branch(&name, &commit, force)
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to create branch: {e}")))?;
    unsafe {
      Ok(Branch {
        inner: std::mem::transmute::<git2::Branch<'_>, git2::Branch<'static>>(b),
      })
    }
  }

  #[napi]
  pub fn commit(
    &self,
    update_ref: Option<String>,
    author_name: String,
    author_email: String,
    committer_name: String,
    committer_email: String,
    message: String,
    tree_id: String,
    parent_ids: Vec<String>,
  ) -> Result<String> {
    let author = git2::Signature::now(&author_name, &author_email)
      .map_err(|e| Error::new(Status::GenericFailure, format!("invalid author signature: {e}")))?;
    let committer = git2::Signature::now(&committer_name, &committer_email)
      .map_err(|e| Error::new(Status::GenericFailure, format!("invalid committer signature: {e}")))?;

    let tree_oid = git2::Oid::from_str(&tree_id)
      .map_err(|e| Error::new(Status::GenericFailure, format!("invalid tree oid: {e}")))?;
    let tree = self.inner.find_tree(tree_oid)
      .map_err(|e| Error::new(Status::GenericFailure, format!("tree not found: {e}")))?;

    let mut parents = Vec::new();
    for pid in parent_ids {
      let p_oid = git2::Oid::from_str(&pid)
        .map_err(|e| Error::new(Status::GenericFailure, format!("invalid parent oid: {e}")))?;
      let p_commit = self.inner.find_commit(p_oid)
        .map_err(|e| Error::new(Status::GenericFailure, format!("parent commit not found: {e}")))?;
      parents.push(p_commit);
    }

    let parent_refs: Vec<&git2::Commit> = parents.iter().collect();

    self.inner
      .commit(
        update_ref.as_deref(),
        &author,
        &committer,
        &message,
        &tree,
        &parent_refs,
      )
      .map(|oid| oid.to_string())
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to commit: {e}")))
  }

  #[napi]
  pub fn tag(
    &self,
    name: String,
    target_oid: String,
    tagger_name: String,
    tagger_email: String,
    message: String,
    force: bool,
  ) -> Result<String> {
    let oid = git2::Oid::from_str(&target_oid)
      .map_err(|e| Error::new(Status::GenericFailure, format!("invalid target oid: {e}")))?;
    let target = self.inner.find_object(oid, None)
      .map_err(|e| Error::new(Status::GenericFailure, format!("target object not found: {e}")))?;
    let tagger = git2::Signature::now(&tagger_name, &tagger_email)
      .map_err(|e| Error::new(Status::GenericFailure, format!("invalid tagger signature: {e}")))?;

    self.inner
      .tag(&name, &target, &tagger, &message, force)
      .map(|oid| oid.to_string())
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to create tag: {e}")))
  }

  #[napi]
  pub fn checkout_head(&self) -> Result<()> {
    self.inner
      .checkout_head(None)
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to checkout_head: {e}")))
  }

  #[napi]
  pub fn statuses(&self) -> Result<Vec<StatusEntry>> {
    let list = self.inner
      .statuses(None)
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to get statuses: {e}")))?;

    let mut entries = Vec::new();
    for entry in list.iter() {
      entries.push(StatusEntry {
        path: entry.path().ok().map(|s| s.to_string()),
        status_bits: entry.status().bits(),
      });
    }
    Ok(entries)
  }

  #[napi]
  pub fn status_file(&self, path: String) -> Result<u32> {
    self.inner
      .status_file(std::path::Path::new(&path))
      .map(|s| s.bits())
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to get status_file: {e}")))
  }

  #[napi]
  pub fn diff_tree_to_tree(&self, old_tree_id: Option<String>, new_tree_id: Option<String>) -> Result<Diff> {
    let old_tree = match old_tree_id {
      Some(id) => {
        let oid = git2::Oid::from_str(&id)
          .map_err(|e| Error::new(Status::GenericFailure, format!("invalid oid: {e}")))?;
        Some(self.inner.find_tree(oid).map_err(|e| Error::new(Status::GenericFailure, format!("old_tree not found: {e}")))? )
      }
      None => None,
    };

    let new_tree = match new_tree_id {
      Some(id) => {
        let oid = git2::Oid::from_str(&id)
          .map_err(|e| Error::new(Status::GenericFailure, format!("invalid oid: {e}")))?;
        Some(self.inner.find_tree(oid).map_err(|e| Error::new(Status::GenericFailure, format!("new_tree not found: {e}")))? )
      }
      None => None,
    };

    let diff = self.inner
      .diff_tree_to_tree(old_tree.as_ref(), new_tree.as_ref(), None)
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed diff_tree_to_tree: {e}")))?;

    unsafe {
      Ok(Diff {
        inner: std::mem::transmute::<git2::Diff<'_>, git2::Diff<'static>>(diff),
      })
    }
  }

  #[napi]
  pub fn worktrees(&self) -> Result<Vec<String>> {
    let array = self.inner
      .worktrees()
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to list worktrees: {e}")))?;
    let mut names = Vec::new();
    for i in 0..array.len() {
      if let Ok(Some(s)) = array.get(i) {
        names.push(s.to_string());
      }
    }
    Ok(names)
  }

  #[napi]
  pub fn find_worktree(&self, name: String) -> Result<Worktree> {
    let wt = self.inner
      .find_worktree(&name)
      .map_err(|e| Error::new(Status::GenericFailure, format!("worktree not found: {e}")))?;
    Ok(Worktree { inner: wt })
  }
}
