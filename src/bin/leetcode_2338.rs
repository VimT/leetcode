//! 统计理想数组的数目

/// 通过观察可以发现,最后的理想数组一定是[a0,a0,a0,a1,a1....ak,ak]的格式
/// 所以可以只考虑其中的(a0,a1...ak)即相邻数字不同的情况
/// dp[i][j]统计以i为起始且长度为j的不重复序列共有多少个
/// 求出dp数组后。对于每一个起始元素i,假设其长度为j时有k个
/// 可以假设该序列为(a0,a1,...aj - 1),我们固定a0,显然满足题意的数组位置还有n-1个,
/// 可以想象将后j-1个元素(a1,a2...aj - 1)按顺序的放在这n-1个位置上,并且每存在一种方案,
/// 我们可以重复前一个元素来形成不同的理想数组,因此元素i的不同元素长度为j的理想数组个数为C(n - 1,j - 1) * k,
/// 累加所有答案即可
pub fn ideal_arrays(n: i32, max_value: i32) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    const MAXP: usize = 16;
    let n = n as usize;
    let max_value = max_value as usize;

    // nlnn 求因数，模板
    let mut fac = vec![vec![]; max_value + 1];
    for i in 1..=max_value {
        let mut j = i << 1;
        while j <= max_value {
            fac[j].push(i);
            j += i;
        }
    }

    // 子问题，arr[i] 不能与 arr[i-1] 相等
    let mut f = vec![vec![0; 20]; max_value + 1];
    for i in 1..=max_value {
        f[i][1] = 1;
        for j in 2..=MAXP {
            for &t in &fac[i] {
                f[i][j] = (f[i][j] + f[t][j - 1]) % MOD;
            }
        }
    }

    // 组合数，模板
    let mut c = vec![vec![0; 20]; n + 1];
    c[0][0] = 1;
    c[1][0] = 1;
    c[1][1] = 1;
    for i in 2..=n {
        c[i][0] = 1;
        for j in 1..=i.min(MAXP) {
            c[i][j] = (c[i - 1][j] + c[i - 1][j - 1]) % MOD;
        }
    }

    let mut result = 0;
    // 固定第一个数为i， 剩下n-1 个数放 j-1个元素，即C(n-1, j-1)，每种组合可以有 f[i][j] 种放法
    for i in 1..=max_value {
        for j in 1..=MAXP {
            result = (result + c[n - 1][j - 1] * f[i][j]) % MOD;
        }
    }
    result as i32
}

fn main() {
    assert_eq!(ideal_arrays(5, 9), 111);
    assert_eq!(ideal_arrays(2, 5), 10);
    assert_eq!(ideal_arrays(5, 3), 11);
}
