//! This crate provides the CRC-32-Castagnoli algorithm.
//!
//! It provides both a software implementation, and a hardware-optimized one for SSE 4.2.
//!
//! # Example
//!
//! ```rust
//! let message = b"Hello world!";
//!
//! let crc = crc32c::crc32c(message);
//!
//! assert_eq!(crc, 0x7B_98_E7_51);
//! ```
//!
//! # Enabling hardware acceleration
//!
//! If you compile your code with `-C target-features=+sse4.2`,
//! then the hardware-optimized version will be compiled into the code.
//!
//! Otherwise, the crate will use `cpuid` at runtime to detect the
//! running CPU's features, and enable the appropiate algorithm.

#![feature(cfg_target_feature, target_feature, stdsimd)]

mod hw;
mod sw;
mod util;

/// Computes the CRC for the data payload.
///
/// Equivalent to calling `crc32c_append(0, data)`.
#[inline]
pub fn crc32c(data: &[u8]) -> u32 {
    crc32c_append(0, data)
}

/// Computes the CRC for the data payload, starting with a previous CRC value.
#[inline]
pub fn crc32c_append(crc: u32, data: &[u8]) -> u32 {
    if is_x86_feature_detected!("sse4.2") {
        unsafe { hw::crc32c(crc, data) }
    } else {
        sw::crc32c(crc, data)
    }
}
