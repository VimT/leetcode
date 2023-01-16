//! 使字符串总不同字符的数目相等

pub fn is_it_possible(word1: String, word2: String) -> bool {
    let mut cnt1 = [0; 26];
    let mut cnt2 = [0; 26];
    for &ch in word1.as_bytes() {
        cnt1[(ch - b'a') as usize] += 1;
    }
    for &ch in word2.as_bytes() {
        cnt2[(ch - b'a') as usize] += 1;
    }
    let cntc1 = cnt1.iter().filter(|x| **x > 0).count() as i32;
    let cntc2 = cnt2.iter().filter(|x| **x > 0).count() as i32;
    for (x, &c) in cnt1.iter().enumerate() {
        for (y, &d) in cnt2.iter().enumerate() {
            if c > 0 && d > 0 {
                if x == y { // 无变化
                    if cntc1 == cntc2 {
                        return true;
                    }
                } else if cntc1 - (c == 1) as i32 + (cnt1[y] == 0) as i32 == cntc2 - (d == 1) as i32 + (cnt2[x] == 0) as i32 { // 变化量
                    return true;
                }
            }
        }
    }
    false
}

fn main() {
    fn test(func: fn(word1: String, word2: String) -> bool) {
        assert_eq!(func(String::from("aa"), String::from("ab")), false);
        assert_eq!(func(String::from("aa"), String::from("bb")), true);
        assert_eq!(func(String::from("a"), String::from("bb")), false);
        assert_eq!(func(String::from("ac"), String::from("b")), false);
        assert_eq!(func(String::from("abcc"), String::from("aab")), true);
        assert_eq!(func(String::from("abcde"), String::from("fghij")), true);
    }
    test(is_it_possible);
}
