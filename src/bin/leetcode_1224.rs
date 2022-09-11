//! 最大相等频率

use std::collections::HashMap;

pub fn max_equal_freq(nums: Vec<i32>) -> i32 {
    let mut freq = HashMap::new();
    let mut count = HashMap::new();
    let mut result = 0;
    let mut max_freq = 0;
    for i in 0..nums.len() {
        let num = nums[i];
        let c = count.entry(num).or_default();
        if *c > 0 {
            *freq.entry(*c).or_default() -= 1;
        }
        *c += 1;
        max_freq = max_freq.max(*c);
        *freq.entry(*c).or_default() += 1;
        let fm: i32 = freq.get(&max_freq).cloned().unwrap_or(0);
        let fm1 = freq.get(&(max_freq - 1)).cloned().unwrap_or(0);
        let fm2 = freq.get(&1).cloned().unwrap_or(0);
        let ok = max_freq == 1 ||
            fm * max_freq + fm1 * (max_freq - 1) == (i + 1) as i32 && fm == 1 ||
            fm * max_freq + 1 == (i + 1) as i32 && fm2 == 1;
        if ok {
            result = result.max(i + 1);
        }
    }
    result as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![2, 2, 1, 1, 5, 3, 3, 5]), 7);
        assert_eq!(func(vec![1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5]), 13);
    }
    test(max_equal_freq);
}
