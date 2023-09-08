//! 最大公约数遍历


use leetcode::algorithm::cal_lpf;
use leetcode::union_find::UnionFind;

/// 一个数的所有质因数之间有一条无向边，判断质因数组成的图是不是连通图
pub fn can_traverse_all_pairs(nums: Vec<i32>) -> bool {
    if nums.len() == 1 { return true; }
    let len = 1e5 as usize + 1;
    let mut uf = UnionFind::new(len);
    let mut seen = vec![false; len];
    for mut num in nums {
        if num == 1 { return false; }
        let mut zys = vec![];
        let mut i = 2;
        while i * i <= num {
            if num % i == 0 {
                num /= i;
                zys.push(i);
                while num % i == 0 { num /= i; }
            }
            i += 1;
        }
        if num > 1 { zys.push(num); }
        for &a in &zys {
            seen[a as usize] = true;
            for &b in &zys {
                us.union(a as usize, b as usize);
            }
        }
    }
    let count = seen.iter().filter(|x| **x).count();
    for x in 0..len {
        if seen[x] {
            return us.size(x) == count;
        }
    }
    true
}

/// 另一种求所有质因数的做法
pub fn can_traverse_all_pairs2(nums: Vec<i32>) -> bool {
    const MX: usize = 1e5 as usize;
    static mut LPF: Vec<i32> = vec![];
    unsafe {
        if LPF.is_empty() { LPF = cal_lpf(MX); }
        let mut uf = UnionFind::new(nums.len() + MX);
        let len = nums.len();
        for (i, mut num) in nums.into_iter().enumerate() {
            while num > 1 {
                let p = LPF[num as usize];
                while LPF[num as usize] == p { num /= p };
                us.union(i, p as usize + len);
            }
        }
        let root = us.find(0);
        for i in 1..len {
            if us.find(i) != root {
                return false;
            }
        }
        true
    }
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> bool) {
        assert_eq!(func(vec![28, 39]), false);
        assert_eq!(func(vec![42, 40, 45, 42, 50, 33, 30, 45, 33, 45, 30, 36, 44, 1, 21, 10, 40, 42, 42]), false);
        assert_eq!(func(vec![2, 3, 6]), true);
        assert_eq!(func(vec![3, 9, 5]), false);
        assert_eq!(func(vec![4, 3, 12, 8]), true);
    }
    test(can_traverse_all_pairs);
    test(can_traverse_all_pairs2);
}
