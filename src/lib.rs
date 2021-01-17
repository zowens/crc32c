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

#![cfg_attr(nightly, feature(stdsimd, asm, aarch64_target_feature))]

#[cfg(all(target_arch = "aarch64", nightly))]
mod hw_aarch64;
#[cfg(any(target_arch = "x86_64", all(target_arch = "aarch64", nightly)))]
mod hw_tables;
#[cfg(target_arch = "x86_64")]
mod hw_x86_64;
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
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("sse4.2") {
            return unsafe { hw_x86_64::crc32c(crc, data) };
        }
    }

    #[cfg(all(target_arch = "aarch64", nightly))]
    {
        if is_aarch64_feature_detected!("crc") {
            return unsafe { hw_aarch64::crc32c(crc, data) };
        }
    }

    sw::crc32c(crc, data)
}
