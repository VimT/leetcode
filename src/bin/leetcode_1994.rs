//! 好子集的数目

const PRIMES: [i32; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
const MOD: i32 = 1e9 as i32 + 7;
const NUM_MAX: usize = 30;

///定义 f[state]为当前子集乘积拆解结果的用到的质数为 state 时的方案数，state
/// 为一个长度 10 的二进制数，若 state中的第 k 位二进制表示为 1，代表数值 p[k] 在拆解结果中出现过
/// 若第 k 位二进制表示为 0 代表 p[k] 在拆解结果中没出现过。
/// f[s] 为「所有合法的 prevprev 的状态数 f[prev]」与「数值 t 的出现次数 cnts[t] 」的乘积之和
pub fn number_of_good_subsets(nums: Vec<i32>) -> i32 {
    let mut freq = vec![0; NUM_MAX + 1];
    for num in nums {
        freq[num as usize] += 1;
    }
    let mut dp = vec![0; 1 << PRIMES.len()];
    dp[0] = 1;
    for _ in 0..freq[1] {
        dp[0] = dp[0] * 2 % MOD;
    }
    for i in 2..=NUM_MAX {
        if freq[i] == 0 {
            continue;
        }
        let mut subset = 0;
        let x = i as i32;
        let mut check = true;
        for (j, &prime) in PRIMES.iter().enumerate() {
            if x % (prime * prime) == 0 {
                check = false;
                break;
            }
            if x % prime == 0 {
                subset |= 1 << j;
            }
        }
        if !check {
            continue;
        }
        for mask in (1..1 << PRIMES.len()).rev() {
            if mask & subset == subset {
                dp[mask] = ((dp[mask] as i64 + dp[mask ^ subset] as i64 * freq[i] as i64) % MOD as i64) as i32;
            }
        }
    }
    let mut result = 0;
    for mask in 1..1 << PRIMES.len() {
        result = (result + dp[mask]) % MOD;
    }
    result
}

fn main() {
    assert_eq!(number_of_good_subsets(vec![1, 2, 3, 4]), 6);
    assert_eq!(number_of_good_subsets(vec![4, 2, 3, 15]), 5);
}
