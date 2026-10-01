use napi::Error;
use napi_derive::napi;

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
}
