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
        let i = i as usize;
        let j = j as usize;
        u64::from(self.0[i][j])
    }
}

lazy_static! {
    /// Table for a quadword-at-a-time software CRC.
    static ref TABLE: CrcTable = {
        let mut table: [[u32; 256]; 8] = unsafe { mem::uninitialized() };

        for n in 0..256 {
            let mut crc = n;

            for _ in 0..8 {
                if crc % 2 == 0 {
                    crc /= 2;
                } else {
                    crc /= 2;
                    crc ^= util::POLYNOMIAL;
                }
            }

            table[0][n as usize] = crc;
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
