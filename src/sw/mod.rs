//! Implements crc32c without hardware support.

use util;

mod table;
use self::table::CrcTable;

/// Software implementation of the algorithm.
pub fn crc32c(crci: u32, buffer: &[u8]) -> u32 {
    let table = CrcTable::table();

    let mut crc = u64::from(!crci);

    let (start, mid, end) = util::split(buffer);

    crc = crc_u8(table, crc, start);

    crc = crc_u64(table, crc, mid);

    crc = crc_u8(table, crc, end);

    !(crc as u32)
}

#[inline]
fn crc_u8(table: &CrcTable, crc: u64, buffer: &[u8]) -> u64 {
    buffer.iter().fold(crc, |crc, &next| {
        let index = (crc ^ u64::from(next)) as u8;
        table.at(0, index) ^ (crc >> 8)
    })
}

#[inline]
fn crc_u64(table: &CrcTable, crci: u64, buffer: &[u64]) -> u64 {
    buffer.iter().fold(crci, |crc, &next| {
        let crc = crc ^ next;

        // Note: I've tried refactoring this to a for-loop,
        // but then it gets worse performance.
        table.at(7, crc as u8) ^ table.at(6, (crc >> 8) as u8) ^ table.at(5, (crc >> 16) as u8) ^
            table.at(4, (crc >> 24) as u8) ^
            table.at(3, (crc >> 32) as u8) ^ table.at(2, (crc >> 40) as u8) ^
            table.at(1, (crc >> 48) as u8) ^ table.at(0, (crc >> 56) as u8)
    })
}
