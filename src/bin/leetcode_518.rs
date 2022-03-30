//! 零钱兑换 II

pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
    fn inner(left: i32, coins: &Vec<i32>, coin_index: usize) -> i32 {
        if left == 0 { return 1; }
        if coin_index == coins.len() { return 0; }
        let mut ans = 0;
        let coin_value = coins[coin_index];
        for coin_num in 0..=(left / coin_value) {
            ans += inner(left - coin_value * coin_num, coins, coin_index + 1);
        }
        ans
    }
    if amount == 0 || coins.len() == 0 { return 0; }
    return inner(amount, &coins, 0);
}

/// dp[coin_index][left_amount] = sum(dp[coin_index-1][left_amount-coin_value])
/// dp[coin_index][left_amount] = left_amount - coin_value ? dp[coin_index][left_amount-coin_value] + 不选 : 不选
pub fn change_dp(amount: i32, coins: Vec<i32>) -> i32 {
    if amount == 0 || coins.len() == 0 { return 0; }
    let mut dp = vec![vec![0; (amount + 1) as usize]; coins.len() + 1];
    for i in 0..=coins.len() {
        dp[i][0] = 1;
    }

    // must first iter coin_index
    for coin_index in 1..=coins.len() {
        let coin_value = coins[coin_index - 1];
        for left in 1..=amount {
            // let mut sum = 0;
            // for coin_num in 0..(left / coin_value + 1) {
            //     sum += dp[coin_index - 1][(left - coin_num * coin_value) as usize];
            // }
            // dp[coin_index][left as usize] = sum;
            // 优化
            dp[coin_index][left as usize] = if left - coin_value >= 0 {
                dp[coin_index][(left - coin_value) as usize] + dp[coin_index - 1][left as usize]
            } else {
                dp[coin_index - 1][left as usize]
            };
        }
    }
    return dp[coins.len()][amount as usize];
}


/// dp[coin_index][left_amount] = sum(dp[coin_index-1][left_amount-coin_value]) 优化
pub fn change_dp_optimise(amount: i32, coins: Vec<i32>) -> i32 {
    if amount == 0 || coins.len() == 0 { return 0; }
    let mut dp = vec![0; (amount + 1) as usize];
    dp[0] = 1;

    for coin_value in coins {
        for left in coin_value..=amount {
            dp[left as usize] += dp[(left - coin_value) as usize];
        }
    }
    return dp[amount as usize];
}


pub fn change_dp_dfs(amount: i32, mut coins: Vec<i32>) -> i32 {
    coins.sort_unstable();
    fn dfs(coins: &Vec<i32>, amount: i32, idx: i32) -> i32 {
        if amount == 0 {
            return 1;
        }
        if amount < 0 || idx < 0 {
            return 0;
        }
        let result = dfs(coins, amount - coins[idx as usize], idx) + dfs(coins, amount, idx - 1);
        result
    }
    if amount == 0 || coins.len() == 0 { return 0; }
    dfs(&coins, amount, (coins.len() - 1) as i32)
}


fn main() {
    fn test(func: fn(amount: i32, coins: Vec<i32>) -> i32) {
        assert_eq!(func(5, vec![1, 2, 5]), 4);
        assert_eq!(func(3, vec![2]), 0);
        assert_eq!(func(10, vec![10]), 1);
    }
    test(change);
    test(change_dp);
    test(change_dp_dfs);
    test(change_dp_optimise);
}
