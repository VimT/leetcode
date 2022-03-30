//! 重复叠加字符串匹配

pub fn repeated_string_match(a: String, b: String) -> i32 {
    let a = a.as_bytes();
    let b = b.as_bytes();
    let mut av = [false; 26];
    for &ch in a {
        av[(ch - b'a') as usize] = true;
    }
    for &ch in b {
        if !av[(ch - b'a') as usize] {
            return -1;
        }
    }
    let alen = a.len();
    let blen = b.len();
    for i in 0..alen {
        if a[i] == b[0] {
            let mut ai = i;
            let mut bi = 0;
            while bi < blen && a[ai % alen] == b[bi] {
                ai += 1;
                bi += 1;
            }
            if bi == blen {
                return ((ai - 1) / alen + 1) as i32;
            }
        }
    }

    -1
}

pub fn repeated_string_match_kmp(a: String, b: String) -> i32 {
    let a = a.as_bytes();
    let b = b.as_bytes();
    let mut av = [false; 26];
    for &ch in a {
        av[(ch - b'a') as usize] = true;
    }
    for &ch in b {
        if !av[(ch - b'a') as usize] {
            return -1;
        }
    }
    let alen = a.len();
    let blen = b.len();
    let mut next = vec![0; blen];
    let mut j = 0;
    for i in 1..blen {
        while j > 0 && b[i] != b[j] {
            j = next[j - 1];
        }
        if b[i] == b[j] { j += 1 }
        next[i] = j;
    }

    let mut i = 0;
    j = 0;
    // while!
    while i - j < alen {
        while j > 0 && a[i % alen] != b[j] {
            j = next[j - 1];
        }
        if a[i % alen] == b[j] { j += 1; }
        if j == blen { return (i / alen + 1) as i32; }
        i += 1;
    }
    -1
}

fn main() {
    assert_eq!(repeated_string_match_kmp(String::from("abcd"), String::from("cdabcdab")), 3);
    assert_eq!(repeated_string_match_kmp(String::from("a"), String::from("aa")), 2);
    assert_eq!(repeated_string_match_kmp(String::from("a"), String::from("a")), 1);
    assert_eq!(repeated_string_match_kmp(String::from("abc"), String::from("wxyz")), -1);
}
