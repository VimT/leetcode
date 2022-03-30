//! 按要求补齐数组

/// 如果[1,x-1]所有数字被覆盖，且 x 在数组中，则[1,2x-1]所有数字会被覆盖
pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
    let mut result = 0;
    let mut i = 0;
    let len = nums.len();
    let mut x = 1i64;
    // 找未被数组覆盖的最小整数x
    while x <= n as i64 {
        if i < len && nums[i] as i64 <= x {
            x += nums[i] as i64;
            i += 1;
        } else {
            x *= 2;
            result += 1;
        }
    }
    result
}

fn main() {
    assert_eq!(min_patches(vec![1, 3], 6), 1);
    assert_eq!(min_patches(vec![1, 5, 10], 20), 2);
    assert_eq!(min_patches(vec![1, 2, 2], 5), 0);
    assert_eq!(min_patches(vec![1, 2, 31, 33], 2147483647), 28);
}
