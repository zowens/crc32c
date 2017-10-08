//! Implements crc32c with SSE 4.2 support.

use util;

use stdsimd::vendor as simd;

mod table;

use std::{mem, slice};

/// Computes CRC-32C using the SSE 4.2 hardware instruction.
#[target_feature = "+sse4.2"]
pub fn crc32c(crci: u32, buffer: &[u8]) -> u32 {
    let mut crc0 = (!crci) as u64;

    let (begin, middle, end) = util::split(buffer);

    // Leading bytes, up to the first one aligned on 8 bytes.
    crc0 = crc_u8(crc0, begin);

    // Most CPUs have a latency of 3 on these instructions,
    // meaning we must use 3 of them at a time, to leverage
    // hardware parallelism.

    let mut len = middle.len();
    let mut ptr: *const u64 = unsafe { mem::transmute(middle.as_ptr()) };

    unsafe {
        let chunk_length = table::LONG;

        while len >= table::LONG * 3 {
            let mut crc1 = 0;
            let mut crc2 = 0;

            let slice0 = slice::from_raw_parts(ptr, chunk_length);
            let slice1 =
                slice::from_raw_parts(ptr.offset((table::LONG / 8) as isize), chunk_length);
            let slice2 =
                slice::from_raw_parts(ptr.offset(((table::LONG / 8) * 2) as isize), chunk_length);

            for i in 0..chunk_length / 8 {
                crc0 = self::simd::_mm_crc32_u64(crc0, slice0[i]);
                crc1 = self::simd::_mm_crc32_u64(crc1, slice1[i]);
                crc2 = self::simd::_mm_crc32_u64(crc2, slice2[i]);
            }

            crc0 = table::LONG_TABLE.shift(crc0) ^ crc1;
            crc0 = table::LONG_TABLE.shift(crc0) ^ crc2;

            len -= table::LONG * 3;
            ptr = ptr.offset(((table::LONG / 8) * 3) as isize);
        }
    }

    unsafe {
        let chunk_length = table::SHORT;

        while len >= table::SHORT * 3 {
            let mut crc1 = 0;
            let mut crc2 = 0;

            let slice0 = slice::from_raw_parts(ptr, chunk_length);
            let slice1 =
                slice::from_raw_parts(ptr.offset((table::SHORT / 8) as isize), chunk_length);
            let slice2 =
                slice::from_raw_parts(ptr.offset(((table::SHORT / 8) * 2) as isize), chunk_length);

            for i in 0..chunk_length / 8 {
                crc0 = self::simd::_mm_crc32_u64(crc0, slice0[i]);
                crc1 = self::simd::_mm_crc32_u64(crc1, slice1[i]);
                crc2 = self::simd::_mm_crc32_u64(crc2, slice2[i]);
            }

            crc0 = table::SHORT_TABLE.shift(crc0) ^ crc1;
            crc0 = table::SHORT_TABLE.shift(crc0) ^ crc2;

            len -= table::SHORT * 3;
            ptr = ptr.offset(((table::SHORT / 8) * 3) as isize);
        }
    }

    unsafe {
        let slice = slice::from_raw_parts(ptr, len);

        crc0 = crc_u64(crc0, slice);
    }

    crc0 = crc_u8(crc0, end);

    !(crc0 as u32)
}

fn crc_u8(crc: u64, buffer: &[u8]) -> u64 {
    buffer.iter().fold(
        crc, #[target_feature = "+sse4.2"]
        #[target_feature = "+sse4.2"]
        |crc, &next| unsafe { self::simd::_mm_crc32_u8(crc as u32, next) as u64 },
    )
}

fn crc_u64(crc: u64, buffer: &[u64]) -> u64 {
    buffer.iter().fold(
        crc, #[target_feature = "+sse4.2"]
        #[target_feature = "+sse4.2"]
        |crc, &next| unsafe { self::simd::_mm_crc32_u64(crc, next) },
    )
}
