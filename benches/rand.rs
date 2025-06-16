#[macro_use]
extern crate criterion;
extern crate crc32c;
extern crate rand;

use crc32c::{crc32c, crc32c_append, crc32c_combine};
use criterion::{Criterion, Throughput};
use rand::RngCore;

fn crc32c_megabyte(c: &mut Criterion) {
    let mut bytes = [0u8; 1_000_000];
    rand::rng().fill_bytes(&mut bytes);

    let mut group = c.benchmark_group("crc32_update_megabytes");
    group.throughput(Throughput::Bytes(1_000_000));
    group.bench_function("crc32_update_megabytes", move |b| b.iter(|| crc32c(&bytes)));
    group.finish();
}

fn crc32c_8kb(c: &mut Criterion) {
    let mut buffer = [0u8; 8192];
    rand::rng().fill_bytes(&mut buffer);

    let mut group = c.benchmark_group("crc32c_8kb");
    group.throughput(Throughput::Bytes(8192));
    group.bench_function("crc32c_8kb", move |b| b.iter(|| crc32c(&buffer)));
    group.finish();
}

/// benchmark combining 4KB blocks into existing check values.
fn crc32c_combine_4kb(c: &mut Criterion) {
    let mut buffer = [0u8; 4096];
    rand::rng().fill_bytes(&mut buffer);

    let crc_a = crc32c(b"abcd");
    let crc_b = crc32c(&buffer);

    let mut group = c.benchmark_group("crc32c_combine_4kb");
    group.bench_function("crc32c_combine_4kb", move |b| {
        b.iter(|| crc32c_combine(crc_a, crc_b, 4096))
    });
    group.finish();
}

/// benchmark appending 4KB blocks to existing check values
fn crc32c_append_4kb(c: &mut Criterion) {
    let mut buffer = [0u8; 4096];
    rand::rng().fill_bytes(&mut buffer);

    let crc_a = crc32c(b"abcd");

    let mut group = c.benchmark_group("crc32c_append_4kb");
    group.bench_function("crc32c_append_4kb", move |b| {
        b.iter(|| crc32c_append(crc_a, &buffer))
    });
    group.finish();
}

/// benchmark combining 1MB blocks into existing check values.
fn crc32c_combine_megabyte(c: &mut Criterion) {
    let mut buffer = [0u8; 1_000_000];
    rand::rng().fill_bytes(&mut buffer);

    let crc_a = crc32c(b"abcd");
    let crc_b = crc32c(&buffer);

    let mut group = c.benchmark_group("crc32c_combine_megabyte");
    group.bench_function("crc32c_combine_megabyte", move |b| {
        b.iter(|| crc32c_combine(crc_a, crc_b, 1_000_000))
    });
    group.finish();
}

fn crc32c_append_megabyte(c: &mut Criterion) {
    let mut buffer = [0u8; 1_000_000];
    rand::rng().fill_bytes(&mut buffer);

    let crc_a = crc32c(b"abcd");

    let mut group = c.benchmark_group("crc32c_append_megabyte");
    group.bench_function("crc32c_append_megabyte", move |b| {
        b.iter(|| crc32c_append(crc_a, &buffer))
    });
    group.finish();
}

criterion_group!(
    crc,
    crc32c_megabyte,
    crc32c_8kb,
    crc32c_append_4kb,
    crc32c_combine_4kb,
    crc32c_combine_megabyte,
    crc32c_append_megabyte
);
criterion_main!(crc);
