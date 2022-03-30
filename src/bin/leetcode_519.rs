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
    let mut s = Solution2::new(3, 1);
    println!("{:?}", s.flip());
    println!("{:?}", s.flip());
    println!("{:?}", s.flip());
    s.reset();
    println!("{:?}", s.flip());
}

pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
    let mut count = std::collections::HashMap::new();
    for i in words1 {
        *count.entry(i).or_insert(0i32) += 1;
    }
    for i in words2 {
        *count.entry(i).or_insert(0i32) += 1000;
    }
    let mut result = 0;
    for (_, v) in count {
        if v == 1001 {
            result += 1;
        }
    }
    result
}