//! 从栈中取出 K 个硬币的最大面值和

/// dp[left_k] 表示剩余多少个币可以取，对每个 栈，更新dp[left_k] = dp[left_k].max( dp[left_k - 取多少个] + sum(取的) )
pub fn max_value_of_coins(piles: Vec<Vec<i32>>, k: i32) -> i32 {
    let k = k as usize;
    let mut dp = vec![0; k + 1];
    for pile in piles {
        let mut ps = vec![0; pile.len() + 1];
        for i in 0..pile.len() {
            ps[i + 1] = ps[i] + pile[i];
        }
        let mut ndp = dp.clone();
        for left_k in 1..=k {
            let mut max = 0;
            for i in 1..=left_k.min(pile.len()) {
                max = max.max(dp[left_k - i] + ps[i]);
            }
            ndp[left_k] = ndp[left_k].max(max);
        }
        dp = ndp;
    }
    dp[k]
}

fn main() {
    fn test(func: fn(piles: Vec<Vec<i32>>, k: i32) -> i32) {
        assert_eq!(func(vec![vec![48, 14, 23, 38, 33, 79, 3, 52, 73, 58, 49, 23, 74, 44, 69, 76, 83, 41, 46, 32, 28]], 10), 421);
        assert_eq!(func(vec![vec![1, 100, 3], vec![7, 8, 9]], 2), 101);
        assert_eq!(func(vec![vec![100], vec![100], vec![100], vec![100], vec![100], vec![100], vec![1, 1, 1, 1, 1, 1, 700]], 7), 706);
    }
    test(max_value_of_coins);
}
