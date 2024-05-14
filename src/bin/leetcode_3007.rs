//! 价值和小于等于 K 的最大数字

/// 二分+数位DP
pub fn find_maximum_number(k: i64, x: i32) -> i64 {
    fn dfs(d @ (num, x): (i64, usize), i: usize, cnt: i64, is_limit: bool, mem: &mut Vec<Vec<i64>>) -> i64 {
        if i == 0 { return cnt; }
        if !is_limit && mem[i][cnt as usize] != -1 {
            return mem[i][cnt as usize];
        }
        let mut result = 0;
        let up = if is_limit { num >> (i - 1) & 1 } else { 1 };
        for b in 0..=up {
            result += dfs(d, i - 1, cnt + (b == 1 && i % x == 0) as i64, is_limit && b == up, mem);
        }
        if !is_limit { mem[i][cnt as usize] = result; }
        result
    }
    let mut left = 0;
    let mut right = (k + 1) << x;
    while left < right {
        let mid = (left + right + 1) / 2;
        let mut mem = vec![vec![-1; 64]; 64];
        let cnt = dfs((mid, x as usize), (64 - mid.leading_zeros()) as usize, 0, true, &mut mem);  // 0-mid 有多少个设置位
        if cnt > k {
            right = mid - 1;
        } else {
            left = mid;
        }
    }
    left
}


/// 构造：贡献法，当第i位位1时，价值能增加多少
pub fn find_maximum_number2(mut k: i64, x: i32) -> i64 {
    let mut num = 0;
    let mut pre = 0i64;
    let x = x as i64;
    for i in (0..63 - ((k + 1) << x).leading_zeros() as i64).rev() {
        let cnt = (pre << i) + (i / x << i >> 1);
        if cnt <= k {
            k -= cnt;
            num |= 1 << i;
            pre += ((i + 1) % x == 0) as i64;
        }
    }
    num - 1
}

fn main() {
    fn test(func: fn(k: i64, x: i32) -> i64) {
        assert_eq!(func(9, 1), 6);
        assert_eq!(func(7, 2), 9);
    }
    test(find_maximum_number);
    test(find_maximum_number2);
}
