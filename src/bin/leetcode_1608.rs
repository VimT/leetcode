//! 特殊数组的特征值

use std::cmp::Ordering;

pub fn special_array(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let len = nums.len();
    for x in 1..=len as i32 {
        let idx = nums.binary_search_by(|num| num.cmp(&x).then(Ordering::Greater)).unwrap_err();
        if x == (len - idx) as i32 {
            return x;
        }
    }
    -1
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![3, 5]), 2);
        assert_eq!(func(vec![0, 0]), -1);
        assert_eq!(func(vec![0, 4, 3, 0, 4]), 3);
        assert_eq!(func(vec![3, 6, 7, 7, 0]), -1);
    }
    test(special_array);
}
