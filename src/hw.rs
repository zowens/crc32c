//! Implements crc32c with SSE 4.2 support.

use util;

use std::arch::x86_64 as simd;

pub struct CrcTable([[u32; 256]; 4]);

impl CrcTable {
    pub fn at(&self, i: u8, j: u8) -> u64 {
        let i = i as usize;
        let j = j as usize;
        u64::from(self.0[i][j])
    }

    pub fn shift(&self, crc: u64) -> u64 {
        let mut result = self.at(0, crc as u8);

        for i in 1..4 {
            let shift = i * 8;
            result ^= self.at(i, (crc >> shift) as u8);
        }

        result
    }
}

const LONG: usize = 8192;
const SHORT: usize = 256;
const LONG_TABLE: CrcTable = CrcTable(include!(concat!(env!("OUT_DIR"), "/", "hw.long.table")));
const SHORT_TABLE: CrcTable = CrcTable(include!(concat!(env!("OUT_DIR"), "/", "hw.short.table")));

/// Computes CRC-32C using the SSE 4.2 hardware instruction.
pub fn crc32c(crci: u32, buffer: &[u8]) -> u32 {
    let mut crc0 = u64::from(!crci);

    let (begin, middle, end) = util::split(buffer);

    // Leading bytes, up to the first one aligned on 8 bytes.
    crc0 = crc_u8(crc0, begin);

    // Most CPUs have a latency of 3 on these instructions,
    // meaning we must use 3 of them at a time, to leverage
    // hardware parallelism.

    // First do chunks of size LONG * 3.
    let chunk_size = (LONG * 3) / 8;
    let last_chunk = middle.len() / chunk_size * chunk_size;

    let (middle_first, middle_last) = middle.split_at(last_chunk);

    crc0 = crc_u64_parallel3(crc0, chunk_size, &LONG_TABLE, middle_first);

    // Now do chunks of size SHORT * 3.
    let chunk_size = (SHORT * 3) / 8;
    let last_chunk = middle_last.len() / chunk_size * chunk_size;

    let (middle_last_first, middle_last_last) = middle_last.split_at(last_chunk);

    crc0 = crc_u64_parallel3(crc0, chunk_size, &SHORT_TABLE, middle_last_first);

    // Now the last part, less than SHORT * 3 but still a multiple of 8-bytes.
    crc0 = crc_u64(crc0, middle_last_last);

    // Final unaligned remainder.
    crc0 = crc_u8(crc0, end);

    !(crc0 as u32)
}

#[inline]
#[target_feature(enable = "sse4.2")]
unsafe fn crc_u8_append(crc: u64, next: u8) -> u64 {
    u64::from(self::simd::_mm_crc32_u8(crc as u32, next))
}

#[inline]
#[target_feature(enable = "sse4.2")]
unsafe fn crc_u64_append(crc: u64, next: u64) -> u64 {
    self::simd::_mm_crc32_u64(crc, next)
}

#[inline]
fn crc_u8(crc: u64, buffer: &[u8]) -> u64 {
    unsafe {
        buffer
            .iter()
            .fold(crc, |crc, &next| crc_u8_append(crc, next))
    }
}

#[inline]
fn crc_u64(crc: u64, buffer: &[u64]) -> u64 {
    unsafe {
        buffer
            .iter()
            .fold(crc, |crc, &next| crc_u64_append(crc, next))
    }
}

/// Hardware-parallel version of the algorithm.
///
/// Calculates the CRC for a chunk of `chunk_size`,
/// by dividing it in 3 separate blocks.
///
/// Uses a pre-made CRC table designed for the given chunk size.
#[inline]
fn crc_u64_parallel3(crc: u64, chunk_size: usize, table: &CrcTable, buffer: &[u64]) -> u64 {
    buffer.chunks(chunk_size).fold(crc, |mut crc0, chunk| {
        let mut crc1 = 0;
        let mut crc2 = 0;

        // Divide it in three.
        let block_size = chunk_size / 3;

        let mut blocks = chunk.chunks(block_size);
        let a = blocks.next().unwrap();
        let b = blocks.next().unwrap();
        let c = blocks.next().unwrap();

        for i in 0..block_size {
            unsafe {
                crc0 = crc_u64_append(crc0, a[i]);
                crc1 = crc_u64_append(crc1, b[i]);
                crc2 = crc_u64_append(crc2, c[i]);
            }
        }

        crc0 = table.shift(crc0) ^ crc1;
        crc0 = table.shift(crc0) ^ crc2;

        crc0
    })
}
