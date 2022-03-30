//! 将数组和减半的最少操作次数

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialEq, PartialOrd, Copy, Clone)]
struct F64(f64);

impl Eq for F64 {}

impl Ord for F64 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn halve_array(nums: Vec<i32>) -> i32 {
    let nums: Vec<f64> = nums.iter().map(|x| *x as f64).collect();
    let sum: f64 = nums.iter().sum();
    let mut heap: BinaryHeap<F64> = nums.iter().map(|x| F64(*x)).collect();
    let mut sub = 0.;
    let mut result = 0;
    while sub < sum / 2. {
        let F64(num) = heap.pop().unwrap();
        sub += num / 2.;
        heap.push(F64(num / 2.));
        result += 1;
    }
    result
}

fn main() {
    assert_eq!(halve_array(vec![5, 19, 8, 1]), 3);
    assert_eq!(halve_array(vec![3, 8, 20]), 3);
}
