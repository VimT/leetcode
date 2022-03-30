//! 全局倒置与局部倒置

pub fn is_ideal_permutation(nums: Vec<i32>) -> bool {
    let len = nums.len();
    let mut min = len as i32;
    for i in (2..len).rev() {
        min = min.min(nums[i]);
        if nums[i - 2] > min { return false; }
    }
    true
}

pub fn is_ideal_permutation_math(nums: Vec<i32>) -> bool {
    nums.into_iter().enumerate().find(|&(i, v)| (v - i as i32).abs() > 1).is_none()
}

fn main() {
    assert_eq!(is_ideal_permutation_math(vec![0, 2, 3, 1]), false);
    assert_eq!(is_ideal_permutation_math(vec![1, 2, 0]), false);
    assert_eq!(is_ideal_permutation_math(vec![1, 0]), true);
    assert_eq!(is_ideal_permutation_math(vec![1, 0, 2]), true);
}
