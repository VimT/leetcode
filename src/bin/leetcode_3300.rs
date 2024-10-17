//! 替换为数位和以后的最小元素

pub fn min_element(nums: Vec<i32>) -> i32 {
    nums.into_iter().map(|mut num| {
        let mut sum = 0;
        while num > 0 {
            sum += num % 10;
            num /= 10;
        }
        sum
    }).min().unwrap()
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![10, 12, 13, 14]), 1);
        assert_eq!(func(vec![1, 2, 3, 4]), 1);
        assert_eq!(func(vec![999, 19, 199]), 10);
    }
    test(min_element);
}
