//! learn from
//! https://github.com/petgraph/fixedbitset

pub trait Block: Copy {
    const U64_COUNT: usize = core::mem::size_of::<Self>() / core::mem::size_of::<u64>();
    const BITS: usize = core::mem::size_of::<Self>() * 8;
    const NONE: Self;
    const ALL: Self;
    fn is_empty(self) -> bool;
    unsafe fn as_u64_slice(&self) -> &[u64] { unsafe { core::slice::from_raw_parts(self as *const Self as *const u64, Self::U64_COUNT) } }
    fn or(self, other: Self) -> Self;
    fn and(self, other: Self) -> Self;
    fn xor(self, other: Self) -> Self;
    fn not(self) -> Self;
    fn count_ones(self) -> u32;
    fn eq(self, other: Self) -> bool;
}


#[inline]
fn div_rem(x: usize, denominator: usize) -> (usize, usize) {
    (x / denominator, x % denominator)
}

pub struct BitSetContainer<T: Block>(Vec<T>);

impl<T: Block> BitSetContainer<T> {
    pub fn from_blocks(b: Vec<u64>) -> Self {
        let mut bitset = Self::with_capacity(b.len() * 64);
        for (a, b) in bitset.as_mut_u64_slice().iter_mut().zip(b) {
            *a = b;
        }
        bitset
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self(vec![T::NONE; (capacity + T::BITS - 1) / T::BITS])
    }
    pub fn as_mut_u64_slice(&mut self) -> &mut [u64] {
        unsafe { core::slice::from_raw_parts_mut(self.0.as_mut_ptr() as *mut u64, self.0.len() * T::U64_COUNT) }
    }
    pub fn get_mut_u64(&mut self, index: usize) -> &mut u64 {
        unsafe { &mut *self.0.as_mut_ptr().cast::<u64>().add(index) }
    }
    pub fn get_u64(&self, index: usize) -> u64 {
        unsafe { *self.0.as_ptr().cast::<u64>().add(index) }
    }
    pub fn has(&self, bit: usize) -> bool {
        let (block, i) = div_rem(bit, 64);
        (self.get_u64(block) & (1 << i)) != 0
    }
    pub fn set(&mut self, bit: usize, enabled: bool) {
        let (block, i) = div_rem(bit, 64);
        if enabled {
            *self.get_mut_u64(block) |= 1 << i;
        } else {
            *self.get_mut_u64(block) &= !(1 << i);
        }
    }
    pub fn toggle(&mut self, bit: usize) {
        let (block, i) = div_rem(bit, 64);
        *self.get_mut_u64(block) ^= 1 << i;
    }
    pub fn count_ones(&self) -> u32 { self.0.iter().map(|&block| block.count_ones()).sum() }
    pub fn and(&self, other: &Self) -> Self { Self(self.0.iter().zip(&other.0).map(|(&a, &b)| a.and(b)).collect()) }
    pub fn or(&self, other: &Self) -> Self { Self(self.0.iter().zip(&other.0).map(|(&a, &b)| a.or(b)).collect()) }
    pub fn xor(&self, other: &Self) -> Self { Self(self.0.iter().zip(&other.0).map(|(&a, &b)| a.xor(b)).collect()) }
    pub fn is_empty(&self) -> bool { self.0.iter().all(|&block| block.is_empty()) }
    pub fn and_assign(&mut self, other: &Self) { for (a, b) in self.0.iter_mut().zip(other.0.iter()) { *a = a.and(*b); } }
    pub fn or_assign(&mut self, other: &Self) { for (a, b) in self.0.iter_mut().zip(other.0.iter()) { *a = a.or(*b); } }
    pub fn xor_assign(&mut self, other: &Self) { for (a, b) in self.0.iter_mut().zip(other.0.iter()) { *a = a.xor(*b); } }
}

#[cfg(not(target_arch = "x86_64"))]
mod u64 {
    use super::Block;

    impl Block for u64 {
        const NONE: Self = 0;
        const ALL: Self = !0;
        fn is_empty(self) -> bool { return self == 0; }
        fn or(self, other: Self) -> Self { self | other }
        fn and(self, other: Self) -> Self { self & other }
        fn xor(self, other: Self) -> Self { self ^ other }
        fn not(self) -> Self { !self }
        fn count_ones(self) -> u32 { self.count_ones() }
        fn eq(self, other: Self) -> bool { return self == other; }
    }

    pub type BitSet = super::BitSetContainer<u64>;
}


#[cfg(target_arch = "x86_64")]
mod avx {
    use core::arch::x86_64::*;

    use super::*;

    impl Block for __m256i {
        const NONE: Self = unsafe { std::mem::transmute([0u64; Self::U64_COUNT]) };
        const ALL: Self = unsafe { std::mem::transmute([u64::MAX; Self::U64_COUNT]) };
        fn is_empty(self) -> bool { unsafe { _mm256_testz_si256(self, self) == 1 } }
        fn or(self, other: Self) -> Self { unsafe { _mm256_or_si256(self, other) } }
        fn and(self, other: Self) -> Self { unsafe { _mm256_and_si256(self, other) } }
        fn xor(self, other: Self) -> Self { unsafe { _mm256_xor_si256(self, other) } }
        fn not(self) -> Self { unsafe { _mm256_xor_si256(self, Self::ALL) } }
        fn count_ones(self) -> u32 { unsafe { self.as_u64_slice().iter().map(|&b| b.count_ones()).sum() } }
        fn eq(self, other: Self) -> bool { unsafe { _mm256_testz_si256(self, other) == 1 } }
    }

    pub type BitSet = BitSetContainer<__m256i>;
}

pub use avx::*;

#[cfg(test)]
mod test {
    use super::avx::BitSet;

    #[test]
    fn test_bitset() {
        let len = 20;
        let mut bitset = BitSet::with_capacity(len);
        for i in 0..len {
            assert_eq!(bitset.has(i), false);
        }
        bitset.set(len / 2, true);
        for i in 0..len {
            if i == len / 2 {
                assert_eq!(bitset.has(i), true);
            } else {
                assert_eq!(bitset.has(i), false);
            }
        }
        bitset.and_assign(&BitSet::with_capacity(len));
        for i in 0..len {
            assert_eq!(bitset.has(i), false);
        }
        bitset.or_assign(&BitSet::from_blocks(vec![1]));
        assert!(bitset.has(0));
        bitset.xor_assign(&BitSet::from_blocks(vec![1]));
        assert!(!bitset.has(0));
        bitset.toggle(0);
        assert!(bitset.has(0));
        assert_eq!(bitset.count_ones(), 1);
        let c = bitset.and(&BitSet::from_blocks(vec![2]));
        assert_eq!(c.count_ones(), 0);
        let c = bitset.or(&BitSet::from_blocks(vec![2]));
        assert_eq!(c.count_ones(), 2);
        let c = bitset.xor(&BitSet::from_blocks(vec![2]));
        assert_eq!(c.count_ones(), 2);
        assert_eq!(bitset.is_empty(), false);
    }
}