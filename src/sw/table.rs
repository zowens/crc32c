use util;
use std::mem;

/// 8-KiB lookup table.
pub struct CrcTable([[u32; 256]; 8]);

impl CrcTable {
    /// Returns a pre-made CRC32 table.
    pub fn table() -> &'static CrcTable {
        &TABLE
    }

    /// Returns an entry from the table.
    #[inline]
    pub fn at(&self, i: u8, j: u8) -> u64 {
        unsafe { *self.0.get_unchecked(i as usize).get_unchecked(j as usize) as u64 }
    }
}

lazy_static! {
    /// Table for a quadword-at-a-time software CRC.
    static ref TABLE: CrcTable = {
        let mut table: [[u32; 256]; 8] = unsafe { mem::uninitialized() };

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

        CrcTable(table)
    };
}
