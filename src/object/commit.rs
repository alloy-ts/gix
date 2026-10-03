use crate::{Signature, parse_time_str};
use napi_derive::napi;

#[napi]
#[doc(alias = "git2::Commit")]
pub struct Commit {
    id: String,
    message: Option<String>,
    summary: Option<String>,
    author_name: String,
    author_email: String,
    author_time: i64,
    committer_name: String,
    committer_email: String,
    committer_time: i64,
    parent_ids: Vec<String>,
    tree_id: String,
}

#[napi]
impl Commit {
    #[napi]
    pub fn id(&self) -> String {
        self.id.clone()
    }

    #[napi]
    pub fn message(&self) -> Option<String> {
        self.message.clone()
    }

    #[napi]
    pub fn summary(&self) -> Option<String> {
        self.summary.clone()
    }

    #[napi]
    pub fn author(&self) -> Signature {
        Signature::new(
            self.author_name.clone(),
            self.author_email.clone(),
            self.author_time,
        )
    }

    #[napi]
    pub fn committer(&self) -> Signature {
        Signature::new(
            self.committer_name.clone(),
            self.committer_email.clone(),
            self.committer_time,
        )
    }

    #[napi]
    pub fn parent_count(&self) -> u32 {
        self.parent_ids.len() as u32
    }

    #[napi]
    pub fn parent_ids(&self) -> Vec<String> {
        self.parent_ids.clone()
    }

    #[napi]
    pub fn tree_id(&self) -> String {
        self.tree_id.clone()
    }
}

impl Commit {
    pub fn from_gix(commit: &gix::Commit) -> Self {
        let id = commit.id.to_string();
        let message = commit.message_raw().ok().map(|s| s.to_string());

        let (author_name, author_email, author_time) = if let Ok(author) = commit.author() {
            (
                author.name.to_string(),
                author.email.to_string(),
                parse_time_str(author.time),
            )
        } else {
            ("".to_string(), "".to_string(), 0)
        };

        let (committer_name, committer_email, committer_time) =
            if let Ok(committer) = commit.committer() {
                (
                    committer.name.to_string(),
                    committer.email.to_string(),
                    parse_time_str(committer.time),
                )
            } else {
                ("".to_string(), "".to_string(), 0)
            };

        let parent_ids = commit.parent_ids().map(|oid| oid.to_string()).collect();
        let tree_id = commit
            .tree_id()
            .ok()
            .map(|id| id.to_string())
            .unwrap_or_default();

        Self {
            id,
            message: message.clone(),
            summary: message,
            author_name,
            author_email,
            author_time,
            committer_name,
            committer_email,
            committer_time,
            parent_ids,
            tree_id,
        }
    }
}
