//! 对角线上的质数

fn is_prime(num: i32) -> bool {
    if num == 1 { return false; }
    let mut x = 2;
    while x * x <= num {
        if num % x == 0 {
            return false;
        }
        x += 1;
    }
    true
}

pub fn diagonal_prime(nums: Vec<Vec<i32>>) -> i32 {
    let len = nums.len();
    let mut max = 0;
    for i in 0..len {
        if is_prime(nums[i][i]) {
            max = max.max(nums[i][i]);
        }
        if is_prime(nums[i][len - 1 - i]) {
            max = max.max(nums[i][len - 1 - i]);
        }
    }
    max
}

fn main() {
    fn test(func: fn(nums: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![1, 2, 3], vec![5, 6, 7], vec![9, 10, 11]]), 11);
        assert_eq!(func(vec![vec![1, 2, 3], vec![5, 17, 7], vec![9, 11, 10]]), 17);
    }
    test(diagonal_prime);
}
