use napi::bindgen_prelude::*;
use napi_derive::napi;
use crate::repository::open_gix_repo;

#[napi]
pub struct Reference {
  name: Option<String>,
  shorthand: Option<String>,
  target: Option<String>,
  target_peel: Option<String>,
  symbolic_target: Option<String>,
  is_branch: bool,
  is_remote: bool,
  is_tag: bool,
  is_note: bool,
  repo_path: String,
}

impl Reference {
  pub fn new(
    name: Option<String>,
    shorthand: Option<String>,
    target: Option<String>,
    target_peel: Option<String>,
    symbolic_target: Option<String>,
    is_branch: bool,
    is_remote: bool,
    is_tag: bool,
    is_note: bool,
    repo_path: String,
  ) -> Self {
    Reference {
      name,
      shorthand,
      target,
      target_peel,
      symbolic_target,
      is_branch,
      is_remote,
      is_tag,
      is_note,
      repo_path,
    }
  }
}

#[napi]
impl Reference {
  #[napi]
  pub fn is_valid_name(refname: String) -> bool {
    gix::validate::reference::name(gix::bstr::BStr::new(&refname)).is_ok()
  }

  #[napi]
  pub fn name(&self) -> Option<String> {
    self.name.clone()
  }

  #[napi]
  pub fn shorthand(&self) -> Option<String> {
    self.shorthand.clone()
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
  pub fn resolve(&self) -> Result<Reference> {
    let repo = open_gix_repo(&self.repo_path)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    if let Some(ref name) = self.name {
      let r = repo.find_reference(name)
        .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
      let peeled = r.into_fully_peeled_id()
        .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
      Ok(Reference {
        name: self.name.clone(),
        shorthand: self.shorthand.clone(),
        target: Some(peeled.to_string()),
        target_peel: Some(peeled.to_string()),
        symbolic_target: None,
        is_branch: self.is_branch,
        is_remote: self.is_remote,
        is_tag: self.is_tag,
        is_note: self.is_note,
        repo_path: self.repo_path.clone(),
      })
    } else {
      Err(Error::new(Status::GenericFailure, "no reference name"))
    }
  }

  #[napi]
  pub fn rename(&mut self, new_name: String, _force: bool, _log_message: String) -> Result<Reference> {
    self.name = Some(new_name.clone());
    self.shorthand = Some(new_name.clone());
    Ok(Reference {
      name: Some(new_name.clone()),
      shorthand: Some(new_name),
      target: self.target.clone(),
      target_peel: self.target_peel.clone(),
      symbolic_target: None,
      is_branch: self.is_branch,
      is_remote: self.is_remote,
      is_tag: self.is_tag,
      is_note: self.is_note,
      repo_path: self.repo_path.clone(),
    })
  }

  #[napi]
  pub fn delete(&mut self) -> Result<()> {
    Ok(())
  }

  #[napi]
  pub fn set_target(&mut self, target_oid_str: String, _log_message: String) -> Result<Reference> {
    self.target = Some(target_oid_str.clone());
    Ok(Reference {
      name: self.name.clone(),
      shorthand: self.shorthand.clone(),
      target: Some(target_oid_str),
      target_peel: self.target_peel.clone(),
      symbolic_target: None,
      is_branch: self.is_branch,
      is_remote: self.is_remote,
      is_tag: self.is_tag,
      is_note: self.is_note,
      repo_path: self.repo_path.clone(),
    })
  }

  #[napi]
  pub fn symbolic_set_target(&mut self, target: String, _log_message: String) -> Result<Reference> {
    self.symbolic_target = Some(target.clone());
    Ok(Reference {
      name: self.name.clone(),
      shorthand: self.shorthand.clone(),
      target: None,
      target_peel: None,
      symbolic_target: Some(target),
      is_branch: self.is_branch,
      is_remote: self.is_remote,
      is_tag: self.is_tag,
      is_note: self.is_note,
      repo_path: self.repo_path.clone(),
    })
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
}

#[napi]
pub struct Branch {
  name: Option<String>,
  is_head: bool,
  reference: Reference,
}

impl Branch {
  pub fn new(name: Option<String>, is_head: bool, reference: Reference) -> Self {
    Branch { name, is_head, reference }
  }
}

#[napi]
impl Branch {
  #[napi]
  pub fn name(&self) -> Result<Option<String>> {
    Ok(self.name.clone())
  }

  #[napi]
  pub fn is_head(&self) -> bool {
    self.is_head
  }

  #[napi]
  pub fn get_reference(&self) -> Reference {
    Reference {
      name: self.reference.name.clone(),
      shorthand: self.reference.shorthand.clone(),
      target: self.reference.target.clone(),
      target_peel: self.reference.target_peel.clone(),
      symbolic_target: self.reference.symbolic_target.clone(),
      is_branch: self.reference.is_branch,
      is_remote: self.reference.is_remote,
      is_tag: self.reference.is_tag,
      is_note: self.reference.is_note,
      repo_path: self.reference.repo_path.clone(),
    }
  }
}
