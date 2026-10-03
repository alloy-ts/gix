use napi::bindgen_prelude::*;
use napi_derive::napi;

pub mod config;
pub mod index;
pub mod object;
pub mod reference;
pub mod repository;
pub mod revwalk;
pub mod worktree;

pub use config::*;
pub use index::*;
pub use object::*;
pub use reference::*;
pub use repository::*;
pub use revwalk::*;
pub use worktree::*;

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

impl From<gix::state::InProgress> for RepositoryState {
  fn from(s: gix::state::InProgress) -> Self {
    match s {
      gix::state::InProgress::Merge => RepositoryState::Merge,
      gix::state::InProgress::Revert => RepositoryState::Revert,
      gix::state::InProgress::RevertSequence => RepositoryState::RevertSequence,
      gix::state::InProgress::CherryPick => RepositoryState::CherryPick,
      gix::state::InProgress::CherryPickSequence => RepositoryState::CherryPickSequence,
      gix::state::InProgress::Bisect => RepositoryState::Bisect,
      gix::state::InProgress::Rebase => RepositoryState::Rebase,
      gix::state::InProgress::RebaseInteractive => RepositoryState::RebaseInteractive,
      gix::state::InProgress::ApplyMailbox => RepositoryState::ApplyMailbox,
      gix::state::InProgress::ApplyMailboxRebase => RepositoryState::ApplyMailboxOrRebase,
    }
  }
}

#[napi]
pub enum BranchType {
  Local,
  Remote,
}

#[napi]
pub enum ObjectType {
  Any,
  Commit,
  Tree,
  Blob,
  Tag,
}

#[napi]
pub enum ResetType {
  Soft,
  Mixed,
  Hard,
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

#[napi]
pub struct Signature {
  name: String,
  email: String,
  time_seconds: i64,
}

impl Signature {
  pub fn new(name: String, email: String, time_seconds: i64) -> Self {
    Signature { name, email, time_seconds }
  }
}

#[napi]
impl Signature {
  #[napi(factory)]
  pub fn now(name: String, email: String) -> Result<Signature> {
    let now = std::time::SystemTime::now()
      .duration_since(std::time::UNIX_EPOCH)
      .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?
      .as_secs() as i64;
    Ok(Signature {
      name,
      email,
      time_seconds: now,
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
  inner: gix::hash::ObjectId,
}

#[napi]
impl Oid {
  #[napi(factory)]
  pub fn from_str(s: String) -> Result<Oid> {
    let id = gix::hash::ObjectId::from_hex(s.as_bytes())
      .map_err(|e| Error::new(Status::GenericFailure, format!("invalid oid: {e}")))?;
    Ok(Oid { inner: id })
  }

  #[napi]
  pub fn to_string(&self) -> String {
    self.inner.to_string()
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
  deltas: Vec<DiffDelta>,
}

impl Diff {
  pub fn new() -> Self {
    Diff { deltas: Vec::new() }
  }
}

#[napi]
impl Diff {
  #[napi]
  pub fn deltas_len(&self) -> u32 {
    self.deltas.len() as u32
  }

  #[napi]
  pub fn get_delta(&self, idx: u32) -> Option<DiffDelta> {
    self.deltas.get(idx as usize).map(|d| DiffDelta {
      status: d.status,
      old_file: DiffFile {
        path: d.old_file.path.clone(),
        id: d.old_file.id.clone(),
        size: d.old_file.size,
      },
      new_file: DiffFile {
        path: d.new_file.path.clone(),
        id: d.new_file.id.clone(),
        size: d.new_file.size,
      },
    })
  }
}
