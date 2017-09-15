#![feature(test, cfg_target_feature)]
extern crate libc;
#[cfg(test)]
extern crate rand;
#[cfg(test)]
extern crate test;

use libc::{c_void, uint32_t, size_t};

#[allow(dead_code)]
extern {
    fn crc32c_hw(crc: uint32_t, buf: *const c_void, len: size_t) -> uint32_t;
    fn crc32c_sw(crc: uint32_t, buf: *const c_void, len: size_t) -> uint32_t;
}

#[cfg(target_feature="sse4.2")]
pub fn crc32c(data: &[u8]) -> u32 {
    let len = data.len();
    unsafe {
        crc32c_hw(0, data.as_ptr() as *const c_void, len) as u32
    }
}

#[cfg(not(target_feature="sse4.2"))]
pub fn crc32c(data: &[u8]) -> u32 {
    let len = data.len();
    unsafe {
        crc32c_sw(0, data.as_ptr() as *const c_void, len) as u32
    }
}


#[cfg(test)]
mod tests {
    use rand::{OsRng, Rng};
    use super::crc32c;
    use test::Bencher;

    #[test]
    fn test_crc() {
        let v = crc32c(b"012345678910");
        assert_eq!(0x8412e281, v);
    }

    #[bench]
    fn crc(b: &mut Bencher) {
        let mut bytes = [0u8; 8000];

        let mut r = OsRng::new().unwrap();
        r.fill_bytes(&mut bytes);

        b.iter(|| crc32c(&bytes));
    }
}
