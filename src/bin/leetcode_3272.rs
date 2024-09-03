//! 统计好整数的数目

use std::collections::HashSet;

/// 另一种做法：n 和 k 都很小，可以直接打表
pub fn count_good_integers(n: i32, k: i32) -> i64 {
    let n = n as usize;
    let k = k as usize;
    let mut factorial = vec![1; n + 1];
    for i in 1..=n {
        factorial[i] = factorial[i - 1] * i;
    }
    let mut result = 0;
    let mut vis = HashSet::new();
    // 枚举一半
    let base = 10usize.pow((n as u32 - 1) / 2);
    for i in base..base * 10 {
        let mut x = i;
        let mut t = i;
        if n % 2 == 1 {
            t /= 10;
        }
        while t > 0 {
            x = x * 10 + t % 10;
            t /= 10;
        }
        if x % k != 0 { continue; }

        // 通过排序之后的字符串，判断是否已经排列组合过了
        let mut s = x.to_string().into_bytes();
        s.sort_unstable();
        if !vis.insert(s.clone()) { continue; }

        // 组合数学：最高位不能为0，剩下的 A(n-1, n-1)
        let mut cnt = vec![0; 10];
        for &ch in &s {
            cnt[(ch - b'0') as usize] += 1;
        }
        let mut s = (n - cnt[0]) * factorial[n - 1];
        for &num in &cnt {
            s /= factorial[num];
        }
        result += s;
    }
    result as i64
}

fn main() {
    fn test(func: fn(n: i32, k: i32) -> i64) {
        assert_eq!(func(3, 5), 27);
        assert_eq!(func(1, 4), 2);
        assert_eq!(func(5, 6), 2468);
    }
    test(count_good_integers);
}
