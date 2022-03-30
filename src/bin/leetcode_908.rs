//! 最小差值 I

pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
    let mut min = nums[0];
    let mut max = nums[0];
    for num in nums {
        min = min.min(num);
        max = max.max(num);
    }
    return (max - min - 2 * k).max(0);
}


fn main() {
    assert_eq!(smallest_range_i(vec![1], 0), 0);
    assert_eq!(smallest_range_i(vec![0, 10], 2), 6);
    assert_eq!(smallest_range_i(vec![1, 3, 6], 3), 0);
}
