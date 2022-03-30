//! 下一个更大元素 I


pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let len = nums2.len();
    let mut next = vec![-1; 10001];
    let mut s = vec![];
    for i in 0..len {
        while !s.is_empty() && nums2[i] > *s.last().unwrap() {
            let pop = s.pop().unwrap();
            next[pop as usize] = nums2[i];
        }
        s.push(nums2[i]);
    }
    let mut result = Vec::with_capacity(nums1.len());
    for i in nums1 {
        result.push(next[i as usize]);
    }
    result
}

fn main() {
    assert_eq!(next_greater_element(vec![1, 3, 5, 2, 4], vec![6, 5, 4, 3, 2, 1, 7]), vec![7, 7, 7, 7, 7]);
    assert_eq!(next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]), vec![-1, 3, -1]);
    assert_eq!(next_greater_element(vec![2, 4], vec![1, 2, 3, 4]), vec![3, -1]);
}
