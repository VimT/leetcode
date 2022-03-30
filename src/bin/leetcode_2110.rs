//! 股票平滑下跌阶段的数目

pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
    let mut i = 0;
    let len = prices.len();
    let mut result = 0;
    while i < len {
        let mut right = i + 1;
        while right < len && prices[right] == prices[right - 1] - 1 {
            right += 1;
        }
        let sublen = right - i;
        result += (1 + sublen) * sublen / 2;
        i = right;
    }

    result as i64
}


fn main() {
    assert_eq!(get_descent_periods(vec![3, 2, 1, 4]), 7);
    assert_eq!(get_descent_periods(vec![8, 6, 7, 7]), 4);
    assert_eq!(get_descent_periods(vec![1]), 1);
}