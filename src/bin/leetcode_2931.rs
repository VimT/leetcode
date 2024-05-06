//! 购买物品的最大开销

pub fn max_spending(values: Vec<Vec<i32>>) -> i64 {
    let mut values: Vec<i32> = values.into_iter().flatten().collect();
    values.sort_unstable();
    let mut result = 0;
    for i in 0..values.len() {
        result += (i + 1) * values[i] as usize;
    }
    result as i64
}

fn main() {
    fn test(func: fn(values: Vec<Vec<i32>>) -> i64) {
        assert_eq!(func(vec![vec![8, 5, 2], vec![6, 4, 1], vec![9, 7, 3]]), 285);
        assert_eq!(func(vec![vec![10, 8, 6, 4, 2], vec![9, 7, 5, 3, 2]]), 386);
    }
    test(max_spending);
}
