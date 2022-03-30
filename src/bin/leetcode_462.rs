//! 最少移动次数使数组元素相等 II

/// 中位数
pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
    let len = nums.len();
    nums.sort_unstable();
    let mid = nums[len / 2];
    nums.into_iter().map(|x| (x - mid).abs()).sum()
}

fn main() {
    assert_eq!(min_moves2(vec![1, 2, 3]), 2);
    assert_eq!(min_moves2(vec![1, 10, 2, 9]), 16);
}
