use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub struct Blob {
  id: String,
  data: Vec<u8>,
}

impl Blob {
  pub fn new(id: String, data: Vec<u8>) -> Self {
    Blob { id, data }
  }
}

#[napi]
impl Blob {
  #[napi]
  pub fn id(&self) -> String {
    self.id.clone()
  }

  #[napi]
  pub fn content(&self) -> Buffer {
    Buffer::from(self.data.clone())
  }

  #[napi]
  pub fn is_binary(&self) -> bool {
    self.data.contains(&0)
  }

  #[napi]
  pub fn size(&self) -> u32 {
    self.data.len() as u32
  }
}
