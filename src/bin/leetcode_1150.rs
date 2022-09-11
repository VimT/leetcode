//! 检查一个数是否在数组中占绝大多数

pub fn is_majority_element(nums: Vec<i32>, target: i32) -> bool {
    nums.iter().filter(|x| **x == target).count() > nums.len() / 2
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, target: i32) -> bool) {
        assert_eq!(func(vec![2, 4, 5, 5, 5, 5, 5, 6, 6], 5), true);
        assert_eq!(func(vec![10, 100, 101, 101], 101), false);
    }
    test(is_majority_element);
}
