//! 随机翻转矩阵


use std::collections::{HashMap, HashSet};

use rand::{Rng, thread_rng};
use rand::rngs::ThreadRng;

struct Solution {
    m: i32,
    n: i32,
    total: i32,
    map: HashMap<i32, i32>,
    rng: ThreadRng,
}


impl Solution {
    fn new(m: i32, n: i32) -> Self {
        Solution { m, n, total: m * n, map: HashMap::new(), rng: thread_rng() }
    }

    fn flip(&mut self) -> Vec<i32> {
        // 数组映射, [0, self.total) is 0, [self.total, m*n) is 1
        let x = self.rng.gen_range(0, self.total);
        self.total -= 1;
        let idx = *self.map.get(&x).unwrap_or(&x);
        // set 1
        self.map.insert(x, *self.map.get(&self.total).unwrap_or(&self.total));
        vec![idx / self.n, idx % self.n]
    }

    fn reset(&mut self) {
        self.total = self.m * self.n;
        self.map.clear();
    }
}

struct Solution2 {
    m: i32,
    n: i32,
    total: i32,
    bucket_size: i32,
    buckets: Vec<HashSet<i32>>,
    rng: ThreadRng,
}


impl Solution2 {
    fn new(m: i32, n: i32) -> Self {
        let total = m * n;
        let bucket_size = ((total) as f64).sqrt() as i32;
        Solution2 {
            m,
            n,
            total,
            bucket_size,
            buckets: vec![HashSet::new(); (total / bucket_size) as usize],
            rng: thread_rng(),
        }
    }

    fn flip(&mut self) -> Vec<i32> {
        let x = self.rng.gen_range(0, self.total);
        self.total -= 1;
        let mut sum_zero = 0;
        let mut curr = 0;
        for bucket in &mut self.buckets {
            if sum_zero + self.bucket_size - bucket.len() as i32 > x {
                for j in 0..self.bucket_size {
                    if !bucket.contains(&(curr + j)) {
                        if sum_zero == x {
                            bucket.insert(curr + j);
                            return vec![(curr + j) / self.n, (curr + j) % self.n];
                        }
                        sum_zero += 1;
                    }
                }
            }
            curr += self.bucket_size;
            sum_zero += self.bucket_size - bucket.len() as i32;
        }
        vec![]
    }

    fn reset(&mut self) {
        self.total = self.m * self.n;
        for bucket in &mut self.buckets {
            bucket.clear();
        }
    }
}


fn main() {
    let mut solution = Solution::new(3, 1);
    println!("{:?}", solution.flip());  // 返回 [1, 0]，此时返回 [0,0]、[1,0] 和 [2,0] 的概率应当相同
    println!("{:?}", solution.flip());  // 返回 [2, 0]，因为 [1,0] 已经返回过了，此时返回 [2,0] 和 [0,0] 的概率应当相同
    println!("{:?}", solution.flip());  // 返回 [0, 0]，根据前面已经返回过的下标，此时只能返回 [0,0]
    solution.reset(); // 所有值都重置为 0 ，并可以再次选择下标返回
    println!("{:?}", solution.flip());  // 返回 [2, 0]，此时返回 [0,0]、[1,0] 和 [2,0] 的概率应当相同

    let mut solution = Solution2::new(3, 1);
    println!("{:?}", solution.flip());  // 返回 [1, 0]，此时返回 [0,0]、[1,0] 和 [2,0] 的概率应当相同
    println!("{:?}", solution.flip());  // 返回 [2, 0]，因为 [1,0] 已经返回过了，此时返回 [2,0] 和 [0,0] 的概率应当相同
    println!("{:?}", solution.flip());  // 返回 [0, 0]，根据前面已经返回过的下标，此时只能返回 [0,0]
    solution.reset(); // 所有值都重置为 0 ，并可以再次选择下标返回
    println!("{:?}", solution.flip());  // 返回 [2, 0]，此时返回 [0,0]、[1,0] 和 [2,0] 的概率应当相同
}
