use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
#[doc(alias = "git2::Blob")]
pub struct Blob {
    id: String,
    size: u32,
    content: Vec<u8>,
    is_binary: bool,
}

#[napi]
impl Blob {
    #[napi]
    pub fn id(&self) -> String {
        self.id.clone()
    }

    #[napi]
    pub fn size(&self) -> u32 {
        self.size
    }

    #[napi]
    pub fn content(&self) -> Buffer {
        Buffer::from(self.content.clone())
    }

    #[napi]
    pub fn is_binary(&self) -> bool {
        self.is_binary
    }
}

impl Blob {
    pub fn from_gix(blob: &gix::Blob) -> Self {
        let id = blob.id.to_string();
        let content = blob.data.clone();
        let size = content.len() as u32;
        let is_binary = content.contains(&0);
        Self {
            id,
            size,
            content,
            is_binary,
        }
    }
}
