use napi::bindgen_prelude::*;
use napi_derive::napi;

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
  pub fn open(path: String) -> Result<Repository> {
    let repo = git2::Repository::open(&path)
      .map_err(|e| Error::new(Status::GenericFailure, format!("failed to open: {e}")))?;
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
}
