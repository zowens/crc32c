// SPDX-License-Identifier: Apache-2.0
// Folding kernel adapted from crc32fast's AArch64 implementation.
// Copyright (c) 2018 Sam Rijs, Alex Crichton and contributors.
// https://github.com/srijs/rust-crc32fast/blob/bf120c3ac0c28e5d999de2df8139f47f4486f622/src/specialized/aarch64.rs

use std::arch::aarch64 as simd;
use std::convert::TryInto;

const POLYNOMIAL: u32 = 0x82F6_3B78;
const FOLD_128: [u64; 2] = fold_keys(128);
const FOLD_256: [u64; 2] = fold_keys(256);
const FOLD_512: [u64; 2] = fold_keys(512);
const FOLD_1024: [u64; 2] = fold_keys(1024);
const FOLD_2048: [u64; 2] = fold_keys(2048);

// x^bits modulo the Castagnoli polynomial, reflected, shifted for PMULL.
const fn power(mut bits: u32) -> u64 {
    let mut remainder = 0x8000_0000u32;
    while bits != 0 {
        remainder = (remainder >> 1) ^ ((remainder & 1).wrapping_neg() & POLYNOMIAL);
        bits -= 1;
    }
    (remainder as u64) << 1
}

const fn fold_keys(bits: u32) -> [u64; 2] {
    [power(bits + 32), power(bits - 32)]
}

// The caller checks CRC and AES/PMULL support and supplies at least 64 bytes.
#[target_feature(enable = "crc,aes")]
pub(super) unsafe fn crc32c(crc: u32, data: &[u8]) -> u32 {
    debug_assert!(data.len() >= 64);
    match data.len() {
        0..=255 => fold_streams::<4>(crc, data, FOLD_512),
        256..=4095 => fold_streams::<8>(crc, data, FOLD_1024),
        _ => fold_streams::<16>(crc, data, FOLD_2048),
    }
}

// Each lane represents 16 input bytes. Fold lanes forward in parallel, then merge
// pairs until one remains; the final hardware CRC reduction uses Castagnoli too.
#[inline]
#[target_feature(enable = "crc,aes")]
unsafe fn fold_streams<const N: usize>(crc: u32, mut data: &[u8], keys: [u64; 2]) -> u32 {
    let mut lanes = [simd::vdupq_n_u64(0); N];
    for lane in &mut lanes {
        *lane = load(&mut data);
    }
    lanes[0] = simd::veorq_u64(
        lanes[0],
        simd::vsetq_lane_u64::<0>(!crc as u64, simd::vdupq_n_u64(0)),
    );

    let keys = simd::vld1q_u64(keys.as_ptr());
    while data.len() >= 16 * N {
        for lane in &mut lanes {
            *lane = fold(*lane, load(&mut data), keys);
        }
    }

    // Explicit levels keep the accumulators in registers.
    macro_rules! halve {
        ($half:expr, $keys:expr) => {
            if N >= 2 * $half {
                let keys = simd::vld1q_u64($keys.as_ptr());
                for i in 0..$half {
                    lanes[i] = fold(lanes[i], lanes[i + $half], keys);
                }
                if data.len() >= 16 * $half {
                    for lane in &mut lanes[..$half] {
                        *lane = fold(*lane, load(&mut data), keys);
                    }
                }
            }
        };
    }
    halve!(8, FOLD_1024);
    halve!(4, FOLD_512);
    halve!(2, FOLD_256);
    halve!(1, FOLD_128);

    let crc = simd::__crc32cd(0, simd::vgetq_lane_u64::<0>(lanes[0]));
    let mut crc = simd::__crc32cd(crc, simd::vgetq_lane_u64::<1>(lanes[0]));
    if data.len() >= 8 {
        crc = simd::__crc32cd(crc, u64::from_le_bytes(data[..8].try_into().unwrap()));
        data = &data[8..];
    }
    if data.len() >= 4 {
        crc = simd::__crc32cw(crc, u32::from_le_bytes(data[..4].try_into().unwrap()));
        data = &data[4..];
    }
    if data.len() >= 2 {
        crc = simd::__crc32ch(crc, u16::from_le_bytes(data[..2].try_into().unwrap()));
        data = &data[2..];
    }
    if let Some(&byte) = data.first() {
        crc = simd::__crc32cb(crc, byte);
    }
    !crc
}

#[inline]
#[target_feature(enable = "aes")]
unsafe fn fold(
    left: simd::uint64x2_t,
    right: simd::uint64x2_t,
    keys: simd::uint64x2_t,
) -> simd::uint64x2_t {
    let low = simd::vmull_p64(
        simd::vgetq_lane_u64::<0>(left),
        simd::vgetq_lane_u64::<0>(keys),
    );
    let high = simd::vmull_high_p64(
        simd::vreinterpretq_p64_u64(left),
        simd::vreinterpretq_p64_u64(keys),
    );
    simd::veorq_u64(
        right,
        simd::veorq_u64(
            simd::vreinterpretq_u64_p128(low),
            simd::vreinterpretq_u64_p128(high),
        ),
    )
}

#[inline(always)]
unsafe fn load(data: &mut &[u8]) -> simd::uint64x2_t {
    // Check bounds before the unaligned vector load, even if a caller miscounts.
    let block = simd::vld1q_u8(data[..16].as_ptr());
    *data = &data[16..];
    simd::vreinterpretq_u64_u8(block)
}

#[cfg(test)]
mod tests {
    fn supported() -> bool {
        std::arch::is_aarch64_feature_detected!("crc")
            && std::arch::is_aarch64_feature_detected!("aes")
    }

    fn input(length: usize) -> Vec<u8> {
        let mut state = 0x1234_5678u32;
        (0..length)
            .map(|_| {
                state ^= state << 13;
                state ^= state >> 17;
                state ^= state << 5;
                state as u8
            })
            .collect()
    }

    #[test]
    fn lengths_alignments_and_seeds_match_software() {
        if !supported() {
            return;
        }
        let bytes = input(8192 + 16);
        let lengths = (64..=1024).chain([2047, 2048, 4095, 4096, 4097, 8191, 8192]);
        for length in lengths {
            for offset in 0..16 {
                let data = &bytes[offset..offset + length];
                for seed in [0, 1, u32::MAX, 0xDEAD_BEEF] {
                    assert_eq!(
                        unsafe { super::crc32c(seed, data) },
                        crate::sw::crc32c(seed, data),
                        "length={length} offset={offset} seed={seed:#x}"
                    );
                }
            }
        }
    }

    #[test]
    fn dispatch_and_crc_only_fallback_match_software() {
        if !std::arch::is_aarch64_feature_detected!("crc") {
            return;
        }
        let bytes = input(65537 + 16);
        for length in (0..=257).chain([4095, 4096, 4097, 24575, 24576, 24577, 65537]) {
            for offset in 0..16 {
                let data = &bytes[offset..offset + length];
                for seed in [0, u32::MAX, 0xDEAD_BEEF] {
                    let expected = crate::sw::crc32c(seed, data);
                    assert_eq!(unsafe { super::super::crc32c(seed, data) }, expected);
                    assert_eq!(unsafe { super::super::crc32c_crc(seed, data) }, expected);
                }
            }
        }
    }

    #[test]
    fn streaming_and_repeated_patterns_preserve_checksums() {
        if !supported() {
            return;
        }
        let mut bytes = input(65537);
        for pattern in [None, Some(0), Some(255), Some(0x55), Some(0xAA)] {
            if let Some(byte) = pattern {
                bytes.fill(byte);
            }
            for seed in [0, u32::MAX, 0xDEAD_BEEF] {
                let expected = crate::sw::crc32c(seed, &bytes);
                for chunk_size in [1, 7, 63, 64, 65, 255, 256, 257, 4095, 4096, 16384] {
                    let actual = bytes.chunks(chunk_size).fold(seed, crate::crc32c_append);
                    assert_eq!(actual, expected, "chunk_size={chunk_size} seed={seed:#x}");
                }
            }
        }
        assert_eq!(crate::crc32c(b"123456789"), 0xE306_9283);
    }
}
