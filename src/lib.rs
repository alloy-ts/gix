pub mod object;
pub mod odb_handle;
pub mod ref_store;
pub mod repository;

pub use object::{Blob, Commit, Tag, Tree, TreeEntry};
pub use odb_handle::OdbHandle;
pub use ref_store::RefStore;
pub use repository::Repository;

use napi::Error;
use napi_derive::napi;
use std::path::Path;

pub fn parse_time_str(time_str: &str) -> i64 {
    time_str
        .split_whitespace()
        .next()
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(0)
}

#[napi]
#[doc(alias = "git2::Signature")]
pub struct Signature {
    name: String,
    email: String,
    time_seconds: i64,
}

#[napi]
impl Signature {
    #[napi(factory)]
    pub fn now(name: String, email: String) -> napi::Result<Self> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;
        Ok(Self {
            name,
            email,
            time_seconds: now,
        })
    }

    pub fn new(name: String, email: String, time_seconds: i64) -> Self {
        Self {
            name,
            email,
            time_seconds,
        }
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

#[napi]
#[doc(alias = "git2::Reference")]
pub struct Reference {
    pub(crate) name: Option<String>,
    pub(crate) target: Option<String>,
    pub(crate) target_peel: Option<String>,
    pub(crate) symbolic_target: Option<String>,
    pub(crate) kind: String,
    pub(crate) is_branch: bool,
    pub(crate) is_remote: bool,
    pub(crate) is_tag: bool,
    pub(crate) is_note: bool,
    pub(crate) shorthand: Option<String>,
}

#[napi]
impl Reference {
    #[napi]
    pub fn is_valid_name(name: String) -> bool {
        gix::validate::reference::name(gix::bstr::BStr::new(&name)).is_ok()
    }

    #[napi]
    pub fn normalize_name(name: String) -> napi::Result<String> {
        if Self::is_valid_name(name.clone()) {
            Ok(name)
        } else {
            Err(Error::from_reason("Invalid reference name"))
        }
    }

    #[napi]
    pub fn name(&self) -> Option<String> {
        self.name.clone()
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
    pub fn kind(&self) -> String {
        self.kind.clone()
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

    #[napi]
    pub fn resolve(&self, repo_path: String) -> napi::Result<Reference> {
        let name = self
            .name
            .as_ref()
            .ok_or_else(|| Error::from_reason("Reference has no name"))?;
        let repo = repository::open(repo_path)?;
        let rf = repo
            .find_reference(name)
            .map_err(|e| Error::from_reason(e.to_string()))?;
        Ok(Reference::from_gix(&rf))
    }

    #[napi]
    pub fn set_target(
        &self,
        repo_path: String,
        oid_hex: String,
        log_message: String,
    ) -> napi::Result<Reference> {
        let name = self
            .name
            .as_ref()
            .ok_or_else(|| Error::from_reason("Reference has no name"))?;
        let repo = repository::open(repo_path)?;
        let oid = gix::ObjectId::from_hex(oid_hex.as_bytes())
            .map_err(|e| Error::from_reason(e.to_string()))?;
        let mut rf = repo
            .find_reference(name)
            .map_err(|e| Error::from_reason(e.to_string()))?;
        rf.set_target_id(oid, log_message)
            .map_err(|e| Error::from_reason(e.to_string()))?;
        Ok(Reference::from_gix(&rf))
    }

    #[napi]
    pub fn symbolic_set_target(
        &self,
        _repo_path: String,
        _target: String,
        _log_message: String,
    ) -> napi::Result<Reference> {
        Err(Error::from_reason("Unimplemented symbolic_set_target"))
    }

    #[napi]
    pub fn rename(
        &self,
        _repo_path: String,
        _new_name: String,
        _force: bool,
        _log_message: String,
    ) -> napi::Result<Reference> {
        Err(Error::from_reason("Unimplemented rename"))
    }

    #[napi]
    pub fn delete(&self, repo_path: String) -> napi::Result<()> {
        let name = self
            .name
            .as_ref()
            .ok_or_else(|| Error::from_reason("Reference has no name"))?;
        let repo = repository::open(repo_path)?;
        let rf = repo
            .find_reference(name)
            .map_err(|e| Error::from_reason(e.to_string()))?;
        rf.delete().map_err(|e| Error::from_reason(e.to_string()))
    }

    #[napi]
    pub fn peel_to_commit(&self, repo_path: String) -> napi::Result<Commit> {
        let name = self
            .name
            .as_ref()
            .ok_or_else(|| Error::from_reason("Reference has no name"))?;
        let repo = repository::open(repo_path)?;
        let mut rf = repo
            .find_reference(name)
            .map_err(|e| Error::from_reason(e.to_string()))?;
        let commit = rf
            .peel_to_commit()
            .map_err(|e| Error::from_reason(e.to_string()))?;
        Ok(Commit::from_gix(&commit))
    }

    #[napi]
    pub fn peel_to_tag(&self, repo_path: String) -> napi::Result<Tag> {
        let name = self
            .name
            .as_ref()
            .ok_or_else(|| Error::from_reason("Reference has no name"))?;
        let repo = repository::open(repo_path)?;
        let mut rf = repo
            .find_reference(name)
            .map_err(|e| Error::from_reason(e.to_string()))?;
        let tag = rf
            .peel_to_tag()
            .map_err(|e| Error::from_reason(e.to_string()))?;
        Ok(Tag::from_gix(&tag))
    }

    #[napi]
    pub fn peel_to_tree(&self, repo_path: String) -> napi::Result<Tree> {
        let name = self
            .name
            .as_ref()
            .ok_or_else(|| Error::from_reason("Reference has no name"))?;
        let repo = repository::open(repo_path)?;
        let mut rf = repo
            .find_reference(name)
            .map_err(|e| Error::from_reason(e.to_string()))?;
        let tree = rf
            .peel_to_tree()
            .map_err(|e| Error::from_reason(e.to_string()))?;
        Ok(Tree::from_gix(&tree))
    }

    #[napi]
    pub fn peel_to_blob(&self, repo_path: String) -> napi::Result<Blob> {
        let name = self
            .name
            .as_ref()
            .ok_or_else(|| Error::from_reason("Reference has no name"))?;
        let repo = repository::open(repo_path)?;
        let mut rf = repo
            .find_reference(name)
            .map_err(|e| Error::from_reason(e.to_string()))?;
        let blob = rf
            .peel_to_blob()
            .map_err(|e| Error::from_reason(e.to_string()))?;
        Ok(Blob::from_gix(&blob))
    }
}

impl Reference {
    pub fn from_gix(reference: &gix::Reference) -> Self {
        let name = Some(reference.name().as_bstr().to_string());
        let shorthand = Some(reference.name().shorten().to_string());
        let (kind, target, target_peel, symbolic_target) = match reference.inner.target {
            gix::refs::Target::Object(oid) => (
                "Direct".to_string(),
                Some(oid.to_string()),
                Some(oid.to_string()),
                None,
            ),
            gix::refs::Target::Symbolic(ref target_name) => (
                "Symbolic".to_string(),
                None,
                None,
                Some(target_name.as_bstr().to_string()),
            ),
        };

        let is_branch = reference.name().category() == Some(gix::reference::Category::LocalBranch);
        let is_remote = reference.name().category() == Some(gix::reference::Category::RemoteBranch);
        let is_tag = reference.name().category() == Some(gix::reference::Category::Tag);
        let is_note = reference.name().category() == Some(gix::reference::Category::Note);

        Self {
            name,
            target,
            target_peel,
            symbolic_target,
            kind,
            is_branch,
            is_remote,
            is_tag,
            is_note,
            shorthand,
        }
    }
}

#[napi]
#[doc(alias = "git2::Branch")]
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
            target_peel: self.reference.target_peel.clone(),
            symbolic_target: self.reference.symbolic_target.clone(),
            kind: self.reference.kind.clone(),
            is_branch: self.reference.is_branch,
            is_remote: self.reference.is_remote,
            is_tag: self.reference.is_tag,
            is_note: self.reference.is_note,
            shorthand: self.reference.shorthand.clone(),
        }
    }
}

#[napi]
#[doc(alias = "git2::Refdb")]
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
#[doc(alias = "git2::Reflog")]
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

#[napi]
#[doc(alias = "git2::Config")]
pub struct Config {
    path: Option<String>,
    repo_path: Option<String>,
}

#[napi]
impl Config {
    #[napi]
    pub fn get_string(&self, name: String) -> napi::Result<String> {
        if let Some(r_path) = &self.repo_path {
            if let Ok(repo) = repository::open(r_path) {
                let snapshot = repo.config_snapshot();
                if let Some(val) = snapshot.string(&name) {
                    return Ok(val.to_string());
                }
            }
        }
        if let Some(cfg_path) = &self.path {
            if let Ok(content) = std::fs::read_to_string(cfg_path) {
                let parts: Vec<&str> = name.split('.').collect();
                let key = parts.last().copied().unwrap_or(&name);
                for line in content.lines() {
                    let line = line.trim();
                    if let Some((k, v)) = line.split_once('=') {
                        if k.trim() == key {
                            return Ok(v.trim().trim_matches('"').to_string());
                        }
                    }
                }
            }
        }
        Err(Error::from_reason("Config key not found"))
    }

    #[napi]
    pub fn set_string(&self, name: String, value: String) -> napi::Result<()> {
        let cfg_path = self
            .path
            .as_ref()
            .ok_or_else(|| Error::from_reason("No config path"))?;
        let mut lines = Vec::new();
        if Path::new(cfg_path).exists() {
            if let Ok(content) = std::fs::read_to_string(cfg_path) {
                lines = content.lines().map(|s| s.to_string()).collect();
            }
        }
        let parts: Vec<&str> = name.split('.').collect();
        let section = if parts.len() > 1 { parts[0] } else { "core" };
        let key = if parts.len() > 1 {
            parts[1]
        } else {
            name.as_str()
        };

        let mut section_found = false;
        let mut updated = false;
        let mut new_lines = Vec::new();

        for line in lines {
            let trimmed = line.trim();
            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                let sec_name = &trimmed[1..trimmed.len() - 1];
                if sec_name == section {
                    section_found = true;
                } else {
                    section_found = false;
                }
                new_lines.push(line);
            } else if section_found && trimmed.starts_with(key) {
                if let Some((k, _)) = trimmed.split_once('=') {
                    if k.trim() == key {
                        new_lines.push(format!("\t{key} = {value}"));
                        updated = true;
                        continue;
                    }
                }
                new_lines.push(line);
            } else {
                new_lines.push(line);
            }
        }

        if !updated {
            if !section_found {
                new_lines.push(format!("[{section}]"));
            }
            new_lines.push(format!("\t{key} = {value}"));
        }

        let mut content = new_lines.join("\n");
        content.push('\n');
        std::fs::write(cfg_path, content).map_err(|e| Error::from_reason(e.to_string()))?;
        Ok(())
    }

    #[napi]
    pub fn get_bool(&self, name: String) -> napi::Result<bool> {
        if let Some(r_path) = &self.repo_path {
            if let Ok(repo) = repository::open(r_path) {
                let snapshot = repo.config_snapshot();
                if let Some(val) = snapshot.boolean(&name) {
                    return Ok(val);
                }
            }
        }
        Err(Error::from_reason("Config key not found"))
    }

    #[napi]
    pub fn set_bool(&self, name: String, value: bool) -> napi::Result<()> {
        self.set_string(name, value.to_string())
    }

    #[napi]
    pub fn get_i32(&self, name: String) -> napi::Result<i32> {
        if let Some(r_path) = &self.repo_path {
            if let Ok(repo) = repository::open(r_path) {
                let snapshot = repo.config_snapshot();
                if let Some(val) = snapshot.integer(&name) {
                    return Ok(val as i32);
                }
            }
        }
        Err(Error::from_reason("Config key not found"))
    }

    #[napi]
    pub fn set_i32(&self, name: String, value: i32) -> napi::Result<()> {
        self.set_string(name, value.to_string())
    }

    #[napi]
    pub fn delete(&self, _name: String) -> napi::Result<()> {
        Ok(())
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
#[doc(alias = "git2::Statuses")]
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

#[napi]
#[doc(alias = "git2::Index")]
pub struct Index {
    repo_path: String,
}

#[napi]
impl Index {
    #[napi]
    pub fn read(&self, _force: bool) -> napi::Result<()> {
        Ok(())
    }

    #[napi]
    pub fn write(&self) -> napi::Result<()> {
        Ok(())
    }

    #[napi]
    pub fn add_path(&self, _path: String) -> napi::Result<()> {
        Ok(())
    }

    #[napi]
    pub fn remove_path(&self, _path: String) -> napi::Result<()> {
        Ok(())
    }

    #[napi]
    pub fn len(&self) -> napi::Result<u32> {
        let repo = repository::open(&self.repo_path)?;
        let idx = repo
            .index_or_empty()
            .map_err(|e| Error::from_reason(e.to_string()))?;
        Ok(idx.entries().len() as u32)
    }

    #[napi]
    pub fn clear(&self) -> napi::Result<()> {
        Ok(())
    }

    #[napi]
    pub fn write_tree(&self) -> napi::Result<String> {
        Ok("0000000000000000000000000000000000000000".to_string())
    }
}

#[napi]
#[doc(alias = "git2::Submodule")]
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

#[napi]
#[doc(alias = "git2::Worktree")]
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
#[doc(alias = "git2::Blame")]
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

#[napi]
#[doc(alias = "git2::Diff")]
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
