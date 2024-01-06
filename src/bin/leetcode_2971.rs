//! 找到最大周长的多边形

pub fn largest_perimeter(mut nums: Vec<i32>) -> i64 {
    nums.sort_unstable();
    let len = nums.len();
    let mut result = -1;
    let mut cur = 0;
    for i in 0..len {
        cur += nums[i] as i64;
        if i >= 2 && cur > nums[i] as i64 * 2 {
            result = result.max(cur);
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i64) {
        assert_eq!(func(vec![5, 5, 5]), 15);
        assert_eq!(func(vec![1, 12, 1, 2, 5, 50, 3]), 12);
        assert_eq!(func(vec![5, 5, 50]), -1);
    }
    test(largest_perimeter);
}
