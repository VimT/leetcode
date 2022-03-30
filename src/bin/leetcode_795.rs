//! 区间子数组个数


/// 假设一个元素小于 L 标记为 0，位于 [L, R] 之间标记为 1，大于 R 标记为 2
/// 希望找出不包含 2 且至少包含一个 1 的子数组数量
/// 问题可以转换为先找出所有的子数组，再从中减去只包含 0 的子数组。
pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
    let count = |x: i32| -> i32 {
        let mut cur = 0;
        let mut result = 0;
        for &num in &nums {
            cur = if num <= x { cur + 1 } else { 0 };
            result += cur;
        }
        result
    };
    count(right) - count(left - 1)
}

pub fn num_subarray_bounded_max_double_point(nums: Vec<i32>, left: i32, right: i32) -> i32 {
    let mut l = 0;
    let mut ok = -1;
    let mut result = 0;
    for (i, v) in nums.into_iter().enumerate() {
        let i = i as i32;
        if v >= left && v <= right {
            ok = i;
        } else if v > right {
            l = i + 1;
            ok = -1;
        }
        if ok >= 0 { result += ok - l + 1; }
    }
    result
}

fn main() {
    assert_eq!(num_subarray_bounded_max_double_point(vec![2, 1, 4, 3], 2, 3), 3);
    assert_eq!(num_subarray_bounded_max_double_point(vec![2, 9, 2, 5, 6], 2, 8), 7);
}
