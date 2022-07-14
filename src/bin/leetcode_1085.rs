//! 最小元素各数位之和

pub fn sum_of_digits(nums: Vec<i32>) -> i32 {
    let mut min = *nums.iter().min().unwrap();
    let mut sum = 0;
    while min > 0 {
        sum += min % 10;
        min /= 10;
    }
    sum & 1 ^ 1
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![34, 23, 1, 24, 75, 33, 54, 8]), 0);
        assert_eq!(func(vec![99, 77, 33, 66, 55]), 1);
    }
    test(sum_of_digits);
}
