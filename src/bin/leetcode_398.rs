//! 随机数索引

use std::collections::HashMap;

use rand::{Rng, thread_rng};

struct Solution {
    m: HashMap<i32, Vec<usize>>,
}


impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        let mut m: HashMap<i32, Vec<usize>> = HashMap::new();
        for i in 0..nums.len() {
            m.entry(nums[i]).or_default().push(i);
        }
        Self { m }
    }

    fn pick(&self, target: i32) -> i32 {
        let v = self.m.get(&target).unwrap();
        let idx = thread_rng().gen_range(0, v.len());
        v[idx] as i32
    }
}


/// 或者蓄水池抽样
fn main() {
    let nums = vec![1, 2, 3, 3, 3];
    let s = Solution::new(nums);
    // pick(3) 应该返回索引 2,3 或者 4。每个索引的返回概率应该相等。
    println!("{}", s.pick(3));

    // pick(1) 应该返回 0。因为只有nums[0]等于1。
    println!("{}", s.pick(1));
}
