//! 分割数组中数字的数位

pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    for mut num in nums {
        let start = result.len();
        while num > 0 {
            result.push(num % 10);
            num /= 10;
        }
        result[start..].reverse();
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> Vec<i32>) {
        assert_eq!(func(vec![13, 25, 83, 77]), vec![1, 3, 2, 5, 8, 3, 7, 7]);
        assert_eq!(func(vec![7, 1, 3, 9]), vec![7, 1, 3, 9]);
    }
    test(separate_digits);
}
