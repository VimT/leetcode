//! 统计染色格子数

pub fn colored_cells(n: i32) -> i64 {
    let mut result = 1;
    for i in 1..n as i64 {
        result += i * 4;
    }
    result
}

fn main() {
    fn test(func: fn(n: i32) -> i64) {
        assert_eq!(func(1), 1);
        assert_eq!(func(2), 5);
    }
    test(colored_cells);
}
