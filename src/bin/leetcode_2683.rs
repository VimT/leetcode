//! 相邻值的按位异或

pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
    derived.into_iter().fold(0, |a, x| a ^ x) == 0
}

fn main() {
    fn test(func: fn(derived: Vec<i32>) -> bool) {
        assert_eq!(func(vec![1, 1, 0]), true);
        assert_eq!(func(vec![1, 1]), true);
        assert_eq!(func(vec![1, 0]), false);
    }
    test(does_valid_array_exist);
}
