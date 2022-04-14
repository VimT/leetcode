//! 招商银行-04. 商店促销活动

/// dp[i][4][3]表示前i个，a数组取了0/1/2/>=3个，b数组发生囤积的个数有0/1/2个时的最小代价和
/// 两次dp:
/// 第一次钦定取够3个（转移按0.7倍转移，只把a>=3的终态当合法状态）。
/// 第二次钦定不够3个（转移按原价转移，只把a<3的终态当合法状态）。
pub fn go_shopping(price_a: Vec<i32>, price_b: Vec<i32>) -> i32 {
    let len = price_a.len();
    let mut price: Vec<(i32, i32)> = price_a.into_iter().zip(price_b).collect();
    price.sort_unstable_by_key(|x| -x.1);
    let mut dp = vec![vec![vec![i64::MAX / 2; 3]; 4]; len + 1];
    dp[0][0][0] = 0;
    for i in 0..len {
        for j in 0..4 {
            for k in 0..3 {
                dp[i + 1][3.min(j + 1)][k] = dp[i + 1][3.min(j + 1)][k].min(dp[i][j][k] + price[i].0 as i64 * 7);
                dp[i + 1][j][(k + 1) % 3] = dp[i + 1][j][(k + 1) % 3].min(dp[i][j][k] + if k == 2 { 0 } else { price[i].1 as i64 * 10 });
            }
        }
    }

    let mut result = i64::MAX / 2;
    for k in 0..3 {
        result = result.min(dp[len][3][k]);
    }
    dp = vec![vec![vec![i64::MAX / 2; 3]; 4]; len + 1];
    dp[0][0][0] = 0;
    for i in 0..len {
        for j in 0..3 {
            for k in 0..3 {
                if j + 1 < 3 {
                    dp[i + 1][j + 1][k] = dp[i + 1][j + 1][k].min(dp[i][j][k] + price[i].0 as i64 * 10);
                }
                dp[i + 1][j][(k + 1) % 3] = dp[i + 1][j][(k + 1) % 3].min(dp[i][j][k] + if k == 2 { 0 } else { price[i].1 as i64 * 10 });
            }
        }
    }
    for j in 0..3 {
        for k in 0..3 {
            result = result.min(dp[len][j][k]);
        }
    }
    (result / 10) as i32
}

fn main() {
    assert_eq!(go_shopping(vec![1, 2, 5], vec![2, 2, 2]), 4);
    assert_eq!(go_shopping(vec![1, 6, 1], vec![2, 2, 6]), 4);
    assert_eq!(go_shopping(vec![3, 13, 5, 12], vec![28, 12, 20, 7]), 21);
}
