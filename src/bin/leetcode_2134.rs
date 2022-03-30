//! 最少交换次数来组合所有的 1 II

pub fn min_swaps(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut one_cnt = 0;
    for &num in &nums {
        if num == 1 { one_cnt += 1; }
    }
    let mut result = i32::MAX;
    let mut zero = 0;
    for i in 0..one_cnt {
        if nums[i] == 0 { zero += 1; }
    }
    for right in one_cnt..2 * len {
        result = result.min(zero);
        if nums[(right - one_cnt) % len] == 0 {
            zero -= 1;
        }
        if nums[right % len] == 0 {
            zero += 1;
        }
    }
    result.min(zero)
}

fn main() {
    assert_eq!(min_swaps(vec![0, 1, 0, 1, 1, 0, 0]), 1);
    assert_eq!(min_swaps(vec![0, 1, 1, 1, 0, 0, 1, 1, 0]), 2);
    assert_eq!(min_swaps(vec![1, 1, 0, 0, 1]), 0);
}
