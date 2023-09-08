//! 奇妙序列

macro_rules! test {
    ($t: ty) => {
        let mut fancy = <$t>::new();
        fancy.append(2);   // 奇妙序列：[2]
        fancy.add_all(3);   // 奇妙序列：[2+3] -> [5]
        fancy.append(7);   // 奇妙序列：[5, 7]
        fancy.mult_all(2);  // 奇妙序列：[5*2, 7*2] -> [10, 14]
        assert_eq!(fancy.get_index(0), 10); // 返回 10
        fancy.add_all(3);   // 奇妙序列：[10+3, 14+3] -> [13, 17]
        fancy.append(10);  // 奇妙序列：[13, 17, 10]
        fancy.mult_all(2);  // 奇妙序列：[13*2, 17*2, 10*2] -> [26, 34, 20]
        assert_eq!(fancy.get_index(0), 26); // 返回 26
        assert_eq!(fancy.get_index(1), 34); // 返回 34
        assert_eq!(fancy.get_index(2), 20); // 返回 20
    };
}

mod 暴力 {
    const MOD: i64 = 1e9 as i64 + 7;

    enum Op {
        Add(i64),
        Multi(i64),
    }

    struct Fancy {
        nums: Vec<(i64, usize)>,
        op: Vec<Op>,
    }


    impl Fancy {
        fn new() -> Self {
            Fancy { nums: vec![], op: vec![] }
        }
        fn append(&mut self, val: i32) {
            self.nums.push((val as i64, self.op.len()));
        }

        fn add_all(&mut self, inc: i32) {
            self.op.push(Op::Add(inc as i64));
        }

        fn mult_all(&mut self, m: i32) {
            self.op.push(Op::Multi(m as i64));
        }

        fn get_index(&mut self, idx: i32) -> i32 {
            if idx as usize >= self.nums.len() { return -1; }
            let (mut num, mut i) = self.nums[idx as usize];
            while i < self.op.len() {
                match self.op[i] {
                    Op::Add(v) => num += v,
                    Op::Multi(v) => num *= v,
                }
                num %= MOD;
                i += 1;
            }
            self.nums[idx as usize] = (num, i);
            num as i32
        }
    }

    pub fn test() {
        test!(Fancy);
    }
}


mod 逆元 {
    use leetcode::algorithm::mod_inv;

    const MOD: i64 = 1e9 as i64 + 7;

    /// 转化成 ax + b 的形式，对于add和multi，更新a和b。对于后面append的新数，用逆元清除前面的
    struct Fancy {
        a: i64,
        b: i64,
        nums: Vec<i64>,
    }

    impl Fancy {
        fn new() -> Self {
            Fancy { a: 1, b: 0, nums: vec![] }
        }
        // push ( val - b ) / a，用逆元替代除法
        fn append(&mut self, val: i32) {
            self.nums.push((val as i64 - self.b + MOD) % MOD * mod_inv(self.a, MOD) % MOD);
        }

        fn add_all(&mut self, inc: i32) {
            self.b = (self.b + inc as i64) % MOD;
        }

        fn mult_all(&mut self, m: i32) {
            self.a = (self.a * m as i64) % MOD;
            self.b = (self.b * m as i64) % MOD;
        }


        fn get_index(&mut self, idx: i32) -> i32 {
            let i = idx as usize;
            if i >= self.nums.len() { return -1; }
            ((self.a * self.nums[i] % MOD + self.b) % MOD) as i32
        }
    }

    pub fn test() {
        test!(Fancy);
    }
}

fn main() {
    暴力::test();
    逆元::test();
}
