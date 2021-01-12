use crate::util;
use crate::sw;

use std::arch::aarch64 as simd;

pub unsafe fn crc32c(crci: u32, buffer: &[u8]) -> u32 {
    let mut crc0 = crci;
    let (begin, middle, end) = util::split(buffer);

    // We're effectively cheating by using the software implementation
    // for now. The bit-flips simulate going back-and-forth between
    // the inner computations of the software implementation
    //
    // This needs a little more optimization, and to use the typical
    // crc32cb instruction rather than using the software implementation.
    crc0 = !sw::crc32c(crc0, begin);
    crc0 = !crc_u64(crc0, middle);
    sw::crc32c(crc0, end)
}

#[inline(always)]
unsafe fn crc_u64(crc: u32, words: &[u64]) -> u32 {
    words
        .iter()
        .fold(crc, |crc, &next| crc_u64_append(crc, next))
}

#[inline(always)]
unsafe fn crc_u64_append(crc: u32, next: u64) -> u32 {
    simd::__crc32cd(crc, next)
}
