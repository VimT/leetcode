//! 满足条件的子序列数目

use leetcode::algorithm::quick_pow;

pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
    nums.sort_unstable();
    let len = nums.len();
    let mut result = 0;
    const MOD: i64 = 1e9 as i64 + 7;
    let mut j = len;
    for i in 0..len {
        while i < j && nums[i] + nums[j - 1] > target { j -= 1; }
        if i >= j { break; }
        result += quick_pow(2, (j - i - 1) as i64, MOD);
        result %= MOD;
    }
    result as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, target: i32) -> i32) {
        assert_eq!(func(vec![3, 5, 6, 7], 9), 4);
        assert_eq!(func(vec![3, 3, 6, 8], 10), 6);
        assert_eq!(func(vec![2, 3, 3, 4, 6, 7], 12), 61);
    }
    test(num_subseq);
}
