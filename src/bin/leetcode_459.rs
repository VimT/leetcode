//! 重复的子字符串

pub fn repeated_substring_pattern(s: String) -> bool {
    let s = s.as_bytes();
    let first = s[0];
    let len = s.len();
    'out: for i in 1..len {
        if s[i] == first && len % i == 0 {
            for j in 1..len / i {
                for k in 0..i {
                    if s[j * i + k] != s[k] {
                        continue 'out;
                    }
                }
            }
            return true;
        }
    }
    false
}

/// string find
pub fn repeated_substring_pattern_find(s: String) -> bool {
    (format!("{}{}", &s, &s))[1..s.len() * 2 - 1].find(&s).is_some()
}

/// kmp
pub fn repeated_substring_pattern_kmp(s: String) -> bool {
    fn kmp(query: &[u8], pattern: &[u8]) -> bool {
        let n = query.len();
        let m = pattern.len();
        let mut fail: Vec<isize> = vec![-1; m];
        for i in 1..m {
            let mut j = fail[i - 1];
            while j != -1 && pattern[j as usize + 1] != pattern[i] {
                j = fail[j as usize];
            }
            if pattern[(j + 1) as usize] == pattern[i] {
                fail[i] = j + 1;
            }
        }
        let mut mt: isize = -1;
        for i in 1..n - 1 {
            while mt != -1 && pattern[(mt + 1) as usize] != query[i] {
                mt = fail[mt as usize];
            }
            if pattern[(mt + 1) as usize] == query[i] {
                mt += 1;
                if mt as usize == m - 1 {
                    return true;
                }
            }
        }
        false
    }
    kmp(format!("{}{}", &s, &s).as_bytes(), s.as_bytes())
}


/// optimise kmp
pub fn repeated_substring_pattern_op_kmp(s: String) -> bool {
    fn kmp(pattern: &[u8]) -> bool {
        let n = pattern.len();
        let mut fail = vec![-1_isize; n];
        for i in 1..n {
            let mut j = fail[i - 1];
            while j != -1 && pattern[(j + 1) as usize] != pattern[i] {
                j = fail[j as usize];
            }
            if pattern[(j + 1) as usize] == pattern[i] {
                fail[i] = j + 1;
            }
        }
        return fail[n - 1] != -1 && (n % (n - fail[n - 1] as usize - 1) == 0);
    }
    kmp(s.as_bytes())
}

fn main() {
    fn test(func: fn(s: String) -> bool) {
        assert_eq!(func(String::from("abab")), true);
        assert_eq!(func(String::from("aba")), false);
        assert_eq!(func(String::from("abcabcabcabc")), true);
    }
    test(repeated_substring_pattern);
    test(repeated_substring_pattern_find);
    test(repeated_substring_pattern_kmp);
    test(repeated_substring_pattern_op_kmp);
}
