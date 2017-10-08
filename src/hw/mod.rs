//! Implements crc32c with SSE 4.2 support.

use util;

use stdsimd::vendor as simd;

/// Computes CRC-32C using the SSE 4.2 hardware instruction.
pub fn crc32c(crci: u32, buffer: &[u8]) -> u32 {
    let mut crc = !crci;

    let (begin, middle, end) = util::split(buffer);

    crc = crc_u8(crc, begin);

    crc = crc_u64(crc, middle);

    crc = crc_u8(crc, end);

    !crc
}

fn crc_u8(crc: u32, buffer: &[u8]) -> u32 {
    buffer.iter().fold(crc,
        #[target_feature = "+sse4.2"] |crc, &next| {
            unsafe {
                self::simd::_mm_crc32_u8(crc, next)
            }
        })
}

fn crc_u64(crc: u32, buffer: &[u64]) -> u32 {
    let crc = crc as u64;

    let crc = buffer.iter().fold(crc,
    #[target_feature = "+sse4.2"] |crc, &next| {
        unsafe {
            self::simd::_mm_crc32_u64(crc, next)
        }
    });

    crc as u32
}
