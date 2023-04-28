//! 商品折扣后的最终价格

pub fn final_prices(mut prices: Vec<i32>) -> Vec<i32> {
    // 单调增
    let len = prices.len();
    let mut s = vec![];
    for i in 0..len {
        while !s.is_empty() && prices[i] <= prices[*s.last().unwrap()] {
            let j = s.pop().unwrap();
            prices[j] -= prices[i];
        }
        s.push(i);
    }
    prices
}

fn main() {
    fn test(func: fn(prices: Vec<i32>) -> Vec<i32>) {
        assert_eq!(func(vec![8, 4, 6, 2, 3]), vec![4, 2, 4, 2, 3]);
        assert_eq!(func(vec![1, 2, 3, 4, 5]), vec![1, 2, 3, 4, 5]);
        assert_eq!(func(vec![10, 1, 1, 6]), vec![9, 0, 1, 6]);
    }
    test(final_prices);
}
