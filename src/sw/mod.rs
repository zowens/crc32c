//! Implements crc32c without hardware support.

use util;

mod table;
use self::table::CrcTable;

/// Software implementation of the algorithm.
pub fn crc32c(crci: u32, buffer: &[u8]) -> u32 {
    let table = CrcTable::table();

    let mut crc = (!crci) as u64;

    let (start, mid, end) = util::split(buffer);

    crc = crc_u8(table, crc, start);

    crc = crc_u64(table, crc, mid);

    crc = crc_u8(table, crc, end);

    !(crc as u32)
}

fn crc_u8(table: &CrcTable, crc: u64, buffer: &[u8]) -> u64 {
    buffer.iter().fold(crc, |crc, &next| {
        let index = (crc ^ (next as u64)) as u8;
        table.at(0, index) ^ (crc >> 8)
    })
}

fn crc_u64(table: &CrcTable, crci: u64, buffer: &[u64]) -> u64 {
    buffer.iter().fold(crci, |crc, &next| {
        let crc = crc ^ next;

        (1..8).fold(table.at(7, crc as u8), |tmp, i| {
            let row = 7 - i;
            let shift = 8 * i;
            let column = (crc >> shift) as u8;
            tmp ^ table.at(row, column)
        })
    })
}
