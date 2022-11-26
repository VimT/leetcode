//! 下一个更大元素 IV

use std::collections::BTreeMap;

pub fn second_greater_element(nums: Vec<i32>) -> Vec<i32> {
    let mut wait1: BTreeMap<i32, Vec<usize>> = BTreeMap::new();
    let mut wait2: BTreeMap<i32, Vec<usize>> = BTreeMap::new();
    let len = nums.len();
    let mut result = vec![-1; len];
    for (i, num) in nums.into_iter().enumerate() {
        let vec1: Vec<i32> = wait2.range(..num).map(|x| *x.0).collect();
        for k in vec1 {
            for j in wait2.remove(&k).unwrap() {
                result[j] = num;
            }
        }
        let vec2: Vec<i32> = wait1.range(..num).map(|x| *x.0).collect();
        for k in vec2 {
            wait2.insert(k, wait1.remove(&k).unwrap());
        }
        wait1.entry(num).or_default().push(i);
    }
    result
}

fn main() {
    assert_eq!(second_greater_element(vec![3,11,19,6,4,6,14,1,11,9]), vec![19,14,-1,11,14,11,-1,9,-1,-1]);
    assert_eq!(second_greater_element(vec![2, 4, 0, 9, 6]), vec![9, 6, 6, -1, -1]);
    assert_eq!(second_greater_element(vec![3, 3]), vec![-1, -1]);
}
