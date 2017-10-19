# CRC32C

[![Crates.io](https://img.shields.io/crates/v/crc32c.svg)](https://crates.io/crates/crc32c)
[![Docs.rs](https://docs.rs/crc32c/badge.svg)](https://docs.rs/crc32c/)

Rust implementation of the CRC-32-Castagnoli algorithm.

Provides support for an optimized software implementation and a hardware-accelerated (SSE 4.2) one.
If you have SSE 4.2 enabled at compile time, it will only build the SSE implementation.
Otherwise, it will build both versions, and use `cpuid` to choose the best implementation at run time.

The code is inspired by [Mark Adler's CRC32C](https://stackoverflow.com/questions/17645167/) implementation.

## License
You may use this code under either the [Apache 2.0 license](https://www.apache.org/licenses/LICENSE-2.0)
or the [MIT license](https://opensource.org/licenses/MIT), at your option.
