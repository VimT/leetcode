//! 使数组变成交替数组的最少操作数

use std::collections::HashMap;

pub fn minimum_operations(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    if len == 1 { return 0; }
    let mut odd = HashMap::new();
    let mut even = HashMap::new();
    for i in 0..len {
        *if i & 1 == 0 { &mut even } else { &mut odd }.entry(nums[i]).or_insert(0) += 1;
    }
    let mut odd: Vec<(i32, i32)> = odd.into_iter().map(|x| (x.1, x.0)).collect();
    let mut even: Vec<(i32, i32)> = even.into_iter().map(|x| (x.1, x.0)).collect();
    odd.sort_unstable_by_key(|x| (-x.0, x.1));
    even.sort_unstable_by_key(|x| (-x.0, x.1));
    let max_len = if odd[0].1 != even[0].1 {
        odd[0].0 + even[0].0
    } else {
        odd[0].0.max(even[0].0).max(if odd.len() > 1 { odd[1].0 + even[0].0 } else { 0 }).max(if even.len() > 1 { even[1].0 + odd[0].0 } else { 0 })
    };
    len as i32 - max_len
}

fn main() {
    assert_eq!(minimum_operations(vec![1, 1, 1, 1, 1]), 2);
    assert_eq!(minimum_operations(vec![3, 1, 3, 2, 4, 3]), 3);
    assert_eq!(minimum_operations(vec![1, 2, 2, 2, 2]), 2);
}
