#[derive(Clone)]
pub struct FixedBitSet {
    bits: Vec<u64>,
}

impl FixedBitSet {
    const BLOCK_SIZE: usize = 64;
    const ALL: u64 = u64::MAX;
    pub fn new(n: usize) -> Self {
        Self { bits: vec![0; (n + Self::BLOCK_SIZE - 1) / Self::BLOCK_SIZE] }
    }
    pub fn has(&self, n: usize) -> bool {
        (self.bits[n / Self::BLOCK_SIZE] & (1 << n % Self::BLOCK_SIZE)) != 0
    }
    pub fn set(&mut self, n: usize) {
        self.bits[n / Self::BLOCK_SIZE] |= 1 << n % Self::BLOCK_SIZE;
    }
    pub fn reset(&mut self, n: usize) {
        self.bits[n / Self::BLOCK_SIZE] &= !(1 << n % Self::BLOCK_SIZE);
    }
    pub fn flip(&mut self, n: usize) {
        self.bits[n / Self::BLOCK_SIZE] ^= 1 << n % Self::BLOCK_SIZE;
    }
    pub fn lsh(&mut self, n: usize) {
        let (shift, offset) = (n / Self::BLOCK_SIZE, n % Self::BLOCK_SIZE);
        let len = self.bits.len();
        if shift >= len {
            self.bits.fill(0);
            return;
        }
        if offset == 0 {
            for i in shift..len {
                self.bits[i] = self.bits[i - shift];
            }
        } else {
            for i in (shift + 1..len).rev() {
                self.bits[i] = self.bits[i - shift] << offset | self.bits[i - shift - 1] >> (Self::BLOCK_SIZE - offset);
            }
            self.bits[shift] = self.bits[0] << offset;
        }
        self.bits[..shift].fill(0);
    }
    // 保留前 n 位，其余清零
    pub fn keep(&mut self, n: usize) {
        let (block, offset) = (n / Self::BLOCK_SIZE, n % Self::BLOCK_SIZE);
        self.bits[block] &= !(Self::ALL << offset);
        for i in block + 1..self.bits.len() {
            self.bits[i] = 0;
        }
    }
    pub fn xor(&self, other: &Self) -> Self {
        Self { bits: self.bits.iter().zip(other.bits.iter()).map(|(&a, &b)| a ^ b).collect() }
    }
    pub fn xor_assign(&mut self, other: &Self) {
        for (a, &b) in self.bits.iter_mut().zip(other.bits.iter()) { *a ^= b; }
    }
    pub fn and(&self, other: &Self) -> Self {
        Self { bits: self.bits.iter().zip(other.bits.iter()).map(|(&a, &b)| a & b).collect() }
    }
    pub fn and_assign(&mut self, other: &Self) {
        for (a, &b) in self.bits.iter_mut().zip(other.bits.iter()) { *a &= b; }
    }
    pub fn or(&self, other: &Self) -> Self {
        Self { bits: self.bits.iter().zip(other.bits.iter()).map(|(&a, &b)| a | b).collect() }
    }
    pub fn or_assign(&mut self, other: &Self) {
        for (a, &b) in self.bits.iter_mut().zip(other.bits.iter()) { *a |= b; }
    }
}

mod debug {
    use std::fmt;
    use std::fmt::{Debug, Formatter};
    use crate::bitset::FixedBitSet;

    impl Debug for FixedBitSet {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            for i in (0..self.bits.len()).rev() {
                write!(f, "{:0128b}", self.bits[i])?;
            }
            Ok(())
        }
    }
}
