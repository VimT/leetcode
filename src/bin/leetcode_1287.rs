//! 有序数组中出现次数超过25%的元素

use std::cmp::Ordering;

pub fn find_special_integer(arr: Vec<i32>) -> i32 {
    let len = arr.len();
    let span = len / 4 + 1;
    for i in (0..len).step_by(span) {
        let l = arr.binary_search_by(|mid| mid.cmp(&arr[i]).then(Ordering::Greater)).unwrap_err();
        let r = arr.binary_search_by(|mid| mid.cmp(&arr[i]).then(Ordering::Less)).unwrap_err();
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
