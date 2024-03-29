//! Provide a CRC-32C implementor of [Hasher].
use std::hash::Hasher;

use crate::crc32c_append;

/// Implementor of [Hasher] for CRC-32C.
///
/// Note that CRC-32C produces a 32-bit hash (as [u32]),
/// but the trait requires that the output value be [u64].
#[derive(Default)]
pub struct Crc32cHasher {
    checksum: u32,
}

impl Crc32cHasher {
    /// Create the [Hasher] pre-loaded with a particular checksum.
    ///
    /// Use the [Default::default()] constructor for a clean start.
    pub fn new(initial: u32) -> Self {
        Self { checksum: initial }
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

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STRING: &[u8] =
        b"This is a very long string which is used to test the CRC-32-Castagnoli function.";
    const CHECKSUM: u32 = 0x20_CB_1E_59;

    #[test]
    fn can_hash() {
        let mut hasher = Crc32cHasher::default();
        hasher.write(TEST_STRING);
        assert_eq!(hasher.finish(), CHECKSUM as u64);
    }
}
