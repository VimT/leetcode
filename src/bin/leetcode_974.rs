//! 和可被 K 整除的子数组

use std::collections::HashMap;

/// 前缀和   sum(i,j) 就可以写成 P[j]−P[i−1]
/// 需要(P[j]-P[i-1]) % k == 0，根据同余定理， 只要 P[j] % k == P[i] % k ，就表示 sum[i,j] 可以被k整除
/// 统计以 i 结尾的符合条件的子数组个数
pub fn subarrays_div_by_k(a: Vec<i32>, k: i32) -> i32 {
    let mut record = HashMap::new();
    record.insert(0, 1);
    let mut ans = 0;
    let mut sum = 0;
    for i in a {
        sum += i;
        let modulus = (sum % k + k) % k;
        if let Some(v) = record.get(&modulus) {
            ans += *v;
        }
        *record.entry(modulus).or_default() += 1;
    }
    ans
}


/// 排列组合
/// 如果 前缀和余数为某个数的 有4个的话，就有 C(4,2)
pub fn subarrays_div_by_k_c(a: Vec<i32>, k: i32) -> i32 {
    let mut record = HashMap::new();
    record.insert(0, 1);
    let mut ans = 0;
    let mut sum = 0;
    for i in a {
        sum += i;
        let modulus = (sum % k + k) % k;
        *record.entry(modulus).or_default() += 1;
    }
    for (_, v) in record {
        ans += v * (v - 1) / 2;
    }
    ans
}

fn main() {
    assert_eq!(subarrays_div_by_k(vec![-6, 1, -5, 10], 9), 1);
    assert_eq!(subarrays_div_by_k(vec![4, 5, 0, -2, -3, 1], 5), 7);
    assert_eq!(subarrays_div_by_k(vec![2, -5], 8), 0);
}
