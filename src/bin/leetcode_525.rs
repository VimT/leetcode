//! 连续数组

use std::collections::HashMap;

pub fn find_max_length(nums: Vec<i32>) -> i32 {
    let mut m = HashMap::new();
    let len = nums.len();
    let mut cur_sum = 0;
    m.insert(0, 0);
    let mut result = 0;
    for i in 0..len {
        if nums[i] == 0 {
            cur_sum -= 1;
        } else {
            cur_sum += 1;
        }
        if let Some(pre) = m.get(&cur_sum) {
            result = result.max(i + 1 - pre);
        } else {
            m.insert(cur_sum, i + 1);
        }
    }
    result as i32
}

fn main() {
    assert_eq!(find_max_length(vec![0, 1]), 2);
    assert_eq!(find_max_length(vec![0, 1, 0]), 2);
    assert_eq!(find_max_length(vec![0, 1, 1, 1, 1, 0, 0, 0, 0]), 8);
}
