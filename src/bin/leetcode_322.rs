//! 零钱兑换
/// 转化为完全背包问题，恰好装满背包时，价值最小。
pub fn coin_change_pack(coins: Vec<i32>, amount: i32) -> i32 {
    let amount = amount as usize;
    let mut dp = vec![i32::MAX / 2; amount + 1];
    dp[0] = 0;
    let len = coins.len();
    for i in 0..len {
        let c = coins[i] as usize;
        for v in c..=amount {
            dp[v] = dp[v].min(dp[v - c] + 1);
        }
    }
    return if dp[amount] >= i32::MAX / 2 { -1 } else { dp[amount] };
}


/// dp[i] = min( dp[i-c] + 1 )  组成i的最少硬币数量
pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    if amount <= 0 { return 0; }
    if coins.len() == 0 { return -1; }
    let amount = amount as usize;
    let mut dp = vec![-1; amount + 1];
    for i in coins.iter().cloned() {
        if i <= amount as i32 {
            dp[i as usize] = 1;
        }
    }
    for i in 1..=amount {
        if dp[i] == -1 {
            for c in coins.iter().cloned() {
                if i > c as usize {
                    let smaller = *dp.get(i - c as usize).unwrap_or(&-1);
                    if smaller > 0 && (dp[i] == -1 || smaller + 1 < dp[i]) {
                        dp[i] = smaller + 1;
                    }
                }
            }
        }
    }

    return dp[amount];
}

pub fn coin_change_greedy(mut coins: Vec<i32>, amount: i32) -> i32 {
    coins.sort_unstable();
    coins.reverse();

    fn inner(coins: &Vec<i32>, current_coin: usize, left_amount: usize, current_count: usize, current_min: &mut usize) {
        if left_amount == 0 {
            if current_count < *current_min {
                *current_min = current_count;
            }
            return;
        }
        if current_coin >= coins.len() { return; }

        let current_coin_num = left_amount / coins[current_coin] as usize;
        for k in (0..=current_coin_num).rev() {
            if k + current_count >= *current_min { break; }
            inner(coins, current_coin + 1, left_amount - k * coins[current_coin] as usize, current_count + k, current_min);
        }
    }
    let mut ans = usize::MAX;
    inner(&coins, 0, amount as usize, 0, &mut ans);
    return if ans == usize::MAX { -1 } else { ans as i32 };
}

pub fn coin_change_sao(coins: Vec<i32>, amount: i32) -> i32 {
    use std::ops::Add;
    use std::ops::Sub;
    let mut dp = vec![-1i32; (amount + 1) as usize];
    dp[0] = 0;
    for i in 1..=amount {
        dp[i as usize] = coins
            .iter()
            .map(|coin| dp.get(i.sub(*coin) as usize).filter(|&c| c.ne(&-1)))
            .flatten()
            .min()
            .map(|c| c.add(&1))
            .unwrap_or(-1);
    }
    dp.pop().unwrap_or(-1)
}


fn main() {
    fn test(func: fn(coins: Vec<i32>, amount: i32) -> i32) {
        assert_eq!(func(vec![1, 2, 5], 11), 3);
        assert_eq!(func(vec![2], 3), -1);
        assert_eq!(func(vec![1], 0), 0);
    }
    test(coin_change);
    test(coin_change_pack);
    test(coin_change_sao);
    test(coin_change_greedy);
}
