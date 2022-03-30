//! 寻找两个有序数组的中位数

pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
    let n = nums1.len();
    let m = nums2.len();
    if n + m == 0 { return 0f64; }
    if n == 0 { return if m % 2 != 0 { nums2[m / 2] as f64 } else { (nums2[m / 2 - 1] + nums2[m / 2]) as f64 / 2f64 }; }
    if m == 0 { return if n % 2 != 0 { nums1[n / 2] as f64 } else { (nums1[n / 2 - 1] + nums1[n / 2]) as f64 / 2f64 }; }

    let mid = (n + m) / 2;
    let has_two = (n + m) % 2 == 0;
    let mut i = 0;
    let mut j = 0;
    let mut left = -1;
    let mut right = -1;
    for _ in 0..=mid {
        left = right;
        if i < n && (j >= m || nums1[i] < nums2[j]) {
            right = nums1[i];
            i += 1;
        } else {
            right = nums2[j];
            j += 1;
        }
    }
    return if has_two {
        (left + right) as f64 / 2f64
    } else {
        right as f64
    };
}

fn main() {
    assert_eq!(find_median_sorted_arrays(vec![3], vec![-2, -1]), -1f64);
    assert_eq!(find_median_sorted_arrays(vec![1, 3], vec![2]), 2f64);
    assert_eq!(find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
    assert_eq!(find_median_sorted_arrays(vec![1, 2], vec![]), 1.5);
}
