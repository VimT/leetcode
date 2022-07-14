//! 表示一个折线图的最少线段数

pub fn minimum_lines(mut stock_prices: Vec<Vec<i32>>) -> i32 {
    if stock_prices.len() == 1 { return 0; }
    stock_prices.sort_unstable();
    let mut cnt = 1;
    for p in stock_prices.windows(3) {
        if (p[2][1] - p[1][1]) as i64 * (p[1][0] - p[0][0]) as i64 != (p[1][1] - p[0][1]) as i64 * (p[2][0] - p[1][0]) as i64 {
            cnt += 1;
        }
    }
    cnt
}

fn main() {
    fn test(func: fn(stock_prices: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![1, 7], vec![2, 6], vec![3, 5], vec![4, 4], vec![5, 4], vec![6, 3], vec![7, 2], vec![8, 1]]), 3);
        assert_eq!(func(vec![vec![3, 4], vec![1, 2], vec![7, 8], vec![2, 3]]), 1);
    }
    test(minimum_lines);
}
