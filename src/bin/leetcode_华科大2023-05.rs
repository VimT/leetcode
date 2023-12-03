//! 计算子集

use leetcode::algorithm::{cal_prime, quick_pow};

const MOD: i64 = 998244353;

fn moebius(arr: &mut [i64]) {
    let prime = unsafe {
        static mut PRIME: Vec<usize> = Vec::new();
        if PRIME.is_empty() { PRIME = cal_prime(1e5 as usize + 1); }
        &PRIME
    };
    if arr.len() <= 2 { return; }
    let m = arr.len() - 1;
    for &p in prime {
        if p > m { break; }
        for i in (1..=m / p).rev() {
            arr[p * i] = (arr[p * i] - arr[i]) % MOD;
        }
    }
}

/// https://leetcode.cn/circle/discuss/Qvv72W/
pub fn subset_counting(n: i64, k: i32, m: i32) -> Vec<i32> {
    let (k, m) = (k as usize, m as usize);
    let mut ret = vec![0; m * 2 + 1];
    if n == 0 { ret[1..=m].fill(1); } else {
        let mut flg = vec![false; m + 1];
        let mut i = usize::MAX;
        loop {
            i = i.wrapping_add(2);
            if i > m / 2 { break; }
            if m % i != 0 { continue; }
            let k = m / i;
            let v = quick_pow(2, n * (k as i64), MOD);
            for j in 1..=i {
                let ind = k * j;
                if !flg[ind] {
                    flg[ind] = true;
                    ret[ind] = v;
                }
            }
        }
        if m & 1 != 0 {
            let v = quick_pow(2, n, MOD);
            for j in 1..=m {
                if !flg[j] {
                    ret[j] = v;
                }
            }
        }
    }
    {
        moebius(&mut ret[..m + 1]);
        {
            let (a, b) = ret.split_at_mut(m + 1);
            b.copy_from_slice(&a[1..]);
        }
        let m_inv = quick_pow(m as i64, MOD - 2, MOD);
        let mut flg = vec![false; m + 1];
        for mut i in 1..=m / 2 + 1 {
            if i > m / 2 { i = m; }
            if m % i != 0 { continue; }
            let k = m / i;
            let v = {
                let k = k as i64;
                let mut v = k * (ret[m + i]) % MOD;
                for d in 1..=k / 2 {
                    if k % d != 0 { continue; }
                    v = (d * (ret[m + m / (d as usize)])
                        + v) % MOD;
                }
                (v * (m_inv as i64) % MOD + MOD) % MOD
            };
            for j in 1..=i {
                let ind = k * j;
                if !flg[ind] {
                    flg[ind] = true;
                    ret[ind] = v;
                }
            }
        }
        ret[0] = ret[m];
    }
    {
        for i in 1..=k {
            let i = i % m;
            if i == 0 {
                for v in &mut ret[..m] {
                    *v = *v * 2 % MOD;
                }
                continue;
            }
            ret[m + i - 1] = ret[i - 1];
            for j in (i..m + i).rev() {
                ret[j] = (ret[j] + ret[j - i]) % MOD;
            }
            {
                let (a, b) = ret.split_at_mut(m);
                a[..i].copy_from_slice(&b[..i]);
            }
        }
    }
    ret[..m].into_iter().map(|x| *x as i32).collect()
}


fn main() {
    fn test(func: fn(n: i64, k: i32, m: i32) -> Vec<i32>) {
        assert_eq!(func(1, 1, 2), vec![4, 4]);
        assert_eq!(func(1919, 8, 10), vec![577613260, 577613260, 822345879, 577613260, 822345879, 577613260, 577613260, 822345879, 577613260, 822345879]);
    }
    test(subset_counting);
}
