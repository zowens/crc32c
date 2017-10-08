//! Implements crc32c without hardware support.

use util;

mod table;
use self::table::crc_at;

/// Software implementation of the algorithm.
pub fn crc32c(crci: u32, buffer: &[u8]) -> u32 {
    let mut crc = !crci as u64;

    let (start, mid, end) = util::split(buffer);

    crc = crc_unaligned(crc, start);

    crc = crc_aligned(crc, mid);

    crc = crc_unaligned(crc, end);

    !(crc as u32)
}

fn crc_unaligned(crci: u64, buffer: &[u8]) -> u64 {
    buffer.iter().fold(crci, |crc, &next| {
        let index = (crc ^ (next as u64)) as u8;
        crc_at(0, index) ^ (crc >> 8)
    })
}

fn crc_aligned(crci: u64, buffer: &[u64]) -> u64 {
    buffer.iter().fold(crci, |crc, &next| {
        let crc = crc ^ next;

        (1..8).fold(crc_at(7, crc as u8), |tmp, i| {
            let row = 7 - i;
            let shift = 8 * i;
            let column = (crc >> shift) as u8;
            tmp ^ crc_at(row, column)
        })
    })
}
