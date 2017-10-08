//! Implements crc32c without hardware support.

use util;

mod table;
use self::table::crc_at;

/// Software implementation of the algorithm.
pub fn crc32c(crci: u32, buffer: &[u8]) -> u32 {
    const MASK: u32 = 0xFFFF_FFFF;

    let mut crc = (crci ^ MASK) as u64;

    let (start, mid, end) = util::split(buffer);

    crc = crc_unaligned(crc, start);

    crc = crc_aligned(crc, mid);

    crc = crc_unaligned(crc, end);

    (crc as u32) ^ MASK
}

fn crc_unaligned(crci: u64, buffer: &[u8]) -> u64 {
    buffer.iter().fold(crci, |crc, &next| {
        let index = (crc ^ (next as u64)) as u8;
        crc_at(0, index) ^ (crc >> 8)
    })
}

fn crc_aligned(crci: u64, buffer: &[u64]) -> u64 {
    buffer.iter().fold(crci, |mut crc, &next| {
        crc ^= next;

        crc_at(7, crc as u8) ^ crc_at(6, (crc >> 8) as u8) ^ crc_at(5, (crc >> 16) as u8) ^
            crc_at(4, (crc >> 24) as u8) ^ crc_at(3, (crc >> 32) as u8) ^
            crc_at(2, (crc >> 40) as u8) ^
            crc_at(1, (crc >> 48) as u8) ^ crc_at(0, (crc >> 56) as u8)
    })
}
