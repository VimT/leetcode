//! 最小差值平方和

pub fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
    let mut diffs: Vec<i64> = nums1.into_iter().zip(nums2).map(|(a, b)| (a - b).abs() as i64).collect();
    diffs.sort_unstable();
    let mut adj = (k1 + k2) as i64;
    let len = diffs.len();
    let mut presum = vec![0; len + 1];
    for i in 0..len {
        presum[i + 1] = presum[i] + diffs[i];
    }

    // 二分找 把 >= x 的值，都修改成x，不会用完adj的x的最小值。
    let mut left = 0;
    let mut right = diffs[diffs.len() - 1];
    while left < right {
        let mid = (left + right) / 2;
        let idx = diffs.binary_search(&mid).unwrap_or_else(|x| x);
        if adj >= presum[len] - (presum[idx] + mid * (len - idx) as i64) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    if left == 0 { return 0; }
    let idx = diffs.binary_search(&left).unwrap_or_else(|x| x);
    adj -= presum[len] - (presum[idx] + left * (len - idx) as i64);
    let mut result = 0;
    // 前面的没有改
    for i in 0..idx {
        result += diffs[i] * diffs[i];
    }
    // 剩余的修改次数，可以再-1
    for _ in 0..adj {
        result += (left - 1) * (left - 1);
    }
    // 其他的都改成left
    for _ in idx + adj as usize..len {
        result += left * left;
    }
    result
}

fn main() {
    fn test(func: fn(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64) {
        assert_eq!(func(vec![10, 10, 10, 11, 5], vec![1, 0, 6, 6, 1], 11, 27), 0);
        assert_eq!(func(vec![7, 11, 4, 19, 11, 5, 6, 1, 8], vec![4, 7, 6, 16, 12, 9, 10, 2, 10], 3, 6), 27);
        assert_eq!(func(vec![1, 2, 3, 4], vec![2, 10, 20, 19], 0, 0), 579);
        assert_eq!(func(vec![1, 4, 10, 12], vec![5, 8, 6, 9], 1, 1), 43);
    }
    test(min_sum_square_diff);
}
