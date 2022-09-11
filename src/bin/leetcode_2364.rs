//! 统计坏数对的数目

use std::collections::HashMap;

pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        *map.entry(num - i as i32).or_default() += 1;
    }
    let len = nums.len() as i64;
    let mut result = 0;
    for (i, &num) in nums.iter().enumerate() {
        let same = *map.get(&(num - i as i32)).unwrap();
        result += len - same as i64;
    }
    result / 2
}

fn main() {
    assert_eq!(count_bad_pairs(vec![4, 1, 3, 3]), 5);
    assert_eq!(count_bad_pairs(vec![1, 2, 3, 4, 5]), 0);
}
