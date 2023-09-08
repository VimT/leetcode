//! 特殊元素平方和

pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    let len = nums.len() as i32;
    for (num, i) in nums.into_iter().zip(1..) {
        if len % i == 0 { result += num * num; }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 2, 3, 4]), 21);
        assert_eq!(func(vec![2, 7, 1, 19, 18, 3]), 63);
    }
    test(sum_of_squares);
}
