use std::{cmp, mem, slice};

/// Splits a buffer into three subslices:
/// - the first one is up to the first 8-byte aligned address.
/// - the second one is 8-byte aligned and its length is a multiple of 8.
/// - the third one is 8-byte aligned but its length is less than 8.
pub fn split(buffer: &[u8]) -> (&[u8], &[u64], &[u8]) {
    let (start, mid) = {
        let split_index = {
            let addr = buffer.as_ptr() as usize;

            // Align to multiples of 8.
            let aligned_addr = (addr + 7) & (!7);

            // Index of the next aligned element.
            let next_i = aligned_addr - addr;

            // Buffer might be too small.
            cmp::min(next_i, buffer.len())
        };

        buffer.split_at(split_index)
    };

    let (mid, end) = {
        // Round length down to multiples of 8.
        let split_index = mid.len() & (!7);

        mid.split_at(split_index)
    };

    let mid = unsafe {
        let ptr = mem::transmute(mid.as_ptr());
        let length = mid.len() / 8;

        slice::from_raw_parts(ptr, length)
    };

    (start, mid, end)
}
