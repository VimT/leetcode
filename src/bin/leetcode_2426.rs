//! 满足不等式的数对数目

use std::collections::HashMap;

struct BitTree {
    tree: Vec<i64>,
}

impl BitTree {
    fn new(len: usize) -> Self {
        Self { tree: vec![0; len] }
    }

    fn add(&mut self, x: i32) {
        let mut x = x + 1;
        while x < self.tree.len() as i32 {
            self.tree[x as usize] += 1;
            x += x & -x
        }
    }

    fn query(&self, x: i32) -> i64 {
        let mut result = 0;
        let mut x = x + 1;
        while x > 0 {
            result += self.tree[x as usize];
            x &= x - 1;
        }
        result
    }
}

/// 记 a[i]=num1[i] - nums2[i], 则题目要求 a[i] <= a[j] + diff
/// 树状数组
pub fn number_of_pairs(mut nums1: Vec<i32>, nums2: Vec<i32>, diff: i32) -> i64 {
    let len = nums1.len();
    let mut tmp = Vec::with_capacity(len * 2);
    for i in 0..len {
        tmp.push(nums1[i] - nums2[i]);
        tmp.push(nums1[i] - nums2[i] + diff);
        nums1[i] -= nums2[i];
    }
    tmp.sort_unstable(); // 离散化
    let mut m = HashMap::new();
    for v in tmp.into_iter() {
        let ml = m.len();
        m.entry(v).or_insert(ml);
    }
    let mut result = 0;
    let mut t = BitTree::new(m.len() + 1);
    for x in nums1 {
        result += t.query(m[&(x + diff)] as i32);
        t.add(m[&x] as i32);
    }
    result
}

fn main() {
    fn test(func: fn(nums1: Vec<i32>, nums2: Vec<i32>, diff: i32) -> i64) {
        assert_eq!(func(vec![3, 2, 5], vec![2, 2, 1], 1), 3);
        assert_eq!(func(vec![3, -1], vec![-2, 2], -1), 0);
    }
    test(number_of_pairs);
}
