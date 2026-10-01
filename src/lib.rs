use napi::bindgen_prelude::*;
use napi::Error;
use napi_derive::napi;
use std::path::Path;

#[napi]
pub struct Signature {
  name: String,
  email: String,
  time_seconds: i64,
}

#[napi]
impl Signature {
  #[napi(factory)]
  pub fn now(name: String, email: String) -> napi::Result<Self> {
    let sig = git2::Signature::now(&name, &email).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Self {
      name: sig.name().unwrap_or("").to_string(),
      email: sig.email().unwrap_or("").to_string(),
      time_seconds: sig.when().seconds(),
    })
  }

  #[napi]
  pub fn name(&self) -> String {
    self.name.clone()
  }

  #[napi]
  pub fn email(&self) -> String {
    self.email.clone()
  }

  #[napi]
  pub fn time_seconds(&self) -> i64 {
    self.time_seconds
  }
}

impl Signature {
  fn to_git2(&self) -> napi::Result<git2::Signature<'static>> {
    git2::Signature::new(&self.name, &self.email, &git2::Time::new(self.time_seconds, 0))
      .map_err(|e| Error::from_reason(e.to_string()))
  }
}

#[napi]
pub struct Commit {
  id: String,
  message: Option<String>,
  summary: Option<String>,
  author_name: String,
  author_email: String,
  author_time: i64,
  committer_name: String,
  committer_email: String,
  committer_time: i64,
  parent_ids: Vec<String>,
  tree_id: String,
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
  pub fn author(&self) -> Signature {
    Signature {
      name: self.author_name.clone(),
      email: self.author_email.clone(),
      time_seconds: self.author_time,
    }
  }

  #[napi]
  pub fn committer(&self) -> Signature {
    Signature {
      name: self.committer_name.clone(),
      email: self.committer_email.clone(),
      time_seconds: self.committer_time,
    }
  }

  #[napi]
  pub fn parent_count(&self) -> u32 {
    self.parent_ids.len() as u32
  }

  #[napi]
  pub fn parent_ids(&self) -> Vec<String> {
    self.parent_ids.clone()
  }

  #[napi]
  pub fn tree_id(&self) -> String {
    self.tree_id.clone()
  }
}

impl Commit {
  fn from_git2(commit: &git2::Commit) -> Self {
    let author = commit.author();
    let committer = commit.committer();
    let parent_ids = commit.parent_ids().map(|oid| oid.to_string()).collect();

    let message = commit.message().ok().map(|s| s.to_string());
    let summary = commit.summary().ok().flatten().map(|s| s.to_string());

    Self {
      id: commit.id().to_string(),
      message,
      summary,
      author_name: author.name().unwrap_or("").to_string(),
      author_email: author.email().unwrap_or("").to_string(),
      author_time: author.when().seconds(),
      committer_name: committer.name().unwrap_or("").to_string(),
      committer_email: committer.email().unwrap_or("").to_string(),
      committer_time: committer.when().seconds(),
      parent_ids,
      tree_id: commit.tree_id().to_string(),
    }
  }
}

#[napi]
pub struct Tag {
  id: String,
  name: Option<String>,
  target_id: String,
  message: Option<String>,
  tagger: Option<Signature>,
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
  pub fn target_id(&self) -> String {
    self.target_id.clone()
  }

  #[napi]
  pub fn message(&self) -> Option<String> {
    self.message.clone()
  }

  #[napi]
  pub fn tagger(&self) -> Option<Signature> {
    self.tagger.as_ref().map(|t| Signature {
      name: t.name.clone(),
      email: t.email.clone(),
      time_seconds: t.time_seconds,
    })
  }
}

impl Tag {
  fn from_git2(tag: &git2::Tag) -> Self {
    let tagger = tag.tagger().map(|t| Signature {
      name: t.name().unwrap_or("").to_string(),
      email: t.email().unwrap_or("").to_string(),
      time_seconds: t.when().seconds(),
    });

    Self {
      id: tag.id().to_string(),
      name: tag.name().ok().map(|s| s.to_string()),
      target_id: tag.target_id().to_string(),
      message: tag.message().ok().flatten().map(|s| s.to_string()),
      tagger,
    }
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
  #[napi]
  pub fn id(&self) -> String {
    self.id.clone()
  }

  #[napi]
  pub fn name(&self) -> Option<String> {
    self.name.clone()
  }

  #[napi]
  pub fn filemode(&self) -> i32 {
    self.filemode
  }
}

#[napi]
pub struct Tree {
  id: String,
  entries: Vec<TreeEntry>,
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
  pub fn entry_by_index(&self, index: u32) -> Option<TreeEntry> {
    self.entries.get(index as usize).map(|e| TreeEntry {
      id: e.id.clone(),
      name: e.name.clone(),
      filemode: e.filemode,
    })
  }

  #[napi]
  pub fn entry_by_name(&self, name: String) -> Option<TreeEntry> {
    self.entries
      .iter()
      .find(|e| e.name.as_deref() == Some(&name))
      .map(|e| TreeEntry {
        id: e.id.clone(),
        name: e.name.clone(),
        filemode: e.filemode,
      })
  }
}

impl Tree {
  fn from_git2(tree: &git2::Tree) -> Self {
    let mut entries = Vec::new();
    for entry in tree.iter() {
      entries.push(TreeEntry {
        id: entry.id().to_string(),
        name: entry.name().ok().map(|s| s.to_string()),
        filemode: entry.filemode(),
      });
    }
    Self {
      id: tree.id().to_string(),
      entries,
    }
  }
}

#[napi]
pub struct Blob {
  id: String,
  size: u32,
  content: Vec<u8>,
  is_binary: bool,
}

#[napi]
impl Blob {
  #[napi]
  pub fn id(&self) -> String {
    self.id.clone()
  }

  #[napi]
  pub fn size(&self) -> u32 {
    self.size
  }

  #[napi]
  pub fn content(&self) -> Buffer {
    Buffer::from(self.content.clone())
  }

  #[napi]
  pub fn is_binary(&self) -> bool {
    self.is_binary
  }
}

impl Blob {
  fn from_git2(blob: &git2::Blob) -> Self {
    Self {
      id: blob.id().to_string(),
      size: blob.size() as u32,
      content: blob.content().to_vec(),
      is_binary: blob.is_binary(),
    }
  }
}

#[napi]
pub struct Reference {
  name: Option<String>,
  target: Option<String>,
  symbolic_target: Option<String>,
  is_branch: bool,
  is_remote: bool,
  is_tag: bool,
  is_note: bool,
  shorthand: Option<String>,
}

#[napi]
impl Reference {
  #[napi]
  pub fn name(&self) -> Option<String> {
    self.name.clone()
  }

  #[napi]
  pub fn target(&self) -> Option<String> {
    self.target.clone()
  }

  #[napi]
  pub fn symbolic_target(&self) -> Option<String> {
    self.symbolic_target.clone()
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

  #[napi]
  pub fn shorthand(&self) -> Option<String> {
    self.shorthand.clone()
  }
}

impl Reference {
  fn from_git2(reference: &git2::Reference) -> Self {
    Self {
      name: reference.name().ok().map(|s| s.to_string()),
      target: reference.target().map(|oid| oid.to_string()),
      symbolic_target: reference.symbolic_target().ok().flatten().map(|s| s.to_string()),
      is_branch: reference.is_branch(),
      is_remote: reference.is_remote(),
      is_tag: reference.is_tag(),
      is_note: reference.is_note(),
      shorthand: reference.shorthand().ok().map(|s| s.to_string()),
    }
  }
}

#[napi]
pub struct Branch {
  name: Option<String>,
  is_head: bool,
  reference: Reference,
}

#[napi]
impl Branch {
  #[napi]
  pub fn name(&self) -> Option<String> {
    self.name.clone()
  }

  #[napi]
  pub fn is_head(&self) -> bool {
    self.is_head
  }

  #[napi]
  pub fn get_reference(&self) -> Reference {
    Reference {
      name: self.reference.name.clone(),
      target: self.reference.target.clone(),
      symbolic_target: self.reference.symbolic_target.clone(),
      is_branch: self.reference.is_branch,
      is_remote: self.reference.is_remote,
      is_tag: self.reference.is_tag,
      is_note: self.reference.is_note,
      shorthand: self.reference.shorthand.clone(),
    }
  }
}

impl Branch {
  fn from_git2(branch: &git2::Branch) -> Self {
    let name = branch.name().ok().flatten().map(|s| s.to_string());
    let is_head = branch.is_head();
    let reference = Reference::from_git2(branch.get());
    Self {
      name,
      is_head,
      reference,
    }
  }
}

#[napi]
pub struct Refdb {}

#[napi]
impl Refdb {
  #[napi]
  pub fn compress(&self) -> napi::Result<()> {
    Ok(())
  }
}

#[napi]
pub struct ReflogEntry {
  id_old: String,
  id_new: String,
  committer_name: String,
  committer_email: String,
  committer_time: i64,
  message: Option<String>,
}

#[napi]
impl ReflogEntry {
  #[napi]
  pub fn id_old(&self) -> String {
    self.id_old.clone()
  }

  #[napi]
  pub fn id_new(&self) -> String {
    self.id_new.clone()
  }

  #[napi]
  pub fn committer(&self) -> Signature {
    Signature {
      name: self.committer_name.clone(),
      email: self.committer_email.clone(),
      time_seconds: self.committer_time,
    }
  }

  #[napi]
  pub fn message(&self) -> Option<String> {
    self.message.clone()
  }
}

#[napi]
pub struct Reflog {
  entries: Vec<ReflogEntry>,
}

#[napi]
impl Reflog {
  #[napi]
  pub fn len(&self) -> u32 {
    self.entries.len() as u32
  }

  #[napi]
  pub fn get(&self, index: u32) -> Option<ReflogEntry> {
    self.entries.get(index as usize).map(|e| ReflogEntry {
      id_old: e.id_old.clone(),
      id_new: e.id_new.clone(),
      committer_name: e.committer_name.clone(),
      committer_email: e.committer_email.clone(),
      committer_time: e.committer_time,
      message: e.message.clone(),
    })
  }
}

impl Reflog {
  fn from_git2(reflog: &git2::Reflog) -> Self {
    let mut entries = Vec::new();
    for entry in reflog.iter() {
      let committer = entry.committer();
      entries.push(ReflogEntry {
        id_old: entry.id_old().to_string(),
        id_new: entry.id_new().to_string(),
        committer_name: committer.name().unwrap_or("").to_string(),
        committer_email: committer.email().unwrap_or("").to_string(),
        committer_time: committer.when().seconds(),
        message: entry.message().ok().flatten().map(|s| s.to_string()),
      });
    }
    Self { entries }
  }
}

#[napi]
pub struct Config {
  path: Option<String>,
}

#[napi]
impl Config {
  #[napi]
  pub fn get_string(&self, name: String) -> napi::Result<String> {
    let cfg = self.open_git2()?;
    cfg.get_string(&name).map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn set_string(&self, name: String, value: String) -> napi::Result<()> {
    let mut cfg = self.open_git2()?;
    cfg.set_str(&name, &value).map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn get_bool(&self, name: String) -> napi::Result<bool> {
    let cfg = self.open_git2()?;
    cfg.get_bool(&name).map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn set_bool(&self, name: String, value: bool) -> napi::Result<()> {
    let mut cfg = self.open_git2()?;
    cfg.set_bool(&name, value).map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn get_i32(&self, name: String) -> napi::Result<i32> {
    let cfg = self.open_git2()?;
    cfg.get_i32(&name).map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn set_i32(&self, name: String, value: i32) -> napi::Result<()> {
    let mut cfg = self.open_git2()?;
    cfg.set_i32(&name, value).map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn delete(&self, name: String) -> napi::Result<()> {
    let mut cfg = self.open_git2()?;
    cfg.remove(&name).map_err(|e| Error::from_reason(e.to_string()))
  }
}

impl Config {
  fn open_git2(&self) -> napi::Result<git2::Config> {
    if let Some(ref p) = self.path {
      git2::Config::open(Path::new(p)).map_err(|e| Error::from_reason(e.to_string()))
    } else {
      git2::Config::open_default().map_err(|e| Error::from_reason(e.to_string()))
    }
  }
}

#[napi]
pub struct StatusEntry {
  path: Option<String>,
  status: Vec<String>,
}

#[napi]
impl StatusEntry {
  #[napi]
  pub fn path(&self) -> Option<String> {
    self.path.clone()
  }

  #[napi]
  pub fn status(&self) -> Vec<String> {
    self.status.clone()
  }
}

#[napi]
pub struct Statuses {
  entries: Vec<StatusEntry>,
}

#[napi]
impl Statuses {
  #[napi]
  pub fn len(&self) -> u32 {
    self.entries.len() as u32
  }

  #[napi]
  pub fn is_empty(&self) -> bool {
    self.entries.is_empty()
  }

  #[napi]
  pub fn get(&self, index: u32) -> Option<StatusEntry> {
    self.entries.get(index as usize).map(|e| StatusEntry {
      path: e.path.clone(),
      status: e.status.clone(),
    })
  }
}

impl Statuses {
  fn from_git2(statuses: &git2::Statuses) -> Self {
    let mut entries = Vec::new();
    for entry in statuses.iter() {
      let mut status_flags = Vec::new();
      let st = entry.status();
      if st.contains(git2::Status::INDEX_NEW) {
        status_flags.push("INDEX_NEW".to_string());
      }
      if st.contains(git2::Status::INDEX_MODIFIED) {
        status_flags.push("INDEX_MODIFIED".to_string());
      }
      if st.contains(git2::Status::INDEX_DELETED) {
        status_flags.push("INDEX_DELETED".to_string());
      }
      if st.contains(git2::Status::WT_NEW) {
        status_flags.push("WT_NEW".to_string());
      }
      if st.contains(git2::Status::WT_MODIFIED) {
        status_flags.push("WT_MODIFIED".to_string());
      }
      if st.contains(git2::Status::WT_DELETED) {
        status_flags.push("WT_DELETED".to_string());
      }

      entries.push(StatusEntry {
        path: entry.path().ok().map(|s| s.to_string()),
        status: status_flags,
      });
    }
    Self { entries }
  }
}

#[napi]
pub struct Index {
  repo_path: String,
}

#[napi]
impl Index {
  #[napi]
  pub fn read(&self, force: bool) -> napi::Result<()> {
    let repo = git2::Repository::open(&self.repo_path).map_err(|e| Error::from_reason(e.to_string()))?;
    let mut idx = repo.index().map_err(|e| Error::from_reason(e.to_string()))?;
    idx.read(force).map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn write(&self) -> napi::Result<()> {
    let repo = git2::Repository::open(&self.repo_path).map_err(|e| Error::from_reason(e.to_string()))?;
    let mut idx = repo.index().map_err(|e| Error::from_reason(e.to_string()))?;
    idx.write().map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn add_path(&self, path: String) -> napi::Result<()> {
    let repo = git2::Repository::open(&self.repo_path).map_err(|e| Error::from_reason(e.to_string()))?;
    let mut idx = repo.index().map_err(|e| Error::from_reason(e.to_string()))?;
    idx.add_path(Path::new(&path)).map_err(|e| Error::from_reason(e.to_string()))?;
    idx.write().map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn remove_path(&self, path: String) -> napi::Result<()> {
    let repo = git2::Repository::open(&self.repo_path).map_err(|e| Error::from_reason(e.to_string()))?;
    let mut idx = repo.index().map_err(|e| Error::from_reason(e.to_string()))?;
    idx.remove_path(Path::new(&path)).map_err(|e| Error::from_reason(e.to_string()))?;
    idx.write().map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn len(&self) -> napi::Result<u32> {
    let repo = git2::Repository::open(&self.repo_path).map_err(|e| Error::from_reason(e.to_string()))?;
    let idx = repo.index().map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(idx.len() as u32)
  }

  #[napi]
  pub fn clear(&self) -> napi::Result<()> {
    let repo = git2::Repository::open(&self.repo_path).map_err(|e| Error::from_reason(e.to_string()))?;
    let mut idx = repo.index().map_err(|e| Error::from_reason(e.to_string()))?;
    idx.clear().map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn write_tree(&self) -> napi::Result<String> {
    let repo = git2::Repository::open(&self.repo_path).map_err(|e| Error::from_reason(e.to_string()))?;
    let mut idx = repo.index().map_err(|e| Error::from_reason(e.to_string()))?;
    let oid = idx.write_tree().map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(oid.to_string())
  }
}

#[napi]
pub struct Submodule {
  name: Option<String>,
  path: String,
  url: Option<String>,
  branch: Option<String>,
}

#[napi]
impl Submodule {
  #[napi]
  pub fn name(&self) -> Option<String> {
    self.name.clone()
  }

  #[napi]
  pub fn path(&self) -> String {
    self.path.clone()
  }

  #[napi]
  pub fn url(&self) -> Option<String> {
    self.url.clone()
  }

  #[napi]
  pub fn branch(&self) -> Option<String> {
    self.branch.clone()
  }
}

impl Submodule {
  fn from_git2(sub: &git2::Submodule) -> Self {
    Self {
      name: sub.name().ok().map(|s| s.to_string()),
      path: sub.path().to_string_lossy().into_owned(),
      url: sub.url().ok().flatten().map(|s| s.to_string()),
      branch: sub.branch().ok().flatten().map(|s| s.to_string()),
    }
  }
}

#[napi]
pub struct Worktree {
  name: Option<String>,
  path: String,
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
}

impl Worktree {
  fn from_git2(wt: &git2::Worktree) -> Self {
    Self {
      name: wt.name().ok().flatten().map(|s| s.to_string()),
      path: wt.path().to_string_lossy().into_owned(),
    }
  }
}

#[napi]
pub struct BlameHunk {
  final_commit_id: String,
  final_signature: Signature,
  lines_in_hunk: u32,
  final_start_line: u32,
}

#[napi]
impl BlameHunk {
  #[napi]
  pub fn final_commit_id(&self) -> String {
    self.final_commit_id.clone()
  }

  #[napi]
  pub fn final_signature(&self) -> Signature {
    Signature {
      name: self.final_signature.name.clone(),
      email: self.final_signature.email.clone(),
      time_seconds: self.final_signature.time_seconds,
    }
  }

  #[napi]
  pub fn lines_in_hunk(&self) -> u32 {
    self.lines_in_hunk
  }

  #[napi]
  pub fn final_start_line(&self) -> u32 {
    self.final_start_line
  }
}

#[napi]
pub struct Blame {
  hunks: Vec<BlameHunk>,
}

#[napi]
impl Blame {
  #[napi]
  pub fn len(&self) -> u32 {
    self.hunks.len() as u32
  }

  #[napi]
  pub fn get_index(&self, index: u32) -> Option<BlameHunk> {
    self.hunks.get(index as usize).map(|h| BlameHunk {
      final_commit_id: h.final_commit_id.clone(),
      final_signature: Signature {
        name: h.final_signature.name.clone(),
        email: h.final_signature.email.clone(),
        time_seconds: h.final_signature.time_seconds,
      },
      lines_in_hunk: h.lines_in_hunk,
      final_start_line: h.final_start_line,
    })
  }
}

impl Blame {
  fn from_git2(blame: &git2::Blame) -> Self {
    let mut hunks = Vec::new();
    for hunk in blame.iter() {
      let (sig_name, sig_email, sig_time) = if let Some(sig) = hunk.final_signature() {
        (
          sig.name().unwrap_or("").to_string(),
          sig.email().unwrap_or("").to_string(),
          sig.when().seconds(),
        )
      } else {
        ("".to_string(), "".to_string(), 0)
      };

      hunks.push(BlameHunk {
        final_commit_id: hunk.final_commit_id().to_string(),
        final_signature: Signature {
          name: sig_name,
          email: sig_email,
          time_seconds: sig_time,
        },
        lines_in_hunk: hunk.lines_in_hunk() as u32,
        final_start_line: hunk.final_start_line() as u32,
      });
    }
    Self { hunks }
  }
}

#[napi]
pub struct Diff {
  deltas_len: u32,
  files_changed: u32,
  insertions: u32,
  deletions: u32,
}

#[napi]
impl Diff {
  #[napi]
  pub fn deltas_len(&self) -> u32 {
    self.deltas_len
  }

  #[napi]
  pub fn stats_files_changed(&self) -> u32 {
    self.files_changed
  }

  #[napi]
  pub fn stats_insertions(&self) -> u32 {
    self.insertions
  }

  #[napi]
  pub fn stats_deletions(&self) -> u32 {
    self.deletions
  }
}

impl Diff {
  fn from_git2(diff: &git2::Diff) -> Self {
    let deltas_len = diff.deltas().len() as u32;
    let (files_changed, insertions, deletions) = if let Ok(stats) = diff.stats() {
      (stats.files_changed() as u32, stats.insertions() as u32, stats.deletions() as u32)
    } else {
      (0, 0, 0)
    };
    Self {
      deltas_len,
      files_changed,
      insertions,
      deletions,
    }
  }
}

#[napi]
pub struct Repository {
  inner: git2::Repository,
}

#[napi]
impl Repository {
  #[napi]
  pub fn init(path: String) -> napi::Result<Self> {
    let repo = git2::Repository::init(path).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Self { inner: repo })
  }

  #[napi]
  pub fn open(path: String) -> napi::Result<Self> {
    let repo = git2::Repository::open(path).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Self { inner: repo })
  }

  #[napi]
  pub fn clone(url: String, path: String) -> napi::Result<Self> {
    let repo = git2::Repository::clone(&url, path).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Self { inner: repo })
  }

  #[napi]
  pub fn open_bare(path: String) -> napi::Result<Self> {
    let repo = git2::Repository::open_bare(path).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Self { inner: repo })
  }

  #[napi]
  pub fn discover(path: String) -> napi::Result<Self> {
    let repo = git2::Repository::discover(path).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Self { inner: repo })
  }

  #[napi]
  pub fn is_bare(&self) -> bool {
    self.inner.is_bare()
  }

  #[napi]
  pub fn is_empty(&self) -> napi::Result<bool> {
    self.inner.is_empty().map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn is_shallow(&self) -> bool {
    self.inner.is_shallow()
  }

  #[napi]
  pub fn is_worktree(&self) -> bool {
    self.inner.is_worktree()
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
  pub fn commondir(&self) -> String {
    self.inner.commondir().to_string_lossy().into_owned()
  }

  #[napi]
  pub fn head_detached(&self) -> napi::Result<bool> {
    self.inner.head_detached().map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn set_head(&self, refname: String) -> napi::Result<()> {
    self.inner.set_head(&refname).map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn set_head_detached(&self, commit_hex: String) -> napi::Result<()> {
    let oid = git2::Oid::from_str(&commit_hex).map_err(|e| Error::from_reason(e.to_string()))?;
    self.inner.set_head_detached(oid).map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn head(&self) -> napi::Result<Reference> {
    let head_ref = self.inner.head().map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Reference::from_git2(&head_ref))
  }

  #[napi]
  pub fn index(&self) -> napi::Result<Index> {
    let path = self.inner.path().to_string_lossy().into_owned();
    Ok(Index { repo_path: path })
  }

  #[napi]
  pub fn refdb(&self) -> napi::Result<Refdb> {
    Ok(Refdb {})
  }

  #[napi]
  pub fn config(&self) -> napi::Result<Config> {
    let config_path = self.inner.path().join("config").to_string_lossy().into_owned();
    Ok(Config {
      path: Some(config_path),
    })
  }

  #[napi]
  pub fn statuses(&self) -> napi::Result<Statuses> {
    let st = self.inner.statuses(None).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Statuses::from_git2(&st))
  }

  #[napi]
  pub fn signature(&self) -> napi::Result<Signature> {
    let sig = self.inner.signature().map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Signature {
      name: sig.name().unwrap_or("").to_string(),
      email: sig.email().unwrap_or("").to_string(),
      time_seconds: sig.when().seconds(),
    })
  }

  #[napi]
  pub fn find_commit(&self, oid_hex: String) -> napi::Result<Commit> {
    let oid = git2::Oid::from_str(&oid_hex).map_err(|e| Error::from_reason(e.to_string()))?;
    let commit = self.inner.find_commit(oid).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Commit::from_git2(&commit))
  }

  #[napi]
  pub fn find_tree(&self, oid_hex: String) -> napi::Result<Tree> {
    let oid = git2::Oid::from_str(&oid_hex).map_err(|e| Error::from_reason(e.to_string()))?;
    let tree = self.inner.find_tree(oid).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Tree::from_git2(&tree))
  }

  #[napi]
  pub fn find_blob(&self, oid_hex: String) -> napi::Result<Blob> {
    let oid = git2::Oid::from_str(&oid_hex).map_err(|e| Error::from_reason(e.to_string()))?;
    let blob = self.inner.find_blob(oid).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Blob::from_git2(&blob))
  }

  #[napi]
  pub fn find_tag(&self, oid_hex: String) -> napi::Result<Tag> {
    let oid = git2::Oid::from_str(&oid_hex).map_err(|e| Error::from_reason(e.to_string()))?;
    let tag = self.inner.find_tag(oid).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Tag::from_git2(&tag))
  }

  #[napi]
  pub fn find_reference(&self, name: String) -> napi::Result<Reference> {
    let rf = self.inner.find_reference(&name).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Reference::from_git2(&rf))
  }

  #[napi]
  pub fn find_branch(&self, name: String, is_remote: bool) -> napi::Result<Branch> {
    let b_type = if is_remote {
      git2::BranchType::Remote
    } else {
      git2::BranchType::Local
    };
    let branch = self
      .inner
      .find_branch(&name, b_type)
      .map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Branch::from_git2(&branch))
  }

  #[napi]
  pub fn find_submodule(&self, name: String) -> napi::Result<Submodule> {
    let sub = self.inner.find_submodule(&name).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Submodule::from_git2(&sub))
  }

  #[napi]
  pub fn find_worktree(&self, name: String) -> napi::Result<Worktree> {
    let wt = self.inner.find_worktree(&name).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Worktree::from_git2(&wt))
  }

  #[napi]
  pub fn commit(
    &self,
    update_ref: Option<String>,
    author: &Signature,
    committer: &Signature,
    message: String,
    tree_oid_hex: String,
    parent_oid_hexes: Vec<String>,
  ) -> napi::Result<String> {
    let author_sig = author.to_git2()?;
    let committer_sig = committer.to_git2()?;
    let tree_oid = git2::Oid::from_str(&tree_oid_hex).map_err(|e| Error::from_reason(e.to_string()))?;
    let tree = self.inner.find_tree(tree_oid).map_err(|e| Error::from_reason(e.to_string()))?;

    let mut parent_commits = Vec::new();
    for p_hex in &parent_oid_hexes {
      let p_oid = git2::Oid::from_str(p_hex).map_err(|e| Error::from_reason(e.to_string()))?;
      let parent_commit = self
        .inner
        .find_commit(p_oid)
        .map_err(|e| Error::from_reason(e.to_string()))?;
      parent_commits.push(parent_commit);
    }

    let parents_refs: Vec<&git2::Commit> = parent_commits.iter().collect();

    let oid = self
      .inner
      .commit(
        update_ref.as_deref(),
        &author_sig,
        &committer_sig,
        &message,
        &tree,
        &parents_refs,
      )
      .map_err(|e| Error::from_reason(e.to_string()))?;

    Ok(oid.to_string())
  }

  #[napi]
  pub fn tag(
    &self,
    name: String,
    target_oid_hex: String,
    tagger: &Signature,
    message: String,
    force: bool,
  ) -> napi::Result<String> {
    let target_oid = git2::Oid::from_str(&target_oid_hex).map_err(|e| Error::from_reason(e.to_string()))?;
    let target_obj = self
      .inner
      .find_object(target_oid, None)
      .map_err(|e| Error::from_reason(e.to_string()))?;
    let tagger_sig = tagger.to_git2()?;

    let oid = self
      .inner
      .tag(&name, &target_obj, &tagger_sig, &message, force)
      .map_err(|e| Error::from_reason(e.to_string()))?;

    Ok(oid.to_string())
  }

  #[napi]
  pub fn tag_lightweight(
    &self,
    name: String,
    target_oid_hex: String,
    force: bool,
  ) -> napi::Result<String> {
    let target_oid = git2::Oid::from_str(&target_oid_hex).map_err(|e| Error::from_reason(e.to_string()))?;
    let target_obj = self
      .inner
      .find_object(target_oid, None)
      .map_err(|e| Error::from_reason(e.to_string()))?;

    let oid = self
      .inner
      .tag_lightweight(&name, &target_obj, force)
      .map_err(|e| Error::from_reason(e.to_string()))?;

    Ok(oid.to_string())
  }

  #[napi]
  pub fn tag_delete(&self, name: String) -> napi::Result<()> {
    self.inner.tag_delete(&name).map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn tag_names(&self, pattern: Option<String>) -> napi::Result<Vec<String>> {
    let arr = self
      .inner
      .tag_names(pattern.as_deref())
      .map_err(|e| Error::from_reason(e.to_string()))?;
    let mut names = Vec::new();
    for i in 0..arr.len() {
      if let Ok(Some(s)) = arr.get(i) {
        names.push(s.to_string());
      }
    }
    Ok(names)
  }

  #[napi]
  pub fn branch(
    &self,
    branch_name: String,
    target_commit_hex: String,
    force: bool,
  ) -> napi::Result<Branch> {
    let oid = git2::Oid::from_str(&target_commit_hex).map_err(|e| Error::from_reason(e.to_string()))?;
    let commit = self.inner.find_commit(oid).map_err(|e| Error::from_reason(e.to_string()))?;
    let branch = self
      .inner
      .branch(&branch_name, &commit, force)
      .map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Branch::from_git2(&branch))
  }

  #[napi]
  pub fn blob(&self, data: Buffer) -> napi::Result<String> {
    let oid = self.inner.blob(&data).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(oid.to_string())
  }

  #[napi]
  pub fn blob_path(&self, path: String) -> napi::Result<String> {
    let oid = self
      .inner
      .blob_path(Path::new(&path))
      .map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(oid.to_string())
  }

  #[napi]
  pub fn reflog(&self, name: String) -> napi::Result<Reflog> {
    let reflog = self.inner.reflog(&name).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Reflog::from_git2(&reflog))
  }

  #[napi]
  pub fn reflog_delete(&self, name: String) -> napi::Result<()> {
    self.inner.reflog_delete(&name).map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn blame_file(&self, path: String) -> napi::Result<Blame> {
    let blame = self
      .inner
      .blame_file(Path::new(&path), None)
      .map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Blame::from_git2(&blame))
  }

  #[napi]
  pub fn cleanup_state(&self) -> napi::Result<()> {
    self.inner.cleanup_state().map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn add_ignore_rule(&self, rules: String) -> napi::Result<()> {
    self.inner.add_ignore_rule(&rules).map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn clear_ignore_rules(&self) -> napi::Result<()> {
    self.inner.clear_ignore_rules().map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn is_path_ignored(&self, path: String) -> napi::Result<bool> {
    self.inner.is_path_ignored(path).map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn state(&self) -> String {
    format!("{:?}", self.inner.state())
  }

  #[napi]
  pub fn diff_tree_to_tree(
    &self,
    old_tree_hex: Option<String>,
    new_tree_hex: Option<String>,
  ) -> napi::Result<Diff> {
    let old_tree = if let Some(hex) = old_tree_hex {
      let oid = git2::Oid::from_str(&hex).map_err(|e| Error::from_reason(e.to_string()))?;
      Some(self.inner.find_tree(oid).map_err(|e| Error::from_reason(e.to_string()))?)
    } else {
      None
    };

    let new_tree = if let Some(hex) = new_tree_hex {
      let oid = git2::Oid::from_str(&hex).map_err(|e| Error::from_reason(e.to_string()))?;
      Some(self.inner.find_tree(oid).map_err(|e| Error::from_reason(e.to_string()))?)
    } else {
      None
    };

    let diff = self
      .inner
      .diff_tree_to_tree(old_tree.as_ref(), new_tree.as_ref(), None)
      .map_err(|e| Error::from_reason(e.to_string()))?;

    Ok(Diff::from_git2(&diff))
  }

  #[napi]
  pub fn diff_index_to_workdir(&self) -> napi::Result<Diff> {
    let diff = self
      .inner
      .diff_index_to_workdir(None, None)
      .map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Diff::from_git2(&diff))
  }
}
