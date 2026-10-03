use crate::object::{Blob, Commit, Tag, Tree};
use napi::Error;
use napi_derive::napi;

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
        let repo = crate::repository::open(repo_path)?;
        let rf = repo
            .find_reference(name)
            .map_err(|e| Error::from_reason(e.to_string()))?;
        Ok(Reference::from_gix(&rf))
    }

    #[napi]
    pub fn set_target_id(
        &self,
        repo_path: String,
        oid_hex: String,
        log_message: String,
    ) -> napi::Result<Reference> {
        let name = self
            .name
            .as_ref()
            .ok_or_else(|| Error::from_reason("Reference has no name"))?;
        let repo = crate::repository::open(repo_path)?;
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
        let repo = crate::repository::open(repo_path)?;
        let rf = repo
            .find_reference(name)
            .map_err(|e| Error::from_reason(e.to_string()))?;
        rf.delete().map_err(|e| Error::from_reason(e.to_string()))
    }

    #[napi]
    pub fn log_exists(&self, repo_path: String) -> napi::Result<bool> {
        let name = self
            .name
            .as_ref()
            .ok_or_else(|| Error::from_reason("Reference has no name"))?;
        let repo = crate::repository::open(repo_path)?;
        let rf = repo
            .find_reference(name)
            .map_err(|e| Error::from_reason(e.to_string()))?;
        Ok(rf.log_exists())
    }

    #[napi]
    pub fn peel_to_commit(&self, repo_path: String) -> napi::Result<Commit> {
        let name = self
            .name
            .as_ref()
            .ok_or_else(|| Error::from_reason("Reference has no name"))?;
        let repo = crate::repository::open(repo_path)?;
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
        let repo = crate::repository::open(repo_path)?;
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
        let repo = crate::repository::open(repo_path)?;
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
        let repo = crate::repository::open(repo_path)?;
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
