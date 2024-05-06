//! 执行操作使数据元素之和大于等于 K

pub fn min_operations(k: i32) -> i32 {
    let mut result = i32::MAX;
    for add in 0..=k {
        result = result.min(add + (k + add) / (add + 1) - 1);
    }
    result
}

fn main() {
    fn test(func: fn(k: i32) -> i32) {
        assert_eq!(func(11), 5);
        assert_eq!(func(1), 0);
    }
    test(min_operations);
}
