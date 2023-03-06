//! 求出最多标记下标

/// 贪心：前半部分匹配后半部分
pub fn max_num_of_marked_indices(mut nums: Vec<i32>) -> i32 {
    let len = nums.len();
    nums.sort_unstable();
    let mut i = 0;
    for &x in &nums[(len + 1) / 2..] {
        if nums[i] * 2 <= x {
            i += 1;
        }
    }
    i as i32 * 2
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![3, 5, 2, 4]), 2);
        assert_eq!(func(vec![9, 2, 5, 4]), 4);
        assert_eq!(func(vec![7, 6, 8]), 0);
    }
    test(max_num_of_marked_indices);
}
