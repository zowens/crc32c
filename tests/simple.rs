extern crate crc32c;
use crc32c::{crc32c, crc32c_append};

#[test]
fn crc() {
    let v = crc32c(b"012345678910");
    assert_eq!(0x8412E281, v);
}

#[test]
fn crc_append() {
    let v = crc32c(b"01234");
    let v = crc32c_append(v, b"5678910");
    assert_eq!(0x8412E281, v);
}

// Tests the smallest possible message.
#[test]
fn very_small() {
    let v = crc32c(b"1");
    assert_eq!(0x90F599E3, v);
}

#[test]
fn long_string() {
    let v = crc32c(
        b"This is a very long string which is used to test the CRC-32-Castagnoli function.",
    );
    assert_eq!(0x20CB1E59, v);
}

// Tests a 32-KiB buffer.
#[test]
fn very_big() {
	let buf = String::from("Hello!.\n").repeat(32768);

	assert_eq!(0x12bd9191, crc32c::crc32c(buf.as_bytes()));
}
