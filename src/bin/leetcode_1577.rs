//! 数的平方等于两数乘积的方法数

use std::collections::HashMap;

pub fn num_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    fn check(nums1: &Vec<i32>, nums2: &Vec<i32>) -> i32 {
        let mut result = 0;
        let mut cnt: HashMap<i64, i32> = HashMap::new();
        for &num in nums2 {
            *cnt.entry(num as i64).or_default() += 1;
        }
        for &num in nums1 {
            let num = num as i64;
            let x = num * num;
            for (&k, &v) in &cnt {
                if x % k == 0 {
                    if k == num { result += v * (v - 1); } else {
                        result += v * cnt.get(&(x / k)).copied().unwrap_or(0);
                    }
                }
            }
        }
        result / 2
    }
    check(&nums1, &nums2) + check(&nums2, &nums1)
}

fn main() {
    fn test(func: fn(nums1: Vec<i32>, nums2: Vec<i32>) -> i32) {
        assert_eq!(func(vec![7, 4], vec![5, 2, 8, 9]), 1);
        assert_eq!(func(vec![1, 1], vec![1, 1, 1]), 9);
        assert_eq!(func(vec![7, 7, 8, 3], vec![1, 2, 9, 7]), 2);
        assert_eq!(func(vec![4, 7, 9, 11, 23], vec![3, 5, 1024, 12, 18]), 0);
    }
    test(num_triplets);
}
