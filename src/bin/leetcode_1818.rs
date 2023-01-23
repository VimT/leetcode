//! 绝对差值和

pub fn min_absolute_sum_diff(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let len = nums1.len();
    let mut result = 0;
    let mut s1 = nums1.clone();
    s1.sort_unstable();
    let mut minus = 0;
    for i in 0..len {
        let abs = (nums1[i] - nums2[i]).abs();
        result += abs as i64;
        let j = s1.binary_search(&nums2[i]).unwrap_or_else(|x| x);
        if j < len {
            minus = minus.max(abs - (s1[j] - nums2[i]));
        }
        if j > 0 {
            minus = minus.max(abs - (nums2[i] - s1[j - 1]));
        }
    }

    ((result - minus as i64) % (1e9 as i64 + 7)) as i32
}

fn main() {
    fn test(func: fn(nums1: Vec<i32>, nums2: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 7, 5], vec![2, 3, 5]), 3);
        assert_eq!(func(vec![2, 4, 6, 8, 10], vec![2, 4, 6, 8, 10]), 0);
        assert_eq!(func(vec![1, 10, 4, 4, 2, 7], vec![9, 3, 5, 1, 7, 4]), 20);
    }
    test(min_absolute_sum_diff);
}
