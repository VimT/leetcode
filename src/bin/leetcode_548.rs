//! 将数组分割成和相等的子数组

use std::collections::HashMap;

pub fn split_array(nums: Vec<i32>) -> bool {
    let len = nums.len();
    if len < 7 { return false; }
    let mut presum = vec![0; len + 1];
    let mut num_map: HashMap<i32, Vec<usize>> = HashMap::new();
    for i in 0..len {
        if i < len - 1 {
            num_map.entry(nums[i]).or_default().push(i);
        }
        presum[i + 1] = presum[i] + nums[i];
    }
    for i in 1..=len - 6 {
        let sum = presum[i];
        for j in i + 2..=len - 4 {
            if presum[j] - presum[i + 1] == sum {
                let k_num = presum[len] - presum[j + 1] - sum * 2;
                if let Some(k_list) = num_map.get(&k_num) {
                    for &k in k_list.iter().rev() {
                        if k <= j + 1 { break; }
                        if presum[len] - presum[k + 1] == sum {
                            // println!("{},{},{}", i, j, k);
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> bool) {
        assert_eq!(func(vec![3, 1, -4, 9, 0, -9, -6, 4, -9, 10, 0, -4, 5, 0]), false);
        assert_eq!(func(vec![1, 2, 1, 3, 0, 0, 2, 2, 1, 3, 3]), true);
        assert_eq!(func(vec![0, -3, 10, -10, -8, -7, 5, -7, 5, -3]), false);
        assert_eq!(func(vec![1, 2, 1, 2, 1, 2, 1]), true);
        assert_eq!(func(vec![1, 2, 1, 2, 1, 2, 1, 2]), false);
    }
    test(split_array);
}
