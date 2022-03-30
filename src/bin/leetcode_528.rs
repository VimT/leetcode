//! 按权重随机选择

use rand::Rng;

struct Solution {
    w: Vec<i32>,
    sum: i32,
    rng: rand::prelude::ThreadRng,
}


impl Solution {
    fn new(w: Vec<i32>) -> Self {
        let mut presum = vec![0; w.len()];
        let mut sum = 0;
        for i in 0..w.len() {
            sum += w[i];
            presum[i] = sum;
        }
        Solution { w: presum, sum, rng: rand::thread_rng() }
    }

    fn pick_index(&mut self) -> i32 {
        let rand = self.rng.gen_range(1, self.sum + 1);
        return self.w.binary_search(&rand).unwrap_or_else(|x| x) as i32;
    }
}


fn main() {
    let mut solution = Solution::new(vec![1, 3]);
    println!("{}", solution.pick_index()); // 返回 1，返回下标 1，返回该下标概率为 3/4 。
    println!("{}", solution.pick_index()); // 返回 1
    println!("{}", solution.pick_index()); // 返回 1
    println!("{}", solution.pick_index()); // 返回 1
    println!("{}", solution.pick_index()); // 返回 0，返回下标 0，返回该下标概率为 1/4 。
}
