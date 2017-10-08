//! Implements the crc32c algorithm.

#[macro_use]
extern crate lazy_static;

mod util;
mod sw;

/// Computes the crc32c for the data payload.
#[inline]
pub fn crc32c(data: &[u8]) -> u32 {
    crc32c_append(0, data)
}

/// Computes the crc32c for the data payload, starting with a previous crc32c value.
#[inline]
pub fn crc32c_append(crc: u32, data: &[u8]) -> u32 {
    sw::crc32c(crc, data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc() {
        let v = crc32c(b"012345678910");
        assert_eq!(0x8412E281, v);
    }

    #[test]
    fn crc_append() {
        let v = crc32c(b"01234");
        let v = crc32c_append(v, b"5678910");
        assert_eq!(0x8412E281, v);
    }

    #[test]
    fn very_small() {
        let v = crc32c(b"1");
        assert_eq!(0x90F599E3, v);
    }

    #[test]
    fn long_string() {
        let v = crc32c(
            b"This is a very long string which is used to test the CRC-32-Castagnoli function.",
        );
        assert_eq!(0x20CB1E59, v);
    }
}
