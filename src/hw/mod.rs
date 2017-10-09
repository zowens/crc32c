//! Implements crc32c with SSE 4.2 support.

use util;

use stdsimd::vendor as simd;

mod table;

/// Computes CRC-32C using the SSE 4.2 hardware instruction.
pub fn crc32c(crci: u32, buffer: &[u8]) -> u32 {
    let mut crc0 = (!crci) as u64;

    let (begin, middle, end) = util::split(buffer);

    // Leading bytes, up to the first one aligned on 8 bytes.
    crc0 = crc_u8(crc0, begin);

    // Most CPUs have a latency of 3 on these instructions,
    // meaning we must use 3 of them at a time, to leverage
    // hardware parallelism.

    let chunk_size = (table::LONG * 3) / 8;
    let last_chunk = middle.len() / chunk_size * chunk_size;

    let (middle_first, middle_last) = middle.split_at(last_chunk);

    crc0 = crc_u64_parallel3(crc0, chunk_size, &table::LONG_TABLE, middle_first);

    let chunk_size = (table::SHORT * 3) / 8;
    let last_chunk = middle_last.len() / chunk_size * chunk_size;

    let (middle_last_first, middle_last_last) = middle_last.split_at(last_chunk);

    crc0 = crc_u64_parallel3(crc0, chunk_size, &table::SHORT_TABLE, middle_last_first);

    crc0 = crc_u64(crc0, middle_last_last);

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

/// Hardware-parallel version of the algorithm.
fn crc_u64_parallel3(crc: u64, chunk_size: usize, table: &table::CrcTable, buffer: &[u64]) -> u64 {
    buffer.chunks(chunk_size).fold(
        crc, #[target_feature = "+sse4.2"]
        #[target_feature = "+sse4.2"]
        |mut crc0, chunk| {
            let mut crc1 = 0;
            let mut crc2 = 0;

            // Divide it in three
            let block_size = chunk_size / 3;

            let mut blocks = chunk.chunks(block_size);
            let a = blocks.next().unwrap();
            let b = blocks.next().unwrap();
            let c = blocks.next().unwrap();
            assert_eq!(blocks.next(), None);

            for i in 0..block_size {
                unsafe {
                    crc0 = self::simd::_mm_crc32_u64(crc0, a[i]);
                    crc1 = self::simd::_mm_crc32_u64(crc1, b[i]);
                    crc2 = self::simd::_mm_crc32_u64(crc2, c[i]);
                }
            }

            crc0 = table.shift(crc0) ^ crc1;
            crc0 = table.shift(crc0) ^ crc2;

            crc0
        },
    )
}
