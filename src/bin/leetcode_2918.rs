//! 数组的最小相等和

pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
    fn cal(nums: Vec<i32>) -> (bool, i64) {
        let mut has_zero = false;
        let mut min_sum = 0;
        for mut num in nums {
            if num == 0 {
                has_zero = true;
                num += 1;
            }
            min_sum += num as i64;
        }
        (has_zero, min_sum)
    }
    let (has_zero1, min_sum1) = cal(nums1);
    let (has_zero2, min_sum2) = cal(nums2);
    if (min_sum1 < min_sum2 && !has_zero1) || (min_sum1 > min_sum2 && !has_zero2) {
        return -1;
    }
    min_sum1.max(min_sum2)
}

fn main() {
    fn test(func: fn(nums1: Vec<i32>, nums2: Vec<i32>) -> i64) {
        assert_eq!(func(vec![3, 2, 0, 1, 0], vec![6, 5, 0]), 12);
        assert_eq!(func(vec![2, 0, 2, 0], vec![1, 4]), -1);
    }
    test(min_sum);
}
