use util;
use std::mem;

/// Returns an entry from the table.
#[inline]
pub fn crc_at(i: u8, j: u8) -> u64 {
    unsafe { *TABLE.get_unchecked(j as usize).get_unchecked(i as usize) as u64 }
}

/// 8-KiB lookup table.
type CrcTable = [[u32; 256]; 8];

lazy_static! {
    /// Table for a quadword-at-a-time software CRC.
    static ref TABLE: CrcTable = {
        let mut table: CrcTable = unsafe { mem::uninitialized() };

        for n in 0..256 {
            table[0][n as usize] = (0..8).fold(n, |crc, _| {
                if crc % 2 == 0 {
                    crc >> 1
                } else {
                    (crc >> 1) ^ util::POLYNOMIAL
                }
            });
        }

        for n in 0..256 {
            let mut crc = table[0][n as usize];
            for k in 1..8 {
                crc = table[0][(crc as u8) as usize] ^ (crc >> 8);
                table[k as usize][n as usize] = crc;
            }
        }

        table
    };
}
