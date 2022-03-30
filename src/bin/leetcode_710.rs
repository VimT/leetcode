//! 黑名单中的随机数

use std::collections::{HashMap, HashSet};

use rand::Rng;
use rand::rngs::OsRng;

/// 黑名单映射,把所有小于 N - len(B) 且在黑名单中数一一映射到大于等于 N - len(B) 且出现在白名单中的数
struct Solution {
    m: HashMap<i32, i32>,
    wlen: i32,
}


impl Solution {
    fn new(n: i32, blacklist: Vec<i32>) -> Self {
        let wlen = n - blacklist.len() as i32;
        let mut set = HashSet::new();
        for i in wlen..n {
            set.insert(i);
        }
        for x in &blacklist {
            set.remove(x);
        }
        let mut m = HashMap::with_capacity(blacklist.len());
        let mut iter = set.into_iter();
        for x in blacklist {
            if x < wlen {
                m.insert(x, iter.next().unwrap());
            }
        }
        Self { m, wlen }
    }

    fn pick(&self) -> i32 {
        let k = OsRng.gen_range(0, self.wlen);
        return *self.m.get(&k).unwrap_or(&k);
    }
}


fn main() {
    let s = Solution::new(6, vec![2, 3, 4]);
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    for _ in 0..1000 {
        let num = s.pick();
        *cnt.entry(num).or_default() += 1;
    }
    for (k, v) in cnt {
        println!("{}: {}", k, v);
    }
}
