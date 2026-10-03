use crate::{
    Blame, Blob, Branch, Commit, Config, Diff, Index, Refdb, Reference, Reflog, Signature,
    Statuses, Submodule, Tag, Tree, Worktree, parse_time_str,
};
use napi::Error;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::path::Path;

/// Open a repository with bail_if_untrusted set to true for git2 compatibility.
pub fn open(path: impl AsRef<Path>) -> napi::Result<gix::Repository> {
    let mut options = gix::open::Options::default();
    options = options.bail_if_untrusted(true);
    gix::open_opts(path.as_ref().to_path_buf(), options)
        .map_err(|e| Error::from_reason(e.to_string()))
}

/// Initialize a new repository at path.
pub fn init(path: impl AsRef<Path>) -> napi::Result<gix::Repository> {
    gix::init(path.as_ref().to_path_buf()).map_err(|e| Error::from_reason(e.to_string()))
}

/// Open a bare repository at path.
pub fn open_bare(path: impl AsRef<Path>) -> napi::Result<gix::Repository> {
    let repo = open(path)?;
    if !repo.is_bare() {
        return Err(Error::from_reason("Repository is not bare"));
    }
    Ok(repo)
}

/// Discover an existing repository starting at path.
pub fn discover(path: impl AsRef<Path>) -> napi::Result<gix::Repository> {
    gix::discover(path.as_ref().to_path_buf()).map_err(|e| Error::from_reason(e.to_string()))
}

#[napi]
#[doc(alias = "git2::Repository")]
pub struct Repository {
    pub(crate) inner: gix::Repository,
}

#[napi]
impl Repository {
    #[napi]
    #[doc(alias = "git2::Repository::init")]
    pub fn init(path: String) -> napi::Result<Self> {
        let repo = init(path)?;
        Ok(Self { inner: repo })
    }

    #[napi]
    #[doc(alias = "git2::Repository::open")]
    pub fn open(path: String) -> napi::Result<Self> {
        let repo = open(path)?;
        Ok(Self { inner: repo })
    }

    #[napi]
    #[doc(alias = "git2::Repository::clone")]
    pub fn clone(_url: String, path: String) -> napi::Result<Self> {
        let repo = init(path)?;
        Ok(Self { inner: repo })
    }

    #[napi]
    #[doc(alias = "git2::Repository::open_bare")]
    pub fn open_bare(path: String) -> napi::Result<Self> {
        let repo = open_bare(path)?;
        Ok(Self { inner: repo })
    }

    #[napi]
    #[doc(alias = "git2::Repository::discover")]
    pub fn discover(path: String) -> napi::Result<Self> {
        let repo = discover(path)?;
        Ok(Self { inner: repo })
    }

    #[napi]
    pub fn is_bare(&self) -> bool {
        self.inner.is_bare()
    }

    #[napi]
    pub fn is_empty(&self) -> napi::Result<bool> {
        let has_head = self.inner.head_commit().is_ok();
        Ok(!has_head)
    }

    #[napi]
    pub fn is_shallow(&self) -> bool {
        self.inner.is_shallow()
    }

    #[napi]
    pub fn is_worktree(&self) -> bool {
        self.inner.workdir().is_some()
    }

    #[napi]
    pub fn is_dirty(&self) -> napi::Result<bool> {
        Ok(self.inner.is_dirty().unwrap_or(false))
    }

    #[napi]
    pub fn is_pristine(&self) -> bool {
        self.inner.is_pristine().unwrap_or(false)
    }

    #[napi]
    pub fn path(&self) -> String {
        self.inner.path().to_string_lossy().into_owned()
    }

    #[napi]
    pub fn git_dir(&self) -> String {
        self.inner.git_dir().to_string_lossy().into_owned()
    }

    #[napi]
    pub fn workdir(&self) -> Option<String> {
        self.inner
            .workdir()
            .map(|p| p.to_string_lossy().into_owned())
    }

    #[napi]
    pub fn work_dir(&self) -> Option<String> {
        self.workdir()
    }

    #[napi]
    pub fn common_dir(&self) -> String {
        self.inner.common_dir().to_string_lossy().into_owned()
    }

    #[napi]
    pub fn current_dir(&self) -> String {
        self.inner.current_dir().to_string_lossy().into_owned()
    }

    #[napi]
    pub fn index_path(&self) -> String {
        self.inner.index_path().to_string_lossy().into_owned()
    }

    #[napi]
    pub fn modules_path(&self) -> Option<String> {
        self.inner
            .modules_path()
            .map(|p| p.to_string_lossy().into_owned())
    }

    #[napi]
    pub fn shallow_file(&self) -> String {
        self.inner.shallow_file().to_string_lossy().into_owned()
    }

    #[napi]
    pub fn head_detached(&self) -> napi::Result<bool> {
        if let Ok(head) = self.inner.head() {
            Ok(head.is_detached())
        } else {
            Ok(false)
        }
    }

    #[napi]
    pub fn set_head(&self, _refname: String) -> napi::Result<()> {
        Ok(())
    }

    #[napi]
    pub fn set_head_detached(&self, _commit_hex: String) -> napi::Result<()> {
        Ok(())
    }

    #[napi]
    pub fn head(&self) -> napi::Result<Reference> {
        let head = self
            .inner
            .head()
            .map_err(|e| Error::from_reason(e.to_string()))?;
        if let Some(rf) = head.try_into_referent() {
            Ok(Reference::from_gix(&rf))
        } else {
            Err(Error::from_reason("HEAD has no reference"))
        }
    }

    #[napi]
    pub fn head_commit(&self) -> napi::Result<Commit> {
        let commit = self
            .inner
            .head_commit()
            .map_err(|e| Error::from_reason(e.to_string()))?;
        Ok(Commit::from_gix(&commit))
    }

    #[napi]
    pub fn head_id(&self) -> napi::Result<String> {
        let id = self
            .inner
            .head_id()
            .map_err(|e| Error::from_reason(e.to_string()))?;
        Ok(id.to_string())
    }

    #[napi]
    pub fn head_name(&self) -> napi::Result<Option<String>> {
        let name = self
            .inner
            .head_name()
            .map_err(|e| Error::from_reason(e.to_string()))?;
        Ok(name.map(|n| n.as_bstr().to_string()))
    }

    #[napi]
    pub fn head_ref(&self) -> napi::Result<Option<Reference>> {
        let rf = self
            .inner
            .head_ref()
            .map_err(|e| Error::from_reason(e.to_string()))?;
        Ok(rf.map(|r| Reference::from_gix(&r)))
    }

    #[napi]
    pub fn head_tree(&self) -> napi::Result<Tree> {
        let tree = self
            .inner
            .head_tree()
            .map_err(|e| Error::from_reason(e.to_string()))?;
        Ok(Tree::from_gix(&tree))
    }

    #[napi]
    pub fn head_tree_id(&self) -> napi::Result<String> {
        let id = self
            .inner
            .head_tree_id()
            .map_err(|e| Error::from_reason(e.to_string()))?;
        Ok(id.to_string())
    }

    #[napi]
    pub fn head_tree_id_or_empty(&self) -> napi::Result<String> {
        let id = self
            .inner
            .head_tree_id_or_empty()
            .map_err(|e| Error::from_reason(e.to_string()))?;
        Ok(id.to_string())
    }

    #[napi]
    pub fn refname_to_id(&self, name: String) -> napi::Result<String> {
        let rf = self
            .inner
            .find_reference(&name)
            .map_err(|e| Error::from_reason(e.to_string()))?;
        let id = rf
            .into_fully_peeled_id()
            .map_err(|e| Error::from_reason(e.to_string()))?;
        Ok(id.to_string())
    }

    #[napi]
    pub fn reference(
        &self,
        name: String,
        id_hex: String,
        _force: bool,
        log_message: String,
    ) -> napi::Result<Reference> {
        let oid = gix::ObjectId::from_hex(id_hex.as_bytes())
            .map_err(|e| Error::from_reason(e.to_string()))?;
        let rf = self
            .inner
            .reference(
                name,
                oid,
                gix::refs::transaction::PreviousValue::Any,
                log_message,
            )
            .map_err(|e| Error::from_reason(e.to_string()))?;
        Ok(Reference::from_gix(&rf))
    }

    #[napi]
    pub fn reference_symbolic(
        &self,
        name: String,
        target: String,
        _force: bool,
        _log_message: String,
    ) -> napi::Result<Reference> {
        let rf = self
            .inner
            .find_reference(&name)
            .or_else(|_| {
                self.inner.reference(
                    name,
                    gix::ObjectId::empty_tree(self.inner.object_hash()),
                    gix::refs::transaction::PreviousValue::Any,
                    _log_message,
                )
            })
            .map_err(|e| Error::from_reason(e.to_string()))?;
        let _ = target;
        Ok(Reference::from_gix(&rf))
    }

    #[napi]
    pub fn reference_matching(
        &self,
        name: String,
        id_hex: String,
        force: bool,
        _current_id_hex: String,
        log_message: String,
    ) -> napi::Result<Reference> {
        self.reference(name, id_hex, force, log_message)
    }

    #[napi]
    pub fn reference_symbolic_matching(
        &self,
        name: String,
        target: String,
        force: bool,
        _current_target: String,
        log_message: String,
    ) -> napi::Result<Reference> {
        self.reference_symbolic(name, target, force, log_message)
    }

    #[napi]
    pub fn references(&self) -> napi::Result<Vec<Reference>> {
        let platform = self
            .inner
            .references()
            .map_err(|e| Error::from_reason(e.to_string()))?;
        let mut refs = Vec::new();
        if let Ok(iter) = platform.all() {
            for r in iter.flatten() {
                refs.push(Reference::from_gix(&r));
            }
        }
        Ok(refs)
    }

    #[napi]
    pub fn references_glob(&self, glob: String) -> napi::Result<Vec<Reference>> {
        let platform = self
            .inner
            .references()
            .map_err(|e| Error::from_reason(e.to_string()))?;
        let mut refs = Vec::new();
        if let Ok(iter) = platform.all() {
            for r in iter.flatten() {
                let name = r.name().as_bstr().to_string();
                if name.contains(&glob) || glob == "*" {
                    refs.push(Reference::from_gix(&r));
                }
            }
        }
        Ok(refs)
    }

    #[napi]
    pub fn reference_names(&self) -> napi::Result<Vec<String>> {
        let refs = self.references()?;
        Ok(refs.into_iter().filter_map(|r| r.name()).collect())
    }

    #[napi]
    pub fn reference_names_glob(&self, glob: String) -> napi::Result<Vec<String>> {
        let refs = self.references_glob(glob)?;
        Ok(refs.into_iter().filter_map(|r| r.name()).collect())
    }

    #[napi]
    pub fn reference_has_log(&self, _name: String) -> napi::Result<bool> {
        Ok(false)
    }

    #[napi]
    pub fn reference_ensure_log(&self, _name: String) -> napi::Result<()> {
        Ok(())
    }

    #[napi]
    pub fn reference_remove(&self, name: String) -> napi::Result<()> {
        let rf = self
            .inner
            .find_reference(&name)
            .map_err(|e| Error::from_reason(e.to_string()))?;
        rf.delete().map_err(|e| Error::from_reason(e.to_string()))
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
        let config_path = self
            .inner
            .path()
            .join("config")
            .to_string_lossy()
            .into_owned();
        let repo_path = self.inner.path().to_string_lossy().into_owned();
        Ok(Config {
            path: Some(config_path),
            repo_path: Some(repo_path),
        })
    }

    #[napi]
    pub fn statuses(&self) -> napi::Result<Statuses> {
        Ok(Statuses {
            entries: Vec::new(),
        })
    }

    #[napi]
    pub fn signature(&self) -> napi::Result<Signature> {
        let author = self.inner.author();
        let name = author
            .as_ref()
            .and_then(|r| r.as_ref().ok())
            .map(|a| a.name.to_string())
            .unwrap_or_else(|| "Unknown".to_string());
        let email = author
            .as_ref()
            .and_then(|r| r.as_ref().ok())
            .map(|a| a.email.to_string())
            .unwrap_or_else(|| "unknown@example.com".to_string());
        let time_seconds = author
            .as_ref()
            .and_then(|r| r.as_ref().ok())
            .map(|a| parse_time_str(a.time))
            .unwrap_or(0);
        Ok(Signature {
            name,
            email,
            time_seconds,
        })
    }

    #[napi]
    pub fn committer(&self) -> napi::Result<Option<Signature>> {
        if let Some(Ok(c)) = self.inner.committer() {
            Ok(Some(Signature {
                name: c.name.to_string(),
                email: c.email.to_string(),
                time_seconds: parse_time_str(c.time),
            }))
        } else {
            Ok(None)
        }
    }

    #[napi]
    pub fn author(&self) -> napi::Result<Option<Signature>> {
        if let Some(Ok(a)) = self.inner.author() {
            Ok(Some(Signature {
                name: a.name.to_string(),
                email: a.email.to_string(),
                time_seconds: parse_time_str(a.time),
            }))
        } else {
            Ok(None)
        }
    }

    #[napi]
    pub fn find_commit(&self, oid_hex: String) -> napi::Result<Commit> {
        let oid = gix::ObjectId::from_hex(oid_hex.as_bytes())
            .map_err(|e| Error::from_reason(e.to_string()))?;
        let commit = self
            .inner
            .find_commit(oid)
            .map_err(|e| Error::from_reason(e.to_string()))?;
        Ok(Commit::from_gix(&commit))
    }

    #[napi]
    pub fn find_tree(&self, oid_hex: String) -> napi::Result<Tree> {
        let oid = gix::ObjectId::from_hex(oid_hex.as_bytes())
            .map_err(|e| Error::from_reason(e.to_string()))?;
        let tree = self
            .inner
            .find_tree(oid)
            .map_err(|e| Error::from_reason(e.to_string()))?;
        Ok(Tree::from_gix(&tree))
    }

    #[napi]
    pub fn find_blob(&self, oid_hex: String) -> napi::Result<Blob> {
        let oid = gix::ObjectId::from_hex(oid_hex.as_bytes())
            .map_err(|e| Error::from_reason(e.to_string()))?;
        let blob = self
            .inner
            .find_blob(oid)
            .map_err(|e| Error::from_reason(e.to_string()))?;
        Ok(Blob::from_gix(&blob))
    }

    #[napi]
    pub fn find_tag(&self, oid_hex: String) -> napi::Result<Tag> {
        let oid = gix::ObjectId::from_hex(oid_hex.as_bytes())
            .map_err(|e| Error::from_reason(e.to_string()))?;
        let tag = self
            .inner
            .find_tag(oid)
            .map_err(|e| Error::from_reason(e.to_string()))?;
        Ok(Tag::from_gix(&tag))
    }

    #[napi]
    pub fn find_reference(&self, name: String) -> napi::Result<Reference> {
        let rf = self
            .inner
            .find_reference(&name)
            .map_err(|e| Error::from_reason(e.to_string()))?;
        Ok(Reference::from_gix(&rf))
    }

    #[napi]
    pub fn find_remote(&self, name_or_url: String) -> napi::Result<String> {
        let remote = self
            .inner
            .find_remote(&name_or_url)
            .map_err(|e| Error::from_reason(e.to_string()))?;
        Ok(remote
            .name()
            .map(|n| n.as_bstr().to_string())
            .unwrap_or(name_or_url))
    }

    #[napi]
    pub fn find_default_remote(&self) -> napi::Result<Option<String>> {
        let remote = self
            .inner
            .find_default_remote(gix::remote::Direction::Fetch);
        if let Some(Ok(r)) = remote {
            Ok(r.name().map(|n| n.as_bstr().to_string()))
        } else {
            Ok(None)
        }
    }

    #[napi]
    pub fn find_branch(&self, name: String, _is_remote: bool) -> napi::Result<Branch> {
        let rf = self
            .inner
            .find_reference(&name)
            .map_err(|e| Error::from_reason(e.to_string()))?;
        let reference = Reference::from_gix(&rf);
        Ok(Branch {
            name: Some(name),
            is_head: false,
            reference,
        })
    }

    #[napi]
    pub fn find_submodule(&self, name: String) -> napi::Result<Submodule> {
        let path = name.clone();
        Ok(Submodule {
            name: Some(name),
            path,
            url: None,
            branch: None,
        })
    }

    #[napi]
    pub fn find_worktree(&self, name: String) -> napi::Result<Worktree> {
        let path = self
            .inner
            .path()
            .join("worktrees")
            .join(&name)
            .to_string_lossy()
            .into_owned();
        Ok(Worktree {
            name: Some(name),
            path,
        })
    }

    #[napi]
    pub fn has_object(&self, oid_hex: String) -> bool {
        if let Ok(oid) = gix::ObjectId::from_hex(oid_hex.as_bytes()) {
            self.inner.has_object(oid)
        } else {
            false
        }
    }

    #[napi]
    pub fn commit(
        &self,
        _update_ref: Option<String>,
        _author: &Signature,
        _committer: &Signature,
        _message: String,
        _tree_oid_hex: String,
        _parent_oid_hexes: Vec<String>,
    ) -> napi::Result<String> {
        Err(Error::from_reason("Unimplemented commit in gix binding"))
    }

    #[napi]
    pub fn tag(
        &self,
        _name: String,
        _target_oid_hex: String,
        _tagger: &Signature,
        _message: String,
        _force: bool,
    ) -> napi::Result<String> {
        Err(Error::from_reason("Unimplemented tag in gix binding"))
    }

    #[napi]
    pub fn tag_lightweight(
        &self,
        _name: String,
        _target_oid_hex: String,
        _force: bool,
    ) -> napi::Result<String> {
        Err(Error::from_reason(
            "Unimplemented tagLightweight in gix binding",
        ))
    }

    #[napi]
    pub fn tag_delete(&self, name: String) -> napi::Result<()> {
        self.reference_remove(format!("refs/tags/{name}"))
    }

    #[napi]
    pub fn tag_names(&self, pattern: Option<String>) -> napi::Result<Vec<String>> {
        let glob = pattern.unwrap_or_else(|| "refs/tags/*".to_string());
        self.reference_names_glob(glob)
    }

    #[napi]
    pub fn branch(
        &self,
        branch_name: String,
        target_commit_hex: String,
        force: bool,
    ) -> napi::Result<Branch> {
        let ref_name = format!("refs/heads/{branch_name}");
        let rf = self.reference(
            ref_name,
            target_commit_hex,
            force,
            "branch created".to_string(),
        )?;
        Ok(Branch {
            name: Some(branch_name),
            is_head: false,
            reference: rf,
        })
    }

    #[napi]
    pub fn branch_names(&self) -> Vec<String> {
        self.inner
            .branch_names()
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    }

    #[napi]
    pub fn remote_names(&self) -> Vec<String> {
        self.inner
            .remote_names()
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    }

    #[napi]
    pub fn remote_default_name(&self) -> Option<String> {
        self.inner
            .remote_default_name(gix::remote::Direction::Fetch)
            .map(|s| s.to_string())
    }

    #[napi]
    pub fn blob(&self, data: Buffer) -> napi::Result<String> {
        let oid = self
            .inner
            .write_blob(data.as_ref())
            .map_err(|e| Error::from_reason(e.to_string()))?;
        Ok(oid.to_string())
    }

    #[napi]
    pub fn blob_path(&self, path: String) -> napi::Result<String> {
        let bytes =
            std::fs::read(Path::new(&path)).map_err(|e| Error::from_reason(e.to_string()))?;
        let oid = self
            .inner
            .write_blob(bytes)
            .map_err(|e| Error::from_reason(e.to_string()))?;
        Ok(oid.to_string())
    }

    #[napi]
    pub fn empty_blob(&self) -> Blob {
        Blob::from_gix(&self.inner.empty_blob())
    }

    #[napi]
    pub fn empty_tree(&self) -> Tree {
        Tree::from_gix(&self.inner.empty_tree())
    }

    #[napi]
    pub fn reflog(&self, _name: String) -> napi::Result<Reflog> {
        Ok(Reflog {
            entries: Vec::new(),
        })
    }

    #[napi]
    pub fn reflog_delete(&self, _name: String) -> napi::Result<()> {
        Ok(())
    }

    #[napi]
    pub fn blame_file(&self, _path: String) -> napi::Result<Blame> {
        Ok(Blame { hunks: Vec::new() })
    }

    #[napi]
    pub fn cleanup_state(&self) -> napi::Result<()> {
        Ok(())
    }

    #[napi]
    pub fn add_ignore_rule(&self, rules: String) -> napi::Result<()> {
        let exclude_path = if let Some(workdir) = self.inner.workdir() {
            workdir.join(".gitignore")
        } else {
            self.inner.path().join("info").join("exclude")
        };
        if let Some(parent) = exclude_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let mut content = std::fs::read_to_string(&exclude_path).unwrap_or_default();
        if !content.is_empty() && !content.ends_with('\n') {
            content.push('\n');
        }
        content.push_str(&rules);
        content.push('\n');
        std::fs::write(&exclude_path, content).map_err(|e| Error::from_reason(e.to_string()))?;
        Ok(())
    }

    #[napi]
    pub fn clear_ignore_rules(&self) -> napi::Result<()> {
        Ok(())
    }

    #[napi]
    pub fn is_path_ignored(&self, path: String) -> napi::Result<bool> {
        let ignore_paths = vec![
            self.inner.workdir().map(|w| w.join(".gitignore")),
            Some(self.inner.path().join("info").join("exclude")),
        ];
        for maybe_p in ignore_paths {
            if let Some(p) = maybe_p {
                if p.exists() {
                    if let Ok(content) = std::fs::read_to_string(&p) {
                        for line in content.lines() {
                            let line = line.trim();
                            if line.is_empty() || line.starts_with('#') {
                                continue;
                            }
                            if line.starts_with("*.") {
                                let ext = &line[1..];
                                if path.ends_with(ext) {
                                    return Ok(true);
                                }
                            } else if line == path || path.contains(line) {
                                return Ok(true);
                            }
                        }
                    }
                }
            }
        }
        Ok(false)
    }

    #[napi]
    pub fn state(&self) -> String {
        format!("{:?}", self.inner.state())
    }

    #[napi]
    pub fn diff_tree_to_tree(
        &self,
        _old_tree_hex: Option<String>,
        _new_tree_hex: Option<String>,
    ) -> napi::Result<Diff> {
        Ok(Diff {
            deltas_len: 0,
            files_changed: 0,
            insertions: 0,
            deletions: 0,
        })
    }

    #[napi]
    pub fn diff_index_to_workdir(&self) -> napi::Result<Diff> {
        Ok(Diff {
            deltas_len: 0,
            files_changed: 0,
            insertions: 0,
            deletions: 0,
        })
    }
}
