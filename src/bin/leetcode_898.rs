//! 子数组按位或操作

use std::collections::HashSet;

pub fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
    let mut result = HashSet::new();
    let mut max = 0;
    for &num in &arr {
        max |= num;
    }
    for (i, &x) in arr.iter().enumerate() {
        let mut num = x;
        result.insert(num);
        for &y in &arr[i + 1..] {
            num |= y;
            result.insert(num);
            if num == max {
                break;
            }
        }
    }
    result.len() as i32
}

fn main() {
    assert_eq!(subarray_bitwise_o_rs(vec![0]), 1);
    assert_eq!(subarray_bitwise_o_rs(vec![1, 1, 2]), 3);
    assert_eq!(subarray_bitwise_o_rs(vec![1, 2, 4]), 6);
}
