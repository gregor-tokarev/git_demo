use anyhow::Result;

pub mod file_object;
pub mod tree;
pub mod tree_row;

pub trait GitObject {
    fn from_bytes(bytes: &[u8]) -> Result<Self>
    where
        Self: Sized;
}
