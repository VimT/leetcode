//! 长度为 K 的子数组的能量值 II

pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let len = nums.len();
    let k = k as usize;
    let mut result = vec![-1; len + 1 - k];
    let mut cnt = 0;
    for (i, &x) in nums.iter().enumerate() {
        cnt = if i == 0 || x == nums[i - 1] + 1 { cnt + 1 } else { 1 };
        if cnt >= k {
            result[i + 1 - k] = x;
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> Vec<i32>) {
        assert_eq!(func(vec![1, 7, 8], 2), vec![-1, 8]);
        assert_eq!(func(vec![1, 2, 3, 4, 3, 2, 5], 3), vec![3, 4, -1, -1, -1]);
        assert_eq!(func(vec![2, 2, 2, 2, 2], 4), vec![-1, -1]);
        assert_eq!(func(vec![3, 2, 3, 2, 3, 2], 2), vec![-1, 3, -1, 3, -1]);
    }
    test(results_array);
}
