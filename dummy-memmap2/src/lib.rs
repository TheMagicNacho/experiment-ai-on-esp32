// This is a minimal dummy implementation of memmap2
// The real functionality is provided by our custom mmap module in the main crate

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::ops::Deref;
use stable_deref_trait::StableDeref;

// Forward declarations of the Mmap struct and its methods
pub struct Mmap {
    data: Vec<u8>,
}

impl Mmap {
    // Just enough to satisfy the compiler - the real implementation is in our mmap.rs
    pub unsafe fn map(_file: &File) -> io::Result<Self> {
        Ok(Self { data: Vec::new() })
    }
    
    pub fn open<P: AsRef<Path>>(_path: P) -> io::Result<Self> {
        Ok(Self { data: Vec::new() })
    }
}

// Add MmapOptions struct required by ug crate
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

// Implement Deref and StableDeref for Mmap
impl Deref for Mmap {
    type Target = [u8];
    
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

// Safe to implement as we're using a Vec which has stable addresses
unsafe impl StableDeref for Mmap {}

// These parts won't be used but need to exist for the compiler
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