use super::GitObject;

pub struct FileObject {}

impl GitObject for FileObject {
    fn from_bytes(_bytes: &[u8]) -> anyhow::Result<Self> {
        Ok(Self {})
    }
}
