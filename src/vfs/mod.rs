use std::io::Error;

pub mod naive_vfs;

/// Generic file, to be used to store information
/// and data of a file, so it can be used later.
pub struct GenericFile {
    pub name: String,
    pub extension: String,

    pub data: Vec<u8>,
}

pub trait VirtualFileSystem {
    fn read_file(&self, path: &str) -> Result<GenericFile, Error>;
    fn write_file(&self, path: &str, data: GenericFile) -> Result<(), Error>;

    fn read_bytes(&self, path: &str, bytes: &mut Vec<u8>) -> Result<(), Error>;
    fn write_bytes(&self, path: &str, data: &[u8]) -> Result<(), Error>;
}
