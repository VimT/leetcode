//! 优质数对的总数 II

use std::collections::HashMap;

pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
    let nums1: Vec<i32> = nums1.into_iter().filter_map(|x| if x % k == 0 { Some(x / k) } else { None }).collect();
    if nums1.is_empty() { return 0; }
    let mx = nums1.iter().max().copied().unwrap() as usize + 1;
    let mut cnt1 = vec![0; mx];
    for num in nums1 {
        cnt1[num as usize] += 1;
    }
    let mut result = 0;
    let mut cnt2: HashMap<i32, i32> = HashMap::new();
    for num in nums2 {
        *cnt2.entry(num).or_default() += 1;
    }
    for (num, cnt) in cnt2 {
        let mut s = 0;
        let mut j = num as usize;
        while j < mx {
            s += cnt1[j];
            j += num as usize;
        }
        result += cnt as i64 * s;
    }
    result
}

fn main() {
    fn test(func: fn(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64) {
        assert_eq!(func(vec![2, 8, 17, 6], vec![3, 1, 1, 8], 2), 7);
        assert_eq!(func(vec![1, 3, 4], vec![1, 3, 4], 1), 5);
        assert_eq!(func(vec![1, 2, 4, 12], vec![2, 4], 3), 2);
    }
    test(number_of_pairs);
}
