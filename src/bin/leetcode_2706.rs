//! 购买两块巧克力

pub fn buy_choco(mut prices: Vec<i32>, money: i32) -> i32 {
    prices.sort_unstable();
    let p = prices[0] + prices[1];
    return if p > money { money } else { money - p }
}

fn main() {
    fn test(func: fn(prices: Vec<i32>, money: i32) -> i32) {
        assert_eq!(func(vec![1, 2, 2], 3), 0);
        assert_eq!(func(vec![3, 2, 3], 3), 3);
    }
    test(buy_choco);
}
