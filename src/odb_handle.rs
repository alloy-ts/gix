use napi_derive::napi;
use std::path::PathBuf;

#[napi]
#[doc(alias = "gix::OdbHandle")]
pub struct OdbHandle {
    pub(crate) repo_path: PathBuf,
}

#[napi]
impl OdbHandle {
    pub fn new(repo_path: PathBuf) -> Self {
        Self { repo_path }
    }

    #[napi]
    pub fn num_objects_in_memory(&self) -> u32 {
        0
    }

    #[napi]
    pub fn enable_object_memory(&mut self) {}

    #[napi]
    pub fn reset_object_memory(&mut self) {}

    #[napi]
    pub fn has_object(&self, oid_hex: String) -> bool {
        if let Ok(repo) = crate::repository::open(&self.repo_path) {
            if let Ok(oid) = gix::ObjectId::from_hex(oid_hex.as_bytes()) {
                return repo.has_object(oid);
            }
        }
        false
    }
}
