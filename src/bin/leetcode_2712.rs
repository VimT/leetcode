//! 使所有字符相等的最小成本

/// 分成 相同字符的n个部分，找分割点，分割点左边都按 [0, i] 翻转，分割点右边按 [i+1, n] 翻转
/// 左边翻转的时候，需要前缀和的前缀和。
pub fn minimum_cost(s: String) -> i64 {
    let mut parts = vec![];
    let s = s.as_bytes();
    let mut last = s[0];
    let mut cnt = 1;
    for i in 1..s.len() {
        if s[i] != last {
            parts.push(cnt);
            cnt = 1;
            last = s[i];
        } else {
            cnt += 1;
        }
    }
    parts.push(cnt);

    let len = parts.len();
    let mut ps = vec![0; len + 1];
    for i in 0..len {
        ps[i + 1] = ps[i] + parts[i];
    }
    let mut pss = vec![0; len + 1];
    for i in 0..len {
        pss[i + 1] = pss[i] + ps[i];
    }
    let mut ss = vec![0; len + 1];
    for i in (0..len).rev() {
        ss[i] = ss[i + 1] + parts[i];
    }
    let mut sss = vec![0; len + 1];
    for i in (0..len).rev() {
        sss[i] = sss[i + 1] + ss[i];
    }
    let mut result = i64::MAX;
    for i in 0..=len {
        result = result.min(pss[i] + sss[i]);
    }
    result
}

/// 简洁写法
pub fn minimum_cost2(s: String) -> i64 {
    let mut result = 0;
    let s = s.as_bytes();
    let len = s.len();
    for i in 1..len {
        if s[i - 1] != s[i] {
            result += i.min(len - i);
        }
    }
    result as i64
}

fn main() {
    fn test(func: fn(s: String) -> i64) {
        assert_eq!(func(String::from("0011")), 2);
        assert_eq!(func(String::from("010101")), 9);
    }
    test(minimum_cost);
    test(minimum_cost2);
}
