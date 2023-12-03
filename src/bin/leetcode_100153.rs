//! 需要添加的硬币的最小数量

/// 思维题
pub fn minimum_added_coins(mut coins: Vec<i32>, target: i32) -> i32 {
    coins.sort_unstable();
    let mut result = 0;
    let mut cover = 0;
    for coin in coins {
        while coin > cover + 1 {
            result += 1;
            cover += cover + 1;
            if cover >= target {
                return result;
            }
        }
        cover += coin;
        if cover >= target {
            return result;
        }
    }
    while cover < target {
        result += 1;
        cover += cover + 1;
    }
    result
}

fn main() {
    fn test(func: fn(coins: Vec<i32>, target: i32) -> i32) {
        assert_eq!(func(vec![1, 4, 10], 19), 2);
        assert_eq!(func(vec![1, 4, 10, 5, 7, 19], 19), 1);
        assert_eq!(func(vec![1, 1, 1], 20), 3);
    }
    test(minimum_added_coins);
}
