use crate::Reference;
use napi::Error;
use napi_derive::napi;
use std::path::PathBuf;

#[napi]
#[doc(alias = "gix::RefStore")]
pub struct RefStore {
    pub(crate) repo_path: PathBuf,
}

#[napi]
impl RefStore {
    pub fn new(repo_path: PathBuf) -> Self {
        Self { repo_path }
    }

    #[napi]
    pub fn git_dir(&self) -> String {
        self.repo_path.to_string_lossy().into_owned()
    }

    #[napi]
    pub fn common_dir(&self) -> String {
        self.repo_path.to_string_lossy().into_owned()
    }

    #[napi]
    pub fn packed_refs_path(&self) -> String {
        self.repo_path
            .join("packed-refs")
            .to_string_lossy()
            .into_owned()
    }

    #[napi]
    pub fn reflog_exists(&self, name: String) -> bool {
        let reflog_path = self.repo_path.join("logs").join(&name);
        reflog_path.exists()
    }

    #[napi]
    pub fn is_pristine(&self) -> bool {
        let refs_dir = self.repo_path.join("refs");
        !refs_dir.exists()
            || std::fs::read_dir(refs_dir)
                .map(|mut d| d.next().is_none())
                .unwrap_or(true)
    }

    #[napi]
    pub fn find(&self, partial: String) -> napi::Result<Reference> {
        let repo = crate::repository::open(&self.repo_path)?;
        let rf = repo
            .find_reference(&partial)
            .map_err(|e| Error::from_reason(e.to_string()))?;
        Ok(Reference::from_gix(&rf))
    }
}
