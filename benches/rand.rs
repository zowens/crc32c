#![feature(test)]

extern crate test;
use test::Bencher;

extern crate rand;
use rand::{OsRng, Rng};

extern crate crc32c;
use crc32c::crc32c;

#[bench]
fn crc(b: &mut Bencher) {
    let mut bytes = [0u8; 8192];

    let mut r = OsRng::new().unwrap();
    r.fill_bytes(&mut bytes);

    b.iter(|| crc32c(&bytes));
}
