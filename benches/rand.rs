#[macro_use]
extern crate criterion;
extern crate crc32c;
extern crate rand;

use crc32c::crc32c;
use criterion::{Benchmark, Criterion, Throughput};
use rand::{rngs::OsRng, RngCore};

fn crc32c_megabyte(c: &mut Criterion) {
    let mut bytes = [0u8; 1_000_000];
    OsRng.fill_bytes(&mut bytes);

    c.bench(
        "crc32_update_megabytes",
        Benchmark::new("crc32_update_megabytes", move |b| b.iter(|| crc32c(&bytes)))
            .throughput(Throughput::Bytes(1_000_000)),
    );
}

criterion_group!(crc, crc32c_megabyte);
criterion_main!(crc);
