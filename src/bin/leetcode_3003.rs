//! 执行操作后的最大分割数量

pub fn max_partitions_after_operations(s: String, k: i32) -> i32 {
    let s = s.as_bytes().iter().map(|x| (*x - b'a') as usize).collect::<Vec<_>>();
    let len = s.len();
    if k == 26 { return 1; }
    if k == 1 { // 时间优化，1单独处理
        let mut result = 1;
        for i in 1..len {
            result += (s[i] != s[i - 1]) as i32;
        }
        if len >= 3 {
            for i in 0..len - 2 {
                if s[i] == s[i + 1] && s[i] == s[i + 2] {
                    return result + 2;
                }
            }
        }
        if len >= 2 {
            for i in 0..len - 1 {
                if s[i] == s[i + 1] {
                    return result + 1;
                }
            }
        }
        return result;
    }
    let mut pre = vec![[0; 26]; len + 1];  // 表示 s[..i] 中每个字符出现的次数
    for i in 0..len {
        pre[i + 1] = pre[i].clone();
        pre[i + 1][s[i]] += 1;
    }
    let mut nxt = vec![0; len];  // nxt[i] 表示 i..len 中满足条件的最小 j
    let mut j = 0;
    let k = k as usize;
    for i in 0..len {
        while j < len && pre[j + 1].iter().zip(&pre[i]).filter(|(a, b)| a > b).count() <= k {
            j += 1;
        }
        nxt[i] = j;
    }
    let mut find = vec![[len; 26]; len + 1];  // find[i] 表示 i 之后（包含i）每个字符第一次出现的位置
    for i in (0..len).rev() {
        find[i] = find[i + 1].clone();
        find[i][s[i]] = i;
    }
    let mut dp = vec![0; len + 1];  // dp[i] 表示 i..len 的最大分割数量
    for i in (0..len).rev() {
        dp[i] = dp[nxt[i]] + 1;
    }
    let mut kk = 0;  // 当前分割的初始位置
    let mut p = 0;  // 当前分割的数量
    let mut result = dp[0];

    // 枚举每个位置i变成j的最大分割数量
    for i in 0..len {
        while nxt[kk] < i {
            kk = nxt[kk];
            p += 1;
        }
        for j in 0..26 {
            if j != s[i] {
                let mut cur = find[kk].clone();
                cur[j] = cur[j].min(i);
                if cur[s[i]] == i {
                    cur[s[i]] = find[i + 1][s[i]];
                }
                cur.sort_unstable();
                if cur[k] == i {  // 如果把 s[i] 变成 j之后，刚好从 i 开始变成了新的分割
                    cur = find[i].clone();
                    cur[j] = i;
                    cur[s[i]] = find[i + 1][s[i]];
                    cur.sort_unstable();
                    result = result.max(p + 2 + dp[cur[k]]);
                } else {
                    result = result.max(p + 1 + dp[cur[k]]);
                }
            }
        }
    }

    result
}

fn main() {
    fn test(func: fn(s: String, k: i32) -> i32) {
        assert_eq!(func(String::from("aca"), 2), 2);
        assert_eq!(func(String::from("ba"), 1), 2);
        assert_eq!(func(String::from("accca"), 2), 3);
        assert_eq!(func(String::from("aabaab"), 3), 1);
        assert_eq!(func(String::from("xxyz"), 1), 4);
    }
    test(max_partitions_after_operations);
}
