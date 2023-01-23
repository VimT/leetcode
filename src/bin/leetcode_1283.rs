//! 使结果不超过阈值的最小除数

pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
    let mut left = 1;
    let mut right = *nums.iter().max().unwrap() + 1;
    while left < right {
        let mid = (left + right) / 2;
        // 除法向上取整
        let sum = nums.iter().map(|&x| ((x + mid - 1) / mid) as i64).sum::<i64>();
        if sum <= threshold as i64 {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, threshold: i32) -> i32) {
        assert_eq!(func(vec![1, 2, 5, 9], 6), 5);
        assert_eq!(func(vec![44, 22, 33, 11, 1], 5), 44);
    }
    test(smallest_divisor);
}
