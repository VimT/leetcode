//! 使子数组元素和相等

use leetcode::gcd::gcd;

pub fn make_sub_k_sum_equal(arr: Vec<i32>, k: i32) -> i64 {
    let len = arr.len();
    let period = gcd(len as i32, k) as usize;
    let mut result = 0;
    let cnt = (arr.len() / period) as i64;
    for start in 0..period {
        let mut i = start;
        let mut nums = Vec::with_capacity(cnt as usize);
        while i < len {
            nums.push(arr[i]);
            i += period;
        }
        nums.sort_unstable();
        let mid = if nums.len() & 1 == 1 {
            nums[nums.len() / 2]
        } else {
            (nums[nums.len() / 2 - 1] + nums[nums.len() / 2]) / 2
        };
        for num in nums {
            result += (num - mid).abs() as i64;
        }
    }
    result
}

fn main() {
    fn test(func: fn(arr: Vec<i32>, k: i32) -> i64) {
        assert_eq!(func(vec![2, 10, 9], 1), 8);
        assert_eq!(func(vec![2, 5, 5, 7], 3), 5);
        assert_eq!(func(vec![1, 4, 1, 3], 2), 1);
    }
    test(make_sub_k_sum_equal);
}
