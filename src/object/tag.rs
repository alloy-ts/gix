use napi_derive::napi;

#[napi]
pub struct Tag {
  id: String,
  name: Option<String>,
  message: Option<String>,
  target_id: String,
}

impl Tag {
  pub fn new(id: String, name: Option<String>, message: Option<String>, target_id: String) -> Self {
    Tag { id, name, message, target_id }
  }
}

#[napi]
impl Tag {
  #[napi]
  pub fn id(&self) -> String {
    self.id.clone()
  }

  #[napi]
  pub fn name(&self) -> Option<String> {
    self.name.clone()
  }

  #[napi]
  pub fn message(&self) -> Option<String> {
    self.message.clone()
  }

  #[napi]
  pub fn target_id(&self) -> String {
    self.target_id.clone()
  }
}
