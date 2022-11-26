//! 摧毁一系列目标


use std::collections::HashMap;

pub fn destroy_targets(nums: Vec<i32>, space: i32) -> i32 {
    let mut count: HashMap<i32, Vec<i32>> = HashMap::new();
    for num in nums {
        count.entry(num % space).or_default().push(num);
    }
    let mut max_len = 0;
    let mut result = i32::MAX;
    for (_, v) in count {
        let vlen = v.len();
        if vlen >= max_len {
            let min = v.into_iter().min().unwrap();
            if vlen == max_len {
                result = result.min(min);
            } else {
                result = min;
            }
            max_len = vlen;
        }
    }
    result
}

fn main() {
    assert_eq!(destroy_targets(vec![3, 7, 8, 1, 1, 5], 2), 1);
    assert_eq!(destroy_targets(vec![1, 3, 5, 2, 4, 6], 2), 1);
    assert_eq!(destroy_targets(vec![6, 2, 5], 100), 2);
}
