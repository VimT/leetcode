//! 最长公共前缀的长度

pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {

}

fn main() {
    fn test(func: fn(arr1: Vec<i32>, arr2: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 10, 100], vec![1000]), 3);
        assert_eq!(func(vec![1, 2, 3], vec![4, 4, 4]), 0);
    }
    test(longest_common_prefix);
}
