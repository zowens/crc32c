use crc::{Crc, CRC_32_ISCSI};
use crc32c::{crc32c, crc32c_append, crc32c_combine};
use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use rand::{rngs::OsRng, RngCore};

const CASTAGNOLI: Crc<u32> = Crc::<u32>::new(&CRC_32_ISCSI);
const MIB1: usize = 1024 * 1024;
const KIB8: usize = 8 * 1024;
const KIB4: usize = 4 * 1024;

fn direct_megabyte(c: &mut Criterion) {
    let mut bytes = [0u8; MIB1];
    OsRng.fill_bytes(&mut bytes);

    let mut group = c.benchmark_group("1MiB");
    group.throughput(Throughput::Bytes(MIB1 as u64));
    group.bench_function("crc32c", move |b| b.iter(|| crc32c(&bytes)));
    group.bench_function("crc_crate", move |b| b.iter(|| CASTAGNOLI.checksum(&bytes)));
    group.finish();
}

fn direct_8kib(c: &mut Criterion) {
    let mut bytes = [0u8; KIB8];
    OsRng.fill_bytes(&mut bytes);

    let mut group = c.benchmark_group("8KiB");
    group.throughput(Throughput::Bytes(KIB8 as u64));
    group.bench_function("crc32c", move |b| b.iter(|| crc32c(&bytes)));
    group.bench_function("crc_crate", move |b| b.iter(|| CASTAGNOLI.checksum(&bytes)));
    group.finish();
}

/// benchmark combining 4KiB blocks into existing check values.
fn combine_4kib(c: &mut Criterion) {
    let mut buffer = [0u8; KIB4];
    OsRng.fill_bytes(&mut buffer);

    let crc_a = crc32c(b"abcd");
    let crc_b = crc32c(&buffer);

    let mut group = c.benchmark_group("combine_4KiB");
    group.bench_function("crc32c", move |b| {
        b.iter(|| crc32c_combine(crc_a, crc_b, 4096))
    });
    group.finish();
}

/// benchmark appending 4KiB blocks to existing check values
fn append_4kib(c: &mut Criterion) {
    let mut buffer = [0u8; KIB4];
    OsRng.fill_bytes(&mut buffer);

    let crc_a = crc32c(b"abcd");

    let mut group = c.benchmark_group("append_4KiB");
    group.throughput(Throughput::Bytes(KIB4 as u64));
    group.bench_function("crc32c", move |b| b.iter(|| crc32c_append(crc_a, &buffer)));
    group.finish();
}

/// benchmark combining 1MiB blocks into existing check values.
fn combine_megabyte(c: &mut Criterion) {
    let mut buffer = [0u8; MIB1];
    OsRng.fill_bytes(&mut buffer);

    let crc_a = crc32c(b"abcd");
    let crc_b = crc32c(&buffer);

    let mut group = c.benchmark_group("combine_1MiB");
    group.bench_function("crc32c", move |b| {
        b.iter(|| crc32c_combine(crc_a, crc_b, 1_000_000))
    });
    group.finish();
}

fn append_megabyte(c: &mut Criterion) {
    let mut buffer = [0u8; MIB1];
    OsRng.fill_bytes(&mut buffer);

    let crc_a = crc32c(b"abcd");

    let mut group = c.benchmark_group("append_1MiB");
    group.throughput(Throughput::Bytes(MIB1 as u64));
    group.bench_function("crc32c", move |b| b.iter(|| crc32c_append(crc_a, &buffer)));
    group.finish();
}

criterion_group!(
    crc,
    direct_megabyte,
    direct_8kib,
    append_4kib,
    combine_4kib,
    combine_megabyte,
    append_megabyte
);
criterion_main!(crc);
