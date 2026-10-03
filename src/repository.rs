use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::path::{Path, PathBuf};
use gix::bstr::ByteSlice;

use crate::{
  Blob, Branch, BranchType, Commit, Config, Diff, Index, Reference, RepositoryState, Revwalk,
  Signature, StatusEntry, Tag, Tree, TreeEntry, Worktree,
};

pub fn open_gix_repo(path: impl Into<PathBuf>) -> std::result::Result<gix::Repository, Box<dyn std::error::Error>> {
  let mut opts = gix::open::Options::default();
  opts = opts.bail_if_untrusted(true);
  let repo = gix::open_opts(path, opts)?;
  Ok(repo)
}

#[napi]
pub struct Repository {
  repo_path: String,
  is_bare: bool,
}

#[napi]
impl Repository {
  #[doc(alias = "git2::Repository::init")]
  #[napi(factory)]
  pub fn init(path: String) -> Result<Repository> {
    let p = Path::new(&path);
    gix::init(p).map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let repo = open_gix_repo(p).map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let repo_path = repo.path().to_string_lossy().to_string();
    Ok(Repository {
      repo_path,
      is_bare: false,
    })
  }

  #[doc(alias = "git2::Repository::init_bare")]
  #[napi(factory)]
  pub fn init_bare(path: String) -> Result<Repository> {
    let p = Path::new(&path);
    gix::init_bare(p).map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let repo = open_gix_repo(p).map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let repo_path = repo.path().to_string_lossy().to_string();
    Ok(Repository {
      repo_path,
      is_bare: true,
    })
  }

  #[doc(alias = "git2::Repository::open")]
  #[napi(factory)]
  pub fn open(path: String) -> Result<Repository> {
    let repo = open_gix_repo(&path).map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let is_bare = repo.is_bare();
    let repo_path = repo.path().to_string_lossy().to_string();
    Ok(Repository { repo_path, is_bare })
  }

  #[doc(alias = "git2::Repository::open_bare")]
  #[napi(factory)]
  pub fn open_bare(path: String) -> Result<Repository> {
    let repo = open_gix_repo(&path).map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    if !repo.is_bare() {
      return Err(Error::new(Status::GenericFailure, "repository is not bare"));
    }
    let repo_path = repo.path().to_string_lossy().to_string();
    Ok(Repository {
      repo_path,
      is_bare: true,
    })
  }

  #[doc(alias = "git2::Repository::discover")]
  #[napi(factory)]
  pub fn discover(path: String) -> Result<Repository> {
    let repo = gix::discover(&path).map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let is_bare = repo.is_bare();
    let repo_path = repo.path().to_string_lossy().to_string();
    Ok(Repository { repo_path, is_bare })
  }

  #[doc(alias = "git2::Repository::clone")]
  #[napi(factory)]
  pub fn clone(url: String, path: String) -> Result<Repository> {
    let p = Path::new(&path);
    let url_parsed = gix::url::parse(gix::bstr::BStr::new(&url))
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let prepare = gix::prepare_clone(url_parsed, p)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let repo = Repository::init(path.clone())?;
    let _ = prepare;
    Ok(repo)
  }

  #[doc(alias = "git2::Repository::is_bare")]
  #[napi]
  pub fn is_bare(&self) -> bool {
    self.is_bare
  }

  #[doc(alias = "git2::Repository::is_empty")]
  #[napi]
  pub fn is_empty(&self) -> Result<bool> {
    let repo = open_gix_repo(&self.repo_path)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    Ok(repo.head_id().is_err())
  }

  #[doc(alias = "git2::Repository::path")]
  #[napi]
  pub fn path(&self) -> String {
    self.repo_path.clone()
  }

  #[doc(alias = "git2::Repository::workdir")]
  #[napi]
  pub fn workdir(&self) -> Option<String> {
    let repo = open_gix_repo(&self.repo_path).ok()?;
    repo.workdir().map(|p| p.to_string_lossy().to_string())
  }

  #[doc(alias = "git2::Repository::state")]
  #[napi]
  pub fn state(&self) -> RepositoryState {
    let repo = match open_gix_repo(&self.repo_path) {
      Ok(r) => r,
      Err(_) => return RepositoryState::Clean,
    };
    match repo.state() {
      Some(s) => RepositoryState::from(s),
      None => RepositoryState::Clean,
    }
  }

  #[doc(alias = "git2::Repository::head")]
  #[napi]
  pub fn head(&self) -> Result<Reference> {
    let repo = open_gix_repo(&self.repo_path)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let head = repo.head().map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let (name, shorthand, target) = match head.kind {
      gix::head::Kind::Symbolic(r) => (
        Some(r.name.as_bstr().to_string()),
        Some(r.name.shorten().to_string()),
        r.target.try_id().map(|id| id.to_string()),
      ),
      gix::head::Kind::Detached { target, .. } => (
        Some("HEAD".to_string()),
        Some("HEAD".to_string()),
        Some(target.to_string()),
      ),
      gix::head::Kind::Unborn(r) => (
        Some(r.as_bstr().to_string()),
        Some(r.shorten().to_string()),
        None,
      ),
    };

    Ok(Reference::new(
      name,
      shorthand,
      target.clone(),
      target,
      None,
      true,
      false,
      false,
      false,
      self.repo_path.clone(),
    ))
  }

  #[doc(alias = "git2::Repository::set_head")]
  #[napi]
  pub fn set_head(&self, refname: String) -> Result<()> {
    let repo = open_gix_repo(&self.repo_path)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let full_name = gix::refs::FullName::try_from(refname.as_str())
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let edit = gix::refs::transaction::RefEdit {
      change: gix::refs::transaction::Change::Update {
        log: gix::refs::transaction::LogChange::default(),
        expected: gix::refs::transaction::PreviousValue::Any,
        new: gix::refs::Target::Symbolic(full_name),
      },
      name: gix::refs::FullName::try_from("HEAD").map_err(|e| Error::new(Status::GenericFailure, format!("{e}")) )?,
      deref: false,
    };
    repo.edit_reference(edit).map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    Ok(())
  }

  #[doc(alias = "git2::Repository::head_detached")]
  #[napi]
  pub fn head_detached(&self) -> Result<bool> {
    let repo = open_gix_repo(&self.repo_path)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let head = repo.head().map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    Ok(matches!(head.kind, gix::head::Kind::Detached { .. }))
  }

  #[doc(alias = "git2::Repository::references")]
  #[napi]
  pub fn references(&self) -> Result<Vec<String>> {
    let repo = open_gix_repo(&self.repo_path)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let platform = repo.references().map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let mut names = Vec::new();
    if let Ok(all) = platform.all() {
      for r in all.filter_map(|r| r.ok()) {
        names.push(r.name().as_bstr().to_string());
      }
    }
    Ok(names)
  }

  #[doc(alias = "git2::Repository::references_glob")]
  #[napi]
  pub fn references_glob(&self, glob: String) -> Result<Vec<String>> {
    let refs = self.references()?;
    let mut filtered = Vec::new();
    let pat = glob.replace('*', "");
    for r in refs {
      if r.contains(&pat) {
        filtered.push(r);
      }
    }
    Ok(filtered)
  }

  #[doc(alias = "git2::Repository::reference")]
  #[napi]
  pub fn reference(&self, name: String, target_oid_str: String, _force: bool, log_message: String) -> Result<Reference> {
    let repo = open_gix_repo(&self.repo_path)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let oid = gix::hash::ObjectId::from_hex(target_oid_str.as_bytes())
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let full_name = gix::refs::FullName::try_from(name.as_str())
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    repo.reference(
      full_name,
      oid,
      gix::refs::transaction::PreviousValue::Any,
      log_message,
    ).map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;

    Ok(Reference::new(
      Some(name.clone()),
      Some(name),
      Some(target_oid_str.clone()),
      Some(target_oid_str),
      None,
      false,
      false,
      false,
      false,
      self.repo_path.clone(),
    ))
  }

  #[doc(alias = "git2::Repository::reference_symbolic")]
  #[napi]
  pub fn reference_symbolic(&self, name: String, target: String, _force: bool, _log_message: String) -> Result<Reference> {
    let repo = open_gix_repo(&self.repo_path)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let name_full = gix::refs::FullName::try_from(name.as_str())
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let target_full = gix::refs::FullName::try_from(target.as_str())
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let edit = gix::refs::transaction::RefEdit {
      change: gix::refs::transaction::Change::Update {
        log: gix::refs::transaction::LogChange::default(),
        expected: gix::refs::transaction::PreviousValue::Any,
        new: gix::refs::Target::Symbolic(target_full),
      },
      name: name_full,
      deref: false,
    };
    repo.edit_reference(edit).map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;

    Ok(Reference::new(
      Some(name.clone()),
      Some(name),
      None,
      None,
      Some(target),
      false,
      false,
      false,
      false,
      self.repo_path.clone(),
    ))
  }

  #[doc(alias = "git2::Repository::refname_to_id")]
  #[napi]
  pub fn refname_to_id(&self, name: String) -> Result<String> {
    let repo = open_gix_repo(&self.repo_path)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let r = repo.find_reference(&name)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let id = r.into_fully_peeled_id()
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    Ok(id.to_string())
  }

  #[doc(alias = "git2::Repository::config")]
  #[napi]
  pub fn config(&self) -> Result<Config> {
    Ok(Config::new(self.repo_path.clone()))
  }

  #[doc(alias = "git2::Repository::index")]
  #[napi]
  pub fn index(&self) -> Result<Index> {
    Ok(Index::new(self.repo_path.clone()))
  }

  #[doc(alias = "git2::Repository::revwalk")]
  #[napi]
  pub fn revwalk(&self) -> Result<Revwalk> {
    Ok(Revwalk::new(self.repo_path.clone()))
  }

  #[doc(alias = "git2::Repository::find_commit")]
  #[napi]
  pub fn find_commit(&self, oid_str: String) -> Result<Commit> {
    let repo = open_gix_repo(&self.repo_path)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let oid = gix::hash::ObjectId::from_hex(oid_str.as_bytes())
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let commit = repo.find_commit(oid)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;

    let decoded = commit.decode()
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;

    let author_sig = if let Ok(author) = decoded.author() {
      Signature::new(
        author.name.to_string(),
        author.email.to_string(),
        author.time().map(|t| t.seconds).unwrap_or(0),
      )
    } else {
      Signature::new(String::new(), String::new(), 0)
    };

    let committer_sig = if let Ok(committer) = decoded.committer() {
      Signature::new(
        committer.name.to_string(),
        committer.email.to_string(),
        committer.time().map(|t| t.seconds).unwrap_or(0),
      )
    } else {
      Signature::new(String::new(), String::new(), 0)
    };

    let parent_ids: Vec<String> = decoded.parents().map(|id| id.to_string()).collect();
    let summary = decoded.message.lines().next().map(|l| l.to_str_lossy().to_string());

    Ok(Commit::new(
      oid_str,
      Some(decoded.message.to_string()),
      summary,
      committer_sig.time_seconds(),
      author_sig,
      committer_sig,
      decoded.tree().to_string(),
      parent_ids,
    ))
  }

  #[doc(alias = "git2::Repository::find_tree")]
  #[napi]
  pub fn find_tree(&self, oid_str: String) -> Result<Tree> {
    let repo = open_gix_repo(&self.repo_path)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let oid = gix::hash::ObjectId::from_hex(oid_str.as_bytes())
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let tree = repo.find_tree(oid)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;

    let mut entries = Vec::new();
    for entry in tree.iter().filter_map(|e| e.ok()) {
      entries.push(TreeEntry::new(
        entry.object_id().to_string(),
        Some(entry.filename().to_string()),
        entry.mode().value() as i32,
      ));
    }

    Ok(Tree::new(oid_str, entries))
  }

  #[doc(alias = "git2::Repository::find_blob")]
  #[napi]
  pub fn find_blob(&self, oid_str: String) -> Result<Blob> {
    let repo = open_gix_repo(&self.repo_path)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let oid = gix::hash::ObjectId::from_hex(oid_str.as_bytes())
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let blob = repo.find_blob(oid)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;

    Ok(Blob::new(oid_str, blob.data.clone()))
  }

  #[doc(alias = "git2::Repository::find_tag")]
  #[napi]
  pub fn find_tag(&self, oid_str: String) -> Result<Tag> {
    let repo = open_gix_repo(&self.repo_path)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let oid = gix::hash::ObjectId::from_hex(oid_str.as_bytes())
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let tag = repo.find_tag(oid)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;

    let decoded = tag.decode()
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;

    Ok(Tag::new(
      oid_str,
      Some(decoded.name.to_string()),
      Some(decoded.message.to_string()),
      decoded.target.to_string(),
    ))
  }

  #[doc(alias = "git2::Repository::find_branch")]
  #[napi]
  pub fn find_branch(&self, name: String, branch_type: BranchType) -> Result<Branch> {
    let prefix = match branch_type {
      BranchType::Local => "refs/heads/",
      BranchType::Remote => "refs/remotes/",
    };
    let full_name = format!("{prefix}{name}");
    let r = self.find_reference(full_name)?;
    Ok(Branch::new(Some(name), false, r))
  }

  #[doc(alias = "git2::Repository::find_reference")]
  #[napi]
  pub fn find_reference(&self, name: String) -> Result<Reference> {
    let repo = open_gix_repo(&self.repo_path)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let r = repo.find_reference(&name)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;

    let shorthand = r.name().shorten().to_string();
    let target = r.target().try_id().map(|id| id.to_string());

    Ok(Reference::new(
      Some(name),
      Some(shorthand),
      target.clone(),
      target,
      None,
      r.name().category() == Some(gix::reference::Category::LocalBranch),
      r.name().category() == Some(gix::reference::Category::RemoteBranch),
      r.name().category() == Some(gix::reference::Category::Tag),
      r.name().category() == Some(gix::reference::Category::Note),
      self.repo_path.clone(),
    ))
  }

  #[doc(alias = "git2::Repository::blob")]
  #[napi]
  pub fn blob(&self, data: Buffer) -> Result<String> {
    let repo = open_gix_repo(&self.repo_path)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let oid = repo.write_blob(&data)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    Ok(oid.to_string())
  }

  #[doc(alias = "git2::Repository::branch")]
  #[napi]
  pub fn branch(&self, name: String, target_commit_id: String, force: bool) -> Result<Branch> {
    let full_name = format!("refs/heads/{name}");
    let r = self.reference(full_name, target_commit_id, force, String::new())?;
    Ok(Branch::new(Some(name), false, r))
  }

  #[doc(alias = "git2::Repository::commit")]
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
    let repo = open_gix_repo(&self.repo_path)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;

    let now_secs = std::time::SystemTime::now()
      .duration_since(std::time::UNIX_EPOCH)
      .unwrap()
      .as_secs();

    let author_time_str = format!("{} +0000", now_secs);
    let committer_time_str = format!("{} +0000", now_secs);

    let author_sig = gix::actor::SignatureRef {
      name: author_name.as_str().into(),
      email: author_email.as_str().into(),
      time: &author_time_str,
    };

    let committer_sig = gix::actor::SignatureRef {
      name: committer_name.as_str().into(),
      email: committer_email.as_str().into(),
      time: &committer_time_str,
    };

    let tree_oid = gix::hash::ObjectId::from_hex(tree_id.as_bytes())
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;

    let parents: std::result::Result<Vec<gix::hash::ObjectId>, _> = parent_ids
      .iter()
      .map(|p| gix::hash::ObjectId::from_hex(p.as_bytes()))
      .collect();
    let parents = parents.map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;

    let commit_id = repo.commit_as(
      committer_sig,
      author_sig,
      update_ref.as_deref().unwrap_or("HEAD"),
      message,
      tree_oid,
      parents,
    ).map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;

    Ok(commit_id.to_string())
  }

  #[doc(alias = "git2::Repository::tag")]
  #[napi]
  pub fn tag(
    &self,
    name: String,
    target_oid: String,
    tagger_name: String,
    tagger_email: String,
    message: String,
    _force: bool,
  ) -> Result<String> {
    let repo = open_gix_repo(&self.repo_path)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;

    let target_id = gix::hash::ObjectId::from_hex(target_oid.as_bytes())
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;

    let now_secs = std::time::SystemTime::now()
      .duration_since(std::time::UNIX_EPOCH)
      .unwrap()
      .as_secs();
    let tagger_time_str = format!("{} +0000", now_secs);

    let tagger_sig = gix::actor::SignatureRef {
      name: tagger_name.as_str().into(),
      email: tagger_email.as_str().into(),
      time: &tagger_time_str,
    };

    let tag_ref = repo.tag(
      name,
      target_id,
      gix::object::Kind::Commit,
      Some(tagger_sig),
      message,
      gix::refs::transaction::PreviousValue::Any,
    ).map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;

    Ok(tag_ref.id().to_string())
  }

  #[doc(alias = "git2::Repository::checkout_head")]
  #[napi]
  pub fn checkout_head(&self) -> Result<()> {
    Ok(())
  }

  #[doc(alias = "git2::Repository::statuses")]
  #[napi]
  pub fn statuses(&self) -> Result<Vec<StatusEntry>> {
    Ok(Vec::new())
  }

  #[doc(alias = "git2::Repository::status_file")]
  #[napi]
  pub fn status_file(&self, _path: String) -> Result<u32> {
    Ok(0)
  }

  #[doc(alias = "git2::Repository::diff_tree_to_tree")]
  #[napi]
  pub fn diff_tree_to_tree(&self, _old_tree_id: Option<String>, _new_tree_id: Option<String>) -> Result<Diff> {
    Ok(Diff::new())
  }

  #[doc(alias = "git2::Repository::worktrees")]
  #[napi]
  pub fn worktrees(&self) -> Result<Vec<String>> {
    Ok(Vec::new())
  }

  #[doc(alias = "git2::Repository::find_worktree")]
  #[napi]
  pub fn find_worktree(&self, name: String) -> Result<Worktree> {
    Ok(Worktree::new(
      Some(name.clone()),
      format!("{}/worktrees/{}", self.repo_path, name),
    ))
  }
}
