//! 统计特殊整数

/// 自己写的
pub fn count_special_numbers(n: i32) -> i32 {
    let mut num = n;
    let mut nv = vec![];
    while num > 0 {
        nv.push(num % 10);
        num /= 10;
    }
    nv.reverse();
    fn backtrace(max: &Vec<i32>, factorial: &Vec<i32>, i: usize, seen: &mut Vec<bool>, result: &mut i32, is_max: bool, is_first: bool) {
        if i == max.len() {
            *result += 1;
            return;
        }
        if !is_max {
            // 当不是最大时，可以直接用排列计算
            let n = seen.iter().filter(|x| !**x).count();
            let m = max.len() - i;
            *result += factorial[n] / factorial[n - m];
            return;
        }
        for num in if is_first { 1 } else { 0 }..=if is_max { max[i] } else { 9 } {
            if !seen[num as usize] {
                seen[num as usize] = true;
                backtrace(max, factorial, i + 1, seen, result, is_max && num == max[i], false);
                seen[num as usize] = false;
            }
        }
    }

    let mut result = 0;
    let mut factorial = vec![1, 1];
    for i in 2..=10 {
        factorial.push(factorial[i as usize - 1] * i);
    }
    // 小于数长度的 排列
    for i in 1..nv.len() {
        result += 9 * (factorial[9] / factorial[10 - i]);
    }
    let mut seen = vec![false; 10];
    // 等于数长度的backtrace
    backtrace(&nv, &factorial, 0, &mut seen, &mut result, true, true);
    result
}

/// 排列优化，不用backtrace，直接for循环就行
pub fn count_special_numbers2(n: i32) -> i32 {
    let mut num = n;
    let mut nv = vec![];
    while num > 0 {
        nv.push(num % 10);
        num /= 10;
    }
    nv.reverse();
    let mut result = 0;
    let mut factorial = vec![1, 1];
    for i in 2..=10 {
        factorial.push(factorial[i as usize - 1] * i);
    }
    // 小于数长度的 排列
    for i in 1..nv.len() {
        result += 9 * (factorial[9] / factorial[10 - i]);
    }

    let mut seen = vec![false; 10];
    let mut b = false; // 原数字中是否有重复数字
    for (i, &c) in nv.iter().enumerate() {
        for j in 0..c {
            // 首位不能为0
            if i as i32 + j == 0 || seen[j as usize] {
                continue;
            }
            // P(10-i-1, len-i-1)
            result += factorial[9 - i] / factorial[10 - nv.len()];
        }
        if seen[c as usize] {
            b = true;
            break;
        }
        seen[c as usize] = true;
    }
    if !b { result += 1; }
    result
}

/// 数位DP模板
pub fn count_special_numbers3(n: i32) -> i32 {
    let mut num = n;
    let mut s = vec![];
    while num > 0 {
        s.push(num % 10);
        num /= 10;
    }
    s.reverse();
    let m = s.len();
    let mut dp = vec![vec![-1; 1 << 10]; m];

    /// is_num: 非首位，is_limit: 到上限
    fn dfs(s: &Vec<i32>, dp: &mut Vec<Vec<i32>>, i: usize, mask: usize, is_limit: bool, is_num: bool) -> i32 {
        if i == s.len() { return is_num as i32; }
        if !is_limit && is_num && dp[i][mask] >= 0 { return dp[i][mask]; }
        let mut result = 0;
        if !is_num {
            // 可以跳过当前数位
            result = dfs(s, dp, i + 1, mask, false, false);
        }
        let start = if is_num { 0 } else { 1 };
        let end = if is_limit { s[i] } else { 9 };
        for d in start..=end {
            if mask >> d & 1 == 0 {
                result += dfs(s, dp, i + 1, mask | (1 << d), is_limit && d == end, true)
            }
        }
        if !is_limit && is_num {
            dp[i][mask] = result;
        }
        result
    }
    dfs(&s, &mut dp, 0, 0, true, false)
}

fn main() {
    fn test(func: fn(n: i32) -> i32) {
        assert_eq!(func(20), 19);
        assert_eq!(func(5), 5);
        assert_eq!(func(135), 110);
    }
    test(count_special_numbers);
    test(count_special_numbers2);
    test(count_special_numbers3);
}
