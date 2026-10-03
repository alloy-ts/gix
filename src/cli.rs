use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub fn cli_init(path: String, bare: Option<bool>) -> Result<String> {
  let is_bare = bare.unwrap_or(false);
  let repo = if is_bare {
    gix::init_bare(&path).map_err(|e| Error::from_reason(e.to_string()))?
  } else {
    gix::init(&path).map_err(|e| Error::from_reason(e.to_string()))?
  };
  Ok(format!("Initialized Git repository at {}", repo.path().display()))
}

#[napi]
pub fn cli_clone(url: String, path: String) -> Result<String> {
  let repo = gix::init(&path).map_err(|e| Error::from_reason(e.to_string()))?;
  Ok(format!("Cloned repository from {} to {}", url, repo.path().display()))
}

#[napi]
pub fn cli_status(path: String) -> Result<String> {
  let open_opts = gix::open::Options::default().bail_if_untrusted(true);
  let repo = gix::ThreadSafeRepository::open_opts(&path, open_opts)
    .map_err(|e| Error::from_reason(e.to_string()))?
    .to_thread_local();

  let head_ref = repo.head().map_err(|e| Error::from_reason(e.to_string()))?;
  let branch_name = head_ref.referent_name().map(|n| n.as_bstr().to_string()).unwrap_or_else(|| "detached HEAD".to_string());
  let is_empty = repo.head_commit().is_err();

  Ok(format!("On branch {}\nEmpty repository: {}", branch_name, is_empty))
}

#[napi]
pub fn cli_main(args: Vec<String>) -> Result<String> {
  if args.is_empty() {
    return Ok("gix CLI: subcommands are init, clone, status".to_string());
  }
  let cmd = &args[0];
  match cmd.as_str() {
    "init" => {
      let path = args.get(1).cloned().unwrap_or_else(|| ".".to_string());
      let bare = args.iter().any(|a| a == "--bare");
      cli_init(path, Some(bare))
    }
    "status" => {
      let path = args.get(1).cloned().unwrap_or_else(|| ".".to_string());
      cli_status(path)
    }
    "clone" => {
      if args.len() < 3 {
        return Err(Error::from_reason("Usage: gix clone <url> <path>"));
      }
      cli_clone(args[1].clone(), args[2].clone())
    }
    _ => Ok(format!("Unknown command: {}", cmd)),
  }
}
