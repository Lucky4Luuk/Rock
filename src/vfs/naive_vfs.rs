use std::io::{Error, ErrorKind};
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use super::{VirtualFileSystem, GenericFile};

/// A very basic VFS that does no compression
/// and stores files directly on the disk.
pub struct NaiveVFS;

impl NaiveVFS {
    pub fn new() -> Self {
        Self {}
    }
}

impl VirtualFileSystem for NaiveVFS {
    fn read_file(&self, path: &str) -> std::result::Result<GenericFile, Error> {
        let mut bytes = Vec::new();
        self.read_bytes(path, &mut bytes)?;
        Ok(GenericFile {
            name: "Unknown".to_owned(),
            extension: "Unknown".to_owned(),
            data: bytes,
        })
    }

    fn write_file(&self, path: &str, data: GenericFile) -> std::result::Result<(), Error> {
        unimplemented!();
    }

    fn read_bytes(&self, path: &str, bytes: &mut Vec<u8>) -> Result<(), Error> {
        let p = Path::new(path);
        let mut file = File::open(&p)?;
        file.read_to_end(bytes)?;
        Ok(())
    }

    fn write_bytes(&self, path: &str, data: &[u8]) -> Result<(), Error> {
        unimplemented!();
    }
}
