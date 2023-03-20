//! 通过翻转子数组使两个数组相等

pub fn can_be_equal(mut target: Vec<i32>, mut arr: Vec<i32>) -> bool {
    target.sort_unstable();
    arr.sort_unstable();
    target == arr
}

fn main() {
    fn test(func: fn(target: Vec<i32>, arr: Vec<i32>) -> bool) {
        assert_eq!(func(vec![1, 2, 3, 4], vec![2, 4, 1, 3]), true);
        assert_eq!(func(vec![7], vec![7]), true);
        assert_eq!(func(vec![3, 7, 9], vec![3, 7, 11]), false);
    }
    test(can_be_equal);
}
