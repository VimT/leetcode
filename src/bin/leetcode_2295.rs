//! 替换数组中的元素

use std::collections::HashMap;

pub fn array_change(nums: Vec<i32>, operations: Vec<Vec<i32>>) -> Vec<i32> {
    let mut map = HashMap::with_capacity(nums.len());
    for (i, &num) in nums.iter().enumerate() {
        map.insert(num, i);
    }
    for op in operations {
        let idx = map.remove(&op[0]).unwrap();
        map.insert(op[1], idx);
    }
    let mut result = vec![0; nums.len()];
    for (num, idx) in map {
        result[idx] = num;
    }
    result
}

fn main() {
    assert_eq!(array_change(vec![1, 2, 4, 6], vec![vec![1, 3], vec![4, 7], vec![6, 1]]), vec![3, 2, 7, 1]);
    assert_eq!(array_change(vec![1, 2], vec![vec![1, 3], vec![2, 1], vec![3, 2]]), vec![2, 1]);
}
