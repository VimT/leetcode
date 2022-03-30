//! 实现 strStr()


pub fn str_str(haystack: String, needle: String) -> i32 {
    if needle.len() == 0 { return 0; }
    let mut a = haystack.bytes().skip(0);
    let mut b = needle.bytes().skip(0);
    let mut skip = 0;
    loop {
        match (a.next(), b.next()) {
            (Some(x), Some(y)) => if x != y {
                skip += 1;
                a = haystack.bytes().skip(skip);
                b = needle.bytes().skip(0);
            },
            (_, None) => return skip as i32,
            (None, _) => { return -1; }
        }
    }
}

pub fn str_str_kmp(haystack: String, needle: String) -> i32 {
    let query = haystack.as_bytes();
    let pattern = needle.as_bytes();
    let n = query.len();
    let m = pattern.len();
    if m == 0 { return 0; }
    let mut next = vec![0; m];
    let mut j = 0;
    for i in 1..m {
        // why while: aabaaa, last a need while
        while j > 0 && pattern[i] != pattern[j] {
            j = next[j - 1];
        }
        if pattern[i] == pattern[j] { j += 1; }
        next[i] = j;
    }
    j = 0;
    for i in 0..n {
        while j > 0 && query[i] != pattern[j] {
            j = next[j - 1];
        }
        if query[i] == pattern[j] { j += 1; }
        if j == m {
            return (i + 1 - m) as i32;
        }
    }
    -1
}

fn main() {
    fn test(func: fn(haystack: String, needle: String) -> i32) {
        assert_eq!(func(String::from("hello"), String::from("ll")), 2);
        assert_eq!(func(String::from("aaaaa"), String::from("bba")), -1);
        assert_eq!(func(String::from(""), String::from("")), 0);
    }
    test(str_str);
    test(str_str_kmp);
}
