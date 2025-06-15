extern crate crc32c;
extern crate rand;

use crc32c::{crc32c, crc32c_append, crc32c_combine};
use rand::RngCore;

#[test]
fn crc_combine() {
    for a_length in 0..12 {
        for b_length in 0..12 {
            let mut a_buf = vec![0u8; a_length];
            let mut b_buf = vec![0u8; b_length];
            rand::rng().fill_bytes(&mut a_buf);
            rand::rng().fill_bytes(&mut b_buf);

            let a = crc32c(&a_buf);
            let b = crc32c(&b_buf);
            let appended = crc32c_append(a, &b_buf);

            let _ = &a_buf.append(&mut b_buf);

            let ab = crc32c(&a_buf);
            let combined = crc32c_combine(a, b, b_length);

            assert_eq!(ab, appended);
            assert_eq!(ab, combined);
        }
    }
}

#[test]
fn crc() {
    let v = crc32c(b"012345678910");
    assert_eq!(0x84_12_E2_81, v);
}

#[test]
fn crc_append() {
    let v = crc32c(b"01234");
    let v = crc32c_append(v, b"5678910");
    assert_eq!(0x84_12_E2_81, v);
}

// Tests the smallest possible message.
#[test]
fn very_small() {
    let v = crc32c(b"1");
    assert_eq!(0x90_F5_99_E3, v);
}

#[test]
fn long_string() {
    let v =
        crc32c(b"This is a very long string which is used to test the CRC-32-Castagnoli function.");
    assert_eq!(0x20_CB_1E_59, v);
}

// Tests a 32-KiB buffer.
#[test]
fn very_big() {
    let buf = String::from("Hello!.\n").repeat(8192 * 4);

    assert_eq!(0x12_BD_91_91, crc32c::crc32c(buf.as_bytes()));
}
