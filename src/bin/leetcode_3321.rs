//! 计算子数组的 x-sum II

use std::collections::{BTreeMap, BTreeSet};

#[derive(Default)]
struct Help {
    num_cnt: BTreeMap<i32, i32>,
    l: BTreeSet<(i32, i32)>, // 出现最多的 x 个元素
    r: BTreeSet<(i32, i32)>, // 其他元素
    // l 的和
    sum: i64,
}
impl Help {
    fn add(&mut self, num: i32) {
        let cnt = self.num_cnt.get(&num).copied().unwrap_or(0);
        if cnt == 0 { return; }
        let key = (cnt, num);
        if !self.l.is_empty() && key > *self.l.first().unwrap() {
            self.sum += key.0 as i64 * key.1 as i64;
            self.l.insert(key);
        } else {
            self.r.insert(key);
        }
    }
    fn remove(&mut self, num: i32) {
        let cnt = self.num_cnt.get(&num).copied().unwrap_or(0);
        if cnt == 0 { return; }
        let key = (cnt, num);
        if self.l.contains(&key) {
            self.sum -= key.0 as i64 * key.1 as i64;
            self.l.remove(&key);
        } else {
            self.r.remove(&key);
        }
    }
    fn l2r(&mut self) {
        let p = self.l.pop_first().unwrap();
        self.sum -= p.0 as i64 * p.1 as i64;
        self.r.insert(p);
    }
    fn r2l(&mut self) {
        let p = self.r.pop_last().unwrap();
        self.sum += p.0 as i64 * p.1 as i64;
        self.l.insert(p);
    }
}

pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
    let len = nums.len();
    let (k, x) = (k as usize, x as usize);
    let mut h = Help::default();
    let mut result = Vec::with_capacity(len + 1 - k);
    for i in 0..=len {
        if i >= k {
            while !h.r.is_empty() && h.l.len() < x { h.r2l() }
            while h.l.len() > x { h.l2r() }
            result.push(h.sum);
        }
        if i == len { break; }

        let num = nums[i];
        h.remove(num);
        *h.num_cnt.entry(num).or_default() += 1;
        h.add(num);
        if i >= k {
            let num = nums[i - k];
            h.remove(num);
            *h.num_cnt.get_mut(&num).unwrap() -= 1;
            h.add(num);
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32, x: i32) -> Vec<i64>) {
        assert_eq!(func(vec![1, 1, 2, 2, 3, 4, 2, 3], 6, 2), vec![6, 10, 12]);
        assert_eq!(func(vec![3, 8, 7, 8, 7, 5], 2, 2), vec![11, 15, 15, 15, 12]);
    }
    test(find_x_sum);
}
