//! 学生分数的最小差值

pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort_unstable();
    let len = nums.len();
    let mut result = i32::MAX;
    let k = k as usize;
    for i in k - 1..len {
        result = result.min(nums[i] - nums[i + 1 - k]);
    }
    result
}

fn main() {
    assert_eq!(minimum_difference(vec![90], 1), 0);
    assert_eq!(minimum_difference(vec![9, 4, 1, 7], 2), 2);
}
