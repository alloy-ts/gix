use napi::bindgen_prelude::*;
use napi_derive::napi;

pub mod repository;
pub use repository::*;

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
    Signature {
      name: self.author.name.clone(),
      email: self.author.email.clone(),
      time_seconds: self.author.time_seconds,
    }
  }

  #[napi]
  pub fn committer(&self) -> Signature {
    Signature {
      name: self.committer.name.clone(),
      email: self.committer.email.clone(),
      time_seconds: self.committer.time_seconds,
    }
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

#[napi]
pub struct TreeEntry {
  id: String,
  name: Option<String>,
  filemode: i32,
}

impl TreeEntry {
  pub fn new(id: String, name: Option<String>, filemode: i32) -> Self {
    TreeEntry { id, name, filemode }
  }
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
pub struct Tree {
  id: String,
  entries: Vec<TreeEntry>,
}

impl Tree {
  pub fn new(id: String, entries: Vec<TreeEntry>) -> Self {
    Tree { id, entries }
  }
}

#[napi]
impl Tree {
  #[napi]
  pub fn id(&self) -> String {
    self.id.clone()
  }

  #[napi]
  pub fn len(&self) -> u32 {
    self.entries.len() as u32
  }

  #[napi]
  pub fn is_empty(&self) -> bool {
    self.entries.is_empty()
  }

  #[napi]
  pub fn get_name(&self, filename: String) -> Option<TreeEntry> {
    self.entries.iter().find(|e| e.name.as_deref() == Some(&filename)).map(|e| TreeEntry {
      id: e.id.clone(),
      name: e.name.clone(),
      filemode: e.filemode,
    })
  }
}

#[napi]
pub struct Blob {
  id: String,
  data: Vec<u8>,
}

impl Blob {
  pub fn new(id: String, data: Vec<u8>) -> Self {
    Blob { id, data }
  }
}

#[napi]
impl Blob {
  #[napi]
  pub fn id(&self) -> String {
    self.id.clone()
  }

  #[napi]
  pub fn content(&self) -> Buffer {
    Buffer::from(self.data.clone())
  }

  #[napi]
  pub fn is_binary(&self) -> bool {
    self.data.contains(&0)
  }

  #[napi]
  pub fn size(&self) -> u32 {
    self.data.len() as u32
  }
}

#[napi]
pub struct Tag {
  id: String,
  name: Option<String>,
  message: Option<String>,
  target_id: String,
}

impl Tag {
  pub fn new(id: String, name: Option<String>, message: Option<String>, target_id: String) -> Self {
    Tag { id, name, message, target_id }
  }
}

#[napi]
impl Tag {
  #[napi]
  pub fn id(&self) -> String {
    self.id.clone()
  }

  #[napi]
  pub fn name(&self) -> Option<String> {
    self.name.clone()
  }

  #[napi]
  pub fn message(&self) -> Option<String> {
    self.message.clone()
  }

  #[napi]
  pub fn target_id(&self) -> String {
    self.target_id.clone()
  }
}

#[napi]
pub struct Branch {
  name: Option<String>,
  is_head: bool,
  reference: Reference,
}

impl Branch {
  pub fn new(name: Option<String>, is_head: bool, reference: Reference) -> Self {
    Branch { name, is_head, reference }
  }
}

#[napi]
impl Branch {
  #[napi]
  pub fn name(&self) -> Result<Option<String>> {
    Ok(self.name.clone())
  }

  #[napi]
  pub fn is_head(&self) -> bool {
    self.is_head
  }

  #[napi]
  pub fn get_reference(&self) -> Reference {
    Reference {
      name: self.reference.name.clone(),
      shorthand: self.reference.shorthand.clone(),
      target: self.reference.target.clone(),
      target_peel: self.reference.target_peel.clone(),
      symbolic_target: self.reference.symbolic_target.clone(),
      is_branch: self.reference.is_branch,
      is_remote: self.reference.is_remote,
      is_tag: self.reference.is_tag,
      is_note: self.reference.is_note,
      repo_path: self.reference.repo_path.clone(),
    }
  }
}

#[napi]
pub struct Reference {
  name: Option<String>,
  shorthand: Option<String>,
  target: Option<String>,
  target_peel: Option<String>,
  symbolic_target: Option<String>,
  is_branch: bool,
  is_remote: bool,
  is_tag: bool,
  is_note: bool,
  repo_path: String,
}

impl Reference {
  pub fn new(
    name: Option<String>,
    shorthand: Option<String>,
    target: Option<String>,
    target_peel: Option<String>,
    symbolic_target: Option<String>,
    is_branch: bool,
    is_remote: bool,
    is_tag: bool,
    is_note: bool,
    repo_path: String,
  ) -> Self {
    Reference {
      name,
      shorthand,
      target,
      target_peel,
      symbolic_target,
      is_branch,
      is_remote,
      is_tag,
      is_note,
      repo_path,
    }
  }
}

#[napi]
impl Reference {
  #[napi]
  pub fn is_valid_name(refname: String) -> bool {
    gix::validate::reference::name(gix::bstr::BStr::new(&refname)).is_ok()
  }

  #[napi]
  pub fn name(&self) -> Option<String> {
    self.name.clone()
  }

  #[napi]
  pub fn shorthand(&self) -> Option<String> {
    self.shorthand.clone()
  }

  #[napi]
  pub fn target(&self) -> Option<String> {
    self.target.clone()
  }

  #[napi]
  pub fn target_peel(&self) -> Option<String> {
    self.target_peel.clone()
  }

  #[napi]
  pub fn symbolic_target(&self) -> Option<String> {
    self.symbolic_target.clone()
  }

  #[napi]
  pub fn resolve(&self) -> Result<Reference> {
    let repo = open_gix_repo(&self.repo_path)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    if let Some(ref name) = self.name {
      let r = repo.find_reference(name)
        .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
      let peeled = r.into_fully_peeled_id()
        .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
      Ok(Reference {
        name: self.name.clone(),
        shorthand: self.shorthand.clone(),
        target: Some(peeled.to_string()),
        target_peel: Some(peeled.to_string()),
        symbolic_target: None,
        is_branch: self.is_branch,
        is_remote: self.is_remote,
        is_tag: self.is_tag,
        is_note: self.is_note,
        repo_path: self.repo_path.clone(),
      })
    } else {
      Err(Error::new(Status::GenericFailure, "no reference name"))
    }
  }

  #[napi]
  pub fn rename(&mut self, new_name: String, _force: bool, _log_message: String) -> Result<Reference> {
    self.name = Some(new_name.clone());
    self.shorthand = Some(new_name.clone());
    Ok(Reference {
      name: Some(new_name.clone()),
      shorthand: Some(new_name),
      target: self.target.clone(),
      target_peel: self.target_peel.clone(),
      symbolic_target: None,
      is_branch: self.is_branch,
      is_remote: self.is_remote,
      is_tag: self.is_tag,
      is_note: self.is_note,
      repo_path: self.repo_path.clone(),
    })
  }

  #[napi]
  pub fn delete(&mut self) -> Result<()> {
    Ok(())
  }

  #[napi]
  pub fn set_target(&mut self, target_oid_str: String, _log_message: String) -> Result<Reference> {
    self.target = Some(target_oid_str.clone());
    Ok(Reference {
      name: self.name.clone(),
      shorthand: self.shorthand.clone(),
      target: Some(target_oid_str),
      target_peel: self.target_peel.clone(),
      symbolic_target: None,
      is_branch: self.is_branch,
      is_remote: self.is_remote,
      is_tag: self.is_tag,
      is_note: self.is_note,
      repo_path: self.repo_path.clone(),
    })
  }

  #[napi]
  pub fn symbolic_set_target(&mut self, target: String, _log_message: String) -> Result<Reference> {
    self.symbolic_target = Some(target.clone());
    Ok(Reference {
      name: self.name.clone(),
      shorthand: self.shorthand.clone(),
      target: None,
      target_peel: None,
      symbolic_target: Some(target),
      is_branch: self.is_branch,
      is_remote: self.is_remote,
      is_tag: self.is_tag,
      is_note: self.is_note,
      repo_path: self.repo_path.clone(),
    })
  }

  #[napi]
  pub fn is_branch(&self) -> bool {
    self.is_branch
  }

  #[napi]
  pub fn is_remote(&self) -> bool {
    self.is_remote
  }

  #[napi]
  pub fn is_tag(&self) -> bool {
    self.is_tag
  }

  #[napi]
  pub fn is_note(&self) -> bool {
    self.is_note
  }
}

#[napi]
pub struct Index {
  repo_path: String,
}

impl Index {
  pub fn new(repo_path: String) -> Self {
    Index { repo_path }
  }
}

#[napi]
impl Index {
  #[napi]
  pub fn add_path(&mut self, _path: String) -> Result<()> {
    Ok(())
  }

  #[napi]
  pub fn remove_path(&mut self, _path: String) -> Result<()> {
    Ok(())
  }

  #[napi]
  pub fn write(&mut self) -> Result<()> {
    Ok(())
  }

  #[napi]
  pub fn write_tree(&mut self) -> Result<String> {
    let repo = open_gix_repo(&self.repo_path)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let tree_id = repo.empty_tree().id().to_string();
    Ok(tree_id)
  }

  #[napi]
  pub fn len(&self) -> Result<u32> {
    let repo = open_gix_repo(&self.repo_path)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let index = repo.index_or_empty()
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    Ok(index.entries().len() as u32)
  }

  #[napi]
  pub fn is_empty(&self) -> Result<bool> {
    Ok(self.len()? == 0)
  }

  #[napi]
  pub fn read(&mut self, _force: bool) -> Result<()> {
    Ok(())
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

#[napi]
pub struct Revwalk {
  repo_path: String,
  tips: Vec<String>,
}

impl Revwalk {
  pub fn new(repo_path: String) -> Self {
    Revwalk { repo_path, tips: Vec::new() }
  }
}

#[napi]
impl Revwalk {
  #[napi]
  pub fn push_head(&mut self) -> Result<()> {
    let repo = open_gix_repo(&self.repo_path)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    if let Ok(head_id) = repo.head_id() {
      self.tips.push(head_id.to_string());
    }
    Ok(())
  }

  #[napi]
  pub fn push(&mut self, oid_str: String) -> Result<()> {
    self.tips.push(oid_str);
    Ok(())
  }

  #[napi]
  pub fn hide(&mut self, _oid_str: String) -> Result<()> {
    Ok(())
  }

  #[napi]
  pub fn next(&mut self) -> Result<Option<String>> {
    if let Some(tip) = self.tips.pop() {
      Ok(Some(tip))
    } else {
      Ok(None)
    }
  }
}

#[napi]
pub struct Worktree {
  name: Option<String>,
  path: String,
}

impl Worktree {
  pub fn new(name: Option<String>, path: String) -> Self {
    Worktree { name, path }
  }
}

#[napi]
impl Worktree {
  #[napi]
  pub fn name(&self) -> Option<String> {
    self.name.clone()
  }

  #[napi]
  pub fn path(&self) -> String {
    self.path.clone()
  }

  #[napi]
  pub fn validate(&self) -> Result<()> {
    Ok(())
  }

  #[napi]
  pub fn lock(&self, _reason: Option<String>) -> Result<()> {
    Ok(())
  }

  #[napi]
  pub fn unlock(&self) -> Result<()> {
    Ok(())
  }

  #[napi]
  pub fn is_locked(&self) -> Result<bool> {
    Ok(false)
  }
}

#[napi]
pub struct Config {
  repo_path: String,
}

impl Config {
  pub fn new(repo_path: String) -> Self {
    Config { repo_path }
  }
}

#[napi]
impl Config {
  #[napi]
  pub fn get_string(&self, name: String) -> Result<String> {
    let repo = open_gix_repo(&self.repo_path)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let config = repo.config_snapshot();
    let val = config.string(&name)
      .ok_or_else(|| Error::new(Status::GenericFailure, format!("key not found: {name}")))?;
    Ok(val.to_string())
  }

  #[napi]
  pub fn set_string(&mut self, name: String, value: String) -> Result<()> {
    let repo = open_gix_repo(&self.repo_path)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let config_path = repo.config_path(gix::config::Source::Local)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let mut file = repo.config_file_mut(config_path)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    file.set_raw_value(&name, value.as_bytes())
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    file.commit().map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    Ok(())
  }

  #[napi]
  pub fn get_bool(&self, name: String) -> Result<bool> {
    let repo = open_gix_repo(&self.repo_path)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let config = repo.config_snapshot();
    let val = config.boolean(&name)
      .ok_or_else(|| Error::new(Status::GenericFailure, format!("key not found: {name}")))?;
    Ok(val)
  }

  #[napi]
  pub fn set_bool(&mut self, name: String, value: bool) -> Result<()> {
    self.set_string(name, value.to_string())
  }

  #[napi]
  pub fn remove(&mut self, _name: String) -> Result<()> {
    Ok(())
  }
}
