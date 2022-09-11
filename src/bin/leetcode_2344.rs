//! 使数组可以被整除的最少删除次数

fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 { return b; }
    gcd(b % a, a)
}

pub fn min_operations(mut nums: Vec<i32>, nums_divide: Vec<i32>) -> i32 {
    let a = nums_divide.iter().fold(nums_divide[0], |a, &b| gcd(a, b));
    nums.sort_unstable();
    let mut i = 0;
    while i < nums.len() && nums[i] <= a {
        if a % nums[i] == 0 {
            return i as i32;
        }
        i += 1;
    }
    -1
}

fn main() {
    assert_eq!(min_operations(vec![2, 3, 2, 4, 3], vec![9, 6, 9, 3, 15]), 2);
    assert_eq!(min_operations(vec![4, 3, 6], vec![8, 2, 6, 10]), -1);
}