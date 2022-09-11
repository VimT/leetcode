//! 有序数组中出现次数超过25%的元素

use leetcode::algorithm::{binary_search_left, binary_search_right};

pub fn find_special_integer(arr: Vec<i32>) -> i32 {
    let len = arr.len();
    let span = len / 4 + 1;
    for i in (0..len).step_by(span) {
        let l = binary_search_left(&arr, arr[i]);
        let r = binary_search_right(&arr, arr[i]);
        if r - l >= span {
            return arr[i];
        }
    }
    -1
}

fn main() {
    fn test(func: fn(arr: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 2, 2, 6, 6, 6, 6, 7, 10]), 6);
        assert_eq!(func(vec![1, 1]), 1);
    }
    test(find_special_integer);
}
