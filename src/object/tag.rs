use crate::{Signature, parse_time_str};
use napi_derive::napi;

#[napi]
#[doc(alias = "git2::Tag")]
pub struct Tag {
    id: String,
    name: Option<String>,
    target_id: String,
    message: Option<String>,
    tagger: Option<Signature>,
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
    pub fn target_id(&self) -> String {
        self.target_id.clone()
    }

    #[napi]
    pub fn message(&self) -> Option<String> {
        self.message.clone()
    }

    #[napi]
    pub fn tagger(&self) -> Option<Signature> {
        self.tagger
            .as_ref()
            .map(|t| Signature::new(t.name(), t.email(), t.time_seconds()))
    }
}

impl Tag {
    pub fn from_gix(tag: &gix::Tag) -> Self {
        let id = tag.id.to_string();
        if let Ok(decoded) = tag.decode() {
            let name = Some(decoded.name.to_string());
            let target_id = decoded.target.to_string();
            let message = Some(decoded.message.to_string());
            let tagger = decoded.tagger().ok().flatten().map(|t| {
                let name = t.name.to_string();
                let email = t.email.to_string();
                let time_seconds = parse_time_str(t.time);
                Signature::new(name, email, time_seconds)
            });

            Self {
                id,
                name,
                target_id,
                message,
                tagger,
            }
        } else {
            Self {
                id,
                name: None,
                target_id: "".to_string(),
                message: None,
                tagger: None,
            }
        }
    }
}
