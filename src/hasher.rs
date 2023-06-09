//! Provide a CRC-32C implementor of [std::hash::Hasher].
use std::hash::Hasher;

use crate::crc32c_append;

/// Implementor of [std::hash::Hasher] for CRC-32C.
///
/// Note that CRC-32C produces a 32-bit hash (as [u32]),
/// but the trait requires that the value be return as [u64].
pub struct Crc32cHasher {
    checksum: u32,
}

impl Crc32cHasher {
    /// Create the hasher pre-loaded with a particular checksum.
    ///
    /// Use the [Default::default()] constructor for a clean start.
    pub fn new(initial: u32) -> Self {
        Self { checksum: initial }
    }
}

impl Default for Crc32cHasher {
    fn default() -> Self {
        Self { checksum: 0 }
    }
}

impl Hasher for Crc32cHasher {
    fn finish(&self) -> u64 {
        self.checksum as u64
    }

    fn write(&mut self, bytes: &[u8]) {
        self.checksum = crc32c_append(self.checksum, bytes);
    }
}
