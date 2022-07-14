//! 较小的三数之和

/// 排序+双指针
pub fn three_sum_smaller(mut nums: Vec<i32>, target: i32) -> i32 {
    let len = nums.len();
    if len < 3 { return 0; }
    nums.sort_unstable();
    let mut result = 0;
    for i in 0..len - 2 {
        let mut left = i + 1;
        let mut right = len - 1;
        while left < right {
            if nums[i] + nums[left] + nums[right] < target {
                result += right - left;
                left += 1;
            } else {
                right -= 1;
            }
        }
    }
    result as i32
}

fn main() {
    assert_eq!(three_sum_smaller(vec![-2, 0, 1, 3], 2), 2);
    assert_eq!(three_sum_smaller(vec![], 0), 0);
    assert_eq!(three_sum_smaller(vec![0], 0), 0);
}
