use util;
use std::{mem, ops};

/// A matrix over the Galois field of two elements (0 and 1).
/// In this field, multiplication is equivalent to the (very fast) bitwise XOR.
#[derive(Debug, Copy, Clone)]
pub struct Matrix([u32; 32]);

impl Matrix {
    /// Allocates space for a new matrix.
    fn new() -> Self {
        unsafe { mem::uninitialized() }
    }

    /// Multiplies a matrix by itself.
    fn square(self) -> Self {
        let mut result = Self::new();

        for i in 0..32 {
            result[i] = self * self[i];
        }

        result
    }
}

impl ops::Index<u8> for Matrix {
    type Output = u32;

    #[inline]
    fn index(&self, i: u8) -> &Self::Output {
        &self.0[i as usize]
    }
}

impl ops::IndexMut<u8> for Matrix {
    #[inline]
    fn index_mut(&mut self, i: u8) -> &mut Self::Output {
        &mut self.0[i as usize]
    }
}

impl ops::Mul<u32> for Matrix {
    type Output = u32;

    /// Multiplies the matrix with a vector.
    fn mul(self, mut vec: u32) -> Self::Output {
        let mut sum = 0;
        let mut i = 0;

        while vec != 0 {
            if vec % 2 != 0 {
                sum ^= self[i];
            }

            vec /= 2;

            i += 1;
        }

        sum
    }
}

fn create_zero_operator(mut len: usize) -> Matrix {
    // Operator for odd powers-of-two.
    let mut odd = Matrix::new();

    odd[0] = util::POLYNOMIAL;
    let mut row = 1;

    for i in 1..32 {
        odd[i] = row;
        row *= 2;
    }

    let mut even = odd.square();

    let mut odd = even.square();

    loop {
        even = odd.square();

        len /= 2;

        if len == 0 {
            return even;
        }

        odd = even.square();

        len /= 2;

        if len == 0 {
            return odd;
        }
    }
}

pub struct CrcTable([[u32; 256]; 4]);

impl CrcTable {
    fn new(len: usize) -> Self {
        let mut zeroes: [[u32; 256]; 4] = unsafe { mem::uninitialized() };
        let op = create_zero_operator(len);

        for n in 0..256 {
            for i in 0..4 {
                let shift = i * 8;
                zeroes[i as usize][n] = op * ((n << shift) as u32);
            }
        }

        CrcTable(zeroes)
    }

    pub fn at(&self, i: u8, j: u8) -> u64 {
        let i = i as usize;
        let j = j as usize;
        self.0[i][j] as u64
    }

    pub fn shift(&self, crc: u64) -> u64 {
        self.at(0, crc as u8) ^ self.at(1, (crc >> 8) as u8) ^ self.at(2, (crc >> 16) as u8) ^
            self.at(3, (crc >> 24) as u8)
    }
}

pub const LONG: usize = 8192;
pub const SHORT: usize = 256;

lazy_static! {
	pub static ref LONG_TABLE: CrcTable = CrcTable::new(LONG);
	pub static ref SHORT_TABLE: CrcTable = CrcTable::new(SHORT);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_multiply() {
        let mut mat = Matrix::new();

        for i in 0..3 {
            mat[i] = i as u32;
        }

        let vec = 0b111;

        assert_eq!(mat * vec, 3);
    }

    #[test]
    fn zero_op() {
        let op = create_zero_operator(8192);
        assert_eq!(op[0], 0xe040e0ac);
    }
}
