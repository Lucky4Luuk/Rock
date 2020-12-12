use std::io::{Error, ErrorKind};

use super::VirtualFileSystem;

/// A very basic VFS that does no compression
/// and stores files separately on the disk.
/// It serializes everything to .bin files using `bincode`.
pub struct BinVFS;

impl VirtualFileSystem for BinVFS {
    fn read_file<P>() -> std::result::Result<P, Error> {
        Err(Error::new(ErrorKind::Other, "VFS did not return!"))
    }

    fn write_file<P>() -> std::result::Result<(), Error> {
        unimplemented!();
    }
}
