// below 1500 ns

use util;
use std::{mem, ops};

// A matrix over Galois Field 2, 32x32 (each bit is an element).
#[derive(Debug, Copy, Clone)]
struct Matrix([u32; 32]);

impl Matrix {
    fn new() -> Self {
        unsafe { mem::uninitialized() }
    }

    fn square(self) -> Self {
        let mut result = Self::new();

        for i in 0..self.0.len() {
            result.0[i] = self * self.0[i];
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

    fn mul(self, mut vec: u32) -> Self::Output {
        // TODO: improve

        let mut sum = 0;
        let mut i = 0;

        while vec != 0 {
            if vec % 2 == 0 {
                sum ^= self.0[i];
            }

            vec /= 2;

            i += 1;
        }

        sum
    }
}

fn zeros_operator(mut len: usize) -> Matrix {
    // Operator for odd powers-of-two.
    let mut odd = Matrix::new();

    odd[0] = util::POLYNOMIAL;
    let mut row = 1;

    for i in 1..32 {
        odd[i] = row;
        row <<= 1;
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
        let op = zeros_operator(len);

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
