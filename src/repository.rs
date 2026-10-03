use std::path::Path;

/// Open a repository with bail_if_untrusted set to true for git2 compatibility.
pub fn open(path: impl AsRef<Path>) -> Result<gix::Repository, napi::Error> {
    let mut options = gix::open::Options::default();
    options = options.bail_if_untrusted(true);
    gix::open_opts(path.as_ref().to_path_buf(), options)
        .map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Initialize a new repository at path.
pub fn init(path: impl AsRef<Path>) -> Result<gix::Repository, napi::Error> {
    gix::init(path.as_ref().to_path_buf()).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Open a bare repository at path.
pub fn open_bare(path: impl AsRef<Path>) -> Result<gix::Repository, napi::Error> {
    let repo = open(path)?;
    if !repo.is_bare() {
        return Err(napi::Error::from_reason("Repository is not bare"));
    }
    Ok(repo)
}

/// Discover an existing repository starting at path.
pub fn discover(path: impl AsRef<Path>) -> Result<gix::Repository, napi::Error> {
    gix::discover(path.as_ref().to_path_buf()).map_err(|e| napi::Error::from_reason(e.to_string()))
}
