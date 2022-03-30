//! 设计位集

struct Bitset {
    bit: Vec<u64>,
    last_mask: u64,
    is_flip: bool,
}

const MASK: u64 = u64::MAX;

impl Bitset {
    fn new(size: i32) -> Self {
        let mut len = size / 64;
        let mut last_mask = MASK;
        if size % 64 != 0 {
            len += 1;
            last_mask = (1 << (size % 64)) - 1;
        }
        Bitset { bit: vec![0; len as usize], last_mask, is_flip: false }
    }


    fn set_one(&mut self, idx: i32) {
        let m = idx / 64;
        let n = idx % 64;
        self.bit[m as usize] = self.bit[m as usize] | (1 << n);
    }

    fn set_zero(&mut self, idx: i32) {
        let m = idx / 64;
        let n = idx % 64;
        let mask = MASK ^ (1 << n);
        self.bit[m as usize] = self.bit[m as usize] & mask;
    }


    fn fix(&mut self, idx: i32) {
        if self.is_flip { self.set_zero(idx); } else { self.set_one(idx); }
    }

    fn unfix(&mut self, idx: i32) {
        if self.is_flip { return self.set_one(idx); } else { self.set_zero(idx); }
    }

    fn flip(&mut self) {
        self.is_flip = !self.is_flip;
    }

    fn do_flip(&mut self) {
        if self.is_flip {
            let len = self.bit.len();
            for d in &mut self.bit[..len - 1] {
                *d = *d ^ MASK;
            }
            let last = self.bit.last_mut().unwrap();
            *last = *last ^ self.last_mask;
            self.is_flip = false;
        }
    }

    fn all(&mut self) -> bool {
        self.do_flip();
        for &d in &self.bit[..self.bit.len() - 1] {
            if d != MASK { return false; }
        }
        return *self.bit.last().unwrap() == self.last_mask;
    }

    fn one(&mut self) -> bool {
        self.do_flip();
        for &d in &self.bit[..self.bit.len() - 1] {
            if d > 0 { return true; }
        }
        return *self.bit.last().unwrap() > 0;
    }

    fn count(&mut self) -> i32 {
        self.do_flip();
        let mut result = 0;
        for &d in &self.bit {
            result += d.count_ones() as i32;
        }
        result
    }

    fn to_string(&mut self) -> String {
        self.do_flip();
        let mut result = Vec::with_capacity(self.bit.len() * 64);
        for &d in &self.bit[..self.bit.len() - 1] {
            for i in 0..64 {
                result.push((d >> i & 1) as u8 + b'0');
            }
        }
        for i in 0..self.last_mask.count_ones() {
            result.push((*self.bit.last().unwrap() >> i & 1) as u8 + b'0');
        }
        unsafe { String::from_utf8_unchecked(result) }
    }
}

fn main() {
    let mut bs = Bitset::new(5); // bitset = "00000".
    bs.fix(3);     // 将 idx = 3 处的值更新为 1 ，此时 bitset = "00010" 。
    bs.fix(1);     // 将 idx = 1 处的值更新为 1 ，此时 bitset = "01010" 。
    bs.flip();     // 翻转每一位上的值，此时 bitset = "10101" 。
    assert_eq!(bs.all(), false);      // 返回 False ，bitset 中的值不全为 1 。
    bs.unfix(0);   // 将 idx = 0 处的值更新为 0 ，此时 bitset = "00101" 。
    bs.flip();     // 翻转每一位上的值，此时 bitset = "11010" 。
    assert_eq!(bs.one(), true);      // 返回 True ，至少存在一位的值为 1 。
    bs.unfix(0);   // 将 idx = 0 处的值更新为 0 ，此时 bitset = "01010" 。
    assert_eq!(bs.count(), 2);    // 返回 2 ，当前有 2 位的值为 1 。
    assert_eq!(bs.to_string(), "01010"); // 返回 "01010" ，即 bitset 的当前组成情况。

    let mut bs = Bitset::new(2);
    bs.flip();
    bs.unfix(1);
    bs.all();
    bs.fix(1);
    bs.fix(1);
    bs.unfix(1);
    bs.all();
    bs.count();
    bs.to_string();
    bs.to_string();
    bs.to_string();
    bs.unfix(0);
    bs.flip();
    bs.all();
    bs.unfix(0);
    bs.one();
    bs.one();
    bs.all();
    bs.fix(0);
    bs.unfix(0);
}