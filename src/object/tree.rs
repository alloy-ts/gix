use napi_derive::napi;

#[napi]
pub struct TreeEntry {
    id: String,
    name: Option<String>,
    filemode: i32,
}

#[napi]
impl TreeEntry {
    #[napi]
    pub fn id(&self) -> String {
        self.id.clone()
    }

    #[napi]
    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

    #[napi]
    pub fn filemode(&self) -> i32 {
        self.filemode
    }
}

#[napi]
#[doc(alias = "git2::Tree")]
pub struct Tree {
    id: String,
    entries: Vec<TreeEntry>,
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
    pub fn entry_by_index(&self, index: u32) -> Option<TreeEntry> {
        self.entries.get(index as usize).map(|e| TreeEntry {
            id: e.id.clone(),
            name: e.name.clone(),
            filemode: e.filemode,
        })
    }

    #[napi]
    pub fn entry_by_name(&self, name: String) -> Option<TreeEntry> {
        self.entries
            .iter()
            .find(|e| e.name.as_deref() == Some(&name))
            .map(|e| TreeEntry {
                id: e.id.clone(),
                name: e.name.clone(),
                filemode: e.filemode,
            })
    }
}

impl Tree {
    pub fn from_gix(tree: &gix::Tree) -> Self {
        let id = tree.id.to_string();
        let mut entries = Vec::new();
        if let Ok(tree_ref) = tree.decode() {
            for entry in tree_ref.entries {
                let filemode = match entry.mode.kind() {
                    gix::objs::tree::EntryKind::Blob => 0o100644,
                    gix::objs::tree::EntryKind::BlobExecutable => 0o100755,
                    gix::objs::tree::EntryKind::Link => 0o120000,
                    gix::objs::tree::EntryKind::Tree => 0o040000,
                    gix::objs::tree::EntryKind::Commit => 0o160000,
                };
                entries.push(TreeEntry {
                    id: entry.oid.to_string(),
                    name: Some(entry.filename.to_string()),
                    filemode,
                });
            }
        }
        Self { id, entries }
    }
}
