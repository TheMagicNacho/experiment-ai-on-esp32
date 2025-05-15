// This is a simplified implementation of the memmap2 crate
// for ESP32 targets that don't support memory mapping.

use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct Mmap {
    data: Vec<u8>,
}

impl Mmap {
    /// Create a memory map backed by a file
    pub unsafe fn map(file: &File) -> std::io::Result<Self> {
        let mut data = Vec::new();
        let mut file = file.try_clone()?;
        file.read_to_end(&mut data)?;
        Ok(Self { data })
    }
    
    /// Open a file and map it into memory
    pub fn open<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let file = File::open(path)?;
        unsafe { Self::map(&file) }
    }
}

impl AsRef<[u8]> for Mmap {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

pub use std::io::Error;