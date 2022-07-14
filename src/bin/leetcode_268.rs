//! 丢失的数字

pub fn missing_number(nums: Vec<i32>) -> i32 {
    return (nums.len() * (nums.len() + 1) / 2) as i32 - nums.into_iter().sum::<i32>();
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![3, 0, 1]), 2);
        assert_eq!(func(vec![0, 1]), 2);
        assert_eq!(func(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    }
    test(missing_number);
}
