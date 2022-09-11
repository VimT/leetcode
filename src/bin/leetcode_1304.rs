//! 和为零的 N 个不同整数

pub fn sum_zero(n: i32) -> Vec<i32> {
    let len = n / 2;
    let mut result = Vec::with_capacity(n as usize);
    for i in -len..0 {
        result.push(i);
    }
    if n & 1 == 1 { result.push(0) }
    for i in 1..=len {
        result.push(i);
    }
    result
}

fn main() {
    fn test(func: fn(n: i32) -> Vec<i32>) {
        assert_eq!(func(5), vec![-2, -1, 0, 1, 2]);
        assert_eq!(func(3), vec![-1, 0, 1]);
        assert_eq!(func(1), vec![0]);
    }
    test(sum_zero);
}
