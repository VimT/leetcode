//! 统计平衡排列的数目

use leetcode::algorithm::quick_pow;

pub fn count_balanced_permutations(num: String) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let num: Vec<i32> = num.as_bytes().iter().map(|&ch| (ch - b'0') as i32).collect();
    let n = num.len();
    let mut jc = vec![1; n + 1];
    for i in 1..=n { jc[i] = jc[i - 1] * i as i64 % MOD; }
    let mut cnt = [0; 10];
    let mut sum = 0;
    for &x in &num {
        cnt[x as usize] += 1;
        sum += x;
    }
    if sum % 2 == 1 { return 0; }
    let mut dp = vec![vec![0; sum as usize / 2 + 1]; n / 2 + 1]; // dp[i][j] 表示 num 选 i 个数， 和为 j 的方案数
    dp[0][0] = 1;
    for i in 0..n {
        for j in (1..=n / 2).rev() {
            for k in num[i] as usize..=sum as usize / 2 {
                dp[j][k] = (dp[j][k] + dp[j - 1][k - num[i] as usize]) % MOD;
            }
        }
    }
    let mut result = dp[n / 2][sum as usize / 2];
    result = result * jc[n / 2] % MOD * jc[n - n / 2] % MOD;  // 奇数全排列*偶数全排列
    // 为什么可以一起除？ 而不是 / 奇数部分重复的排列数 / 偶数部分的排列数
    // 因为 dp[i][j] 中已经包含了组合数，a! * (k-a)! = k! / C(k,a)，dp[i][j] 中已经包含了 C(k, a)
    for i in 0..10 { result = result * quick_pow(jc[cnt[i]], MOD - 2, MOD) % MOD; } // 除以每个数字重复的次数 （逆元） 
    result as i32
}

fn main() {
    fn test(func: fn(num: String) -> i32) {
        assert_eq!(func(String::from("08143")), 12);
        assert_eq!(func(String::from("3579118200033066464182021687812113473741785427")), 98585545);
        assert_eq!(func(String::from("4622875")), 288);
        assert_eq!(func(String::from("123")), 2);
        assert_eq!(func(String::from("112")), 1);
        assert_eq!(func(String::from("12345")), 0);
    }
    test(count_balanced_permutations);
}
