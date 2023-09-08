//! 几乎唯一子数组的最大和

use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub fn max_sum(nums: Vec<i32>, m: i32, k: i32) -> i64 {
    let k = k as usize;
    let m = m as usize;
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    let mut sum = 0;
    let mut result = if cnt.len() >= m { sum } else { 0 };
    for (i, &num) in nums.iter().enumerate() {
        sum += num as i64;
        *cnt.entry(num).or_default() += 1;
        if i >= k {
            sum -= nums[i - k] as i64;
            if let Entry::Occupied(mut v) = cnt.entry(nums[i - k]) {
                *v.get_mut() -= 1;
                if *v.get() == 0 { v.remove(); }
            }
        }
        if i >= k - 1 && cnt.len() >= m {
            result = result.max(sum);
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, m: i32, k: i32) -> i64) {
        assert_eq!(func(vec![1, 1, 1, 3], 2, 2), 4);
        assert_eq!(func(vec![2, 6, 7, 3, 1, 7], 3, 4), 18);
        assert_eq!(func(vec![5, 9, 9, 2, 4, 5, 4], 1, 3), 23);
        assert_eq!(func(vec![1, 2, 1, 2, 1, 2, 1], 3, 3), 0);
    }
    test(max_sum);
}
