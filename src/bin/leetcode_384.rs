//! 打乱数组

use std::collections::HashMap;

use rand::{Rng, thread_rng};
use rand::prelude::ThreadRng;

struct Solution {
    nums: Vec<i32>,
    origin: Vec<i32>,
    rng: ThreadRng,
}

impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Solution { nums: nums.clone(), origin: nums, rng: (thread_rng()) }
    }

    fn reset(&mut self) -> Vec<i32> {
        self.nums = self.origin.clone();
        self.nums.clone()
    }

    fn shuffle(&mut self) -> Vec<i32> {
        let len = self.nums.len();
        for i in 0..len {
            self.nums.swap(i, self.rng.gen_range(i, len));
        }
        self.nums.clone()
    }
}

fn main() {
    let nums = vec![1, 2, 3];
    let mut obj = Solution::new(nums);
    let mut count = HashMap::new();
    for _ in 0..1000 {
        *count.entry(obj.shuffle()).or_insert(0_i32) += 1;
    }
    for (k, v) in count {
        println!("{:?}: {}", k, v);
    }
    obj.reset();
}
