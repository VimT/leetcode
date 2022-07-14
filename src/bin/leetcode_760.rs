//! 找出变位映射

use std::collections::HashMap;

pub fn anagram_mappings(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
    for (i, num) in nums2.into_iter().enumerate() {
        map.entry(num).or_default().push(i);
    }
    nums1.into_iter().map(|x| {
        map.get_mut(&x).unwrap().pop().unwrap() as i32
    }).collect()
}

fn main() {
    fn test(func: fn(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32>) {
        assert_eq!(func(vec![12, 28, 46, 32, 50], vec![50, 12, 32, 46, 28]), vec![1, 4, 3, 2, 0]);
        assert_eq!(func(vec![84, 46], vec![84, 46]), vec![0, 1]);
    }
    test(anagram_mappings);
}
