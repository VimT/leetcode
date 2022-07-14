//! 最小窗口子序列

/// 字母位置数组，通过二分查找找下一个
pub fn min_window(s1: String, s2: String) -> String {
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();
    let mut pos = vec![vec![]; 26];
    for (i, &ch) in s1.iter().enumerate() {
        pos[(ch - b'a') as usize].push(i);
    }
    let mut result = String::new();
    let mut min_len = usize::MAX;
    'out: for &start in &pos[(s2[0] - b'a') as usize] {
        let mut i = start + 1;
        for &ch in &s2[1..] {
            let next_i_idx = pos[(ch - b'a') as usize].binary_search(&i).unwrap_or_else(|x| x);
            if next_i_idx == pos[(ch - b'a') as usize].len() {
                break 'out;
            }
            i = pos[(ch - b'a') as usize][next_i_idx] + 1;
        }
        if i - start + 1 < min_len {
            min_len = i - start + 1;
            unsafe { result = String::from_utf8_unchecked(s1[start..i].to_vec()); }
        }
    }
    result
}

/// 动态规划(前缀递推)
pub fn min_window2(s1: String, s2: String) -> String {
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();
    let mut cur: Vec<Option<usize>> = s1.iter().enumerate().map(|(i, &ch)| if ch == s2[0] { Some(i) } else { None }).collect();

    // 先计算包含 T 的前缀子串的窗口，再根据前缀子串窗口不断拓展，找到包含整个字符串的窗口。
    for j in 1..s2.len() {
        let mut last = None;
        let mut new = vec![None; s1.len()];
        for (i, &u) in s1.iter().enumerate() {
            if last.is_some() && u == s2[j] {
                new[i] = last;
            }
            if cur[i].is_some() {
                last = cur[i];
            }
        }
        cur = new;
    }
    let mut result = (0, s1.len());
    for (e, s) in cur.iter().enumerate() {
        if let Some(s) = *s {
            if e - s < result.1 - result.0 {
                result = (s, e);
            }
        }
    }
    return if result.1 < s1.len() {
        unsafe { String::from_utf8_unchecked(s1[result.0..=result.1].to_vec()) }
    } else {
        String::new()
    };
}

/// 动态规划(通过 next 数组)
pub fn min_window3(s: String, t: String) -> String {
    let s1 = s.as_bytes();
    let s2 = t.as_bytes();
    let mut next = vec![[0; 26]; s1.len()];
    let mut last = [usize::MAX; 26];
    for i in (0..s1.len()).rev() {
        next[i] = last.clone();
        last[(s1[i] - b'a') as usize] = i;
    }

    let mut result = String::new();
    let mut min_len = usize::MAX;
    'out: for start in 0..s1.len() {
        if s1[start] == s2[0] {
            let mut i = start;
            for &ch in &s2[1..] {
                i = next[i][(ch - b'a') as usize];
                if i >= s1.len() {
                    break 'out;
                }
            }
            if i - start + 1 < min_len {
                min_len = i - start + 1;
                result = s[start..=i].to_string();
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(s1: String, s2: String) -> String) {
        assert_eq!(func(String::from("fgrqsqsnodwmxzkzxwqegkndaa"), String::from("fnok")), String::from("fgrqsqsnodwmxzk"));
        assert_eq!(func(String::from("hpsrhgogezyfrwfrejytjkzvgpjnqil"), String::from("ggj")), String::from("gogezyfrwfrej"));
        assert_eq!(func(String::from("abcdebdde"), String::from("bde")), String::from("bcde"));
        assert_eq!(func(String::from("jmeqksfrsdcmsiwvaovztaqenprpvnbstl"), String::from("u")), String::from(""));
        assert_eq!(func(String::from("cnhczmccqouqadqtmjjzl"), String::from("mm")), String::from("mccqouqadqtm"));
    }
    test(min_window);
    test(min_window2);
    test(min_window3);
}
