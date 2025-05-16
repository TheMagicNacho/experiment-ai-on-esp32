use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::ops::Deref;
use stable_deref_trait::StableDeref;


pub struct Mmap {
    data: Vec<u8>,
}

impl Mmap {
    pub unsafe fn map(_file: &File) -> io::Result<Self> {
        Ok(Self { data: Vec::new() })
    }
    
    pub fn open<P: AsRef<Path>>(_path: P) -> io::Result<Self> {
        Ok(Self { data: Vec::new() })
    }
}

pub struct MmapOptions {}

impl MmapOptions {
    pub fn new() -> Self {
        Self {}
    }

    pub fn map(self, _file: &File) -> io::Result<Mmap> {
        Ok(Mmap { data: Vec::new() })
    }
}

impl AsRef<[u8]> for Mmap {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

impl Deref for Mmap {
    type Target = [u8];
    
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

unsafe impl StableDeref for Mmap {}

pub mod advice {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Advice {
        Normal,
        Random,
        Sequential,
        WillNeed,
        DontNeed,
    }
}