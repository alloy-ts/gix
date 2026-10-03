use napi_derive::napi;

#[napi]
pub struct TreeEntry {
  id: String,
  name: Option<String>,
  filemode: i32,
}

impl TreeEntry {
  pub fn new(id: String, name: Option<String>, filemode: i32) -> Self {
    TreeEntry { id, name, filemode }
  }
}

#[napi]
impl TreeEntry {
  #[napi(getter)]
  pub fn id(&self) -> String {
    self.id.clone()
  }

  #[napi(getter)]
  pub fn name(&self) -> Option<String> {
    self.name.clone()
  }

  #[napi(getter)]
  pub fn filemode(&self) -> i32 {
    self.filemode
  }
}

#[napi]
pub struct Tree {
  id: String,
  entries: Vec<TreeEntry>,
}

impl Tree {
  pub fn new(id: String, entries: Vec<TreeEntry>) -> Self {
    Tree { id, entries }
  }
}

#[napi]
impl Tree {
  #[napi]
  pub fn id(&self) -> String {
    self.id.clone()
  }

  #[napi]
  pub fn len(&self) -> u32 {
    self.entries.len() as u32
  }

  #[napi]
  pub fn is_empty(&self) -> bool {
    self.entries.is_empty()
  }

  #[napi]
  pub fn get_name(&self, filename: String) -> Option<TreeEntry> {
    self.entries.iter().find(|e| e.name.as_deref() == Some(&filename)).map(|e| TreeEntry {
      id: e.id.clone(),
      name: e.name.clone(),
      filemode: e.filemode,
    })
  }
}
