//! 累加数

pub fn is_additive_number(num: String) -> bool {
    let s = num.as_bytes();
    let len = s.len();
    if len < 3 { return false; }
    for i in 0..len - 2 {
        if i > 0 && s[0] == b'0' { break; }
        let start = String::from_utf8_lossy(&s[0..=i]).parse::<u128>().unwrap();
        for j in i + 1..len - 1 {
            if j > i + 1 && s[i + 1] == b'0' { break; }
            let mut s1 = start;
            let mut s2 = String::from_utf8_lossy(&s[i + 1..=j]).parse::<u128>().unwrap();
            let mut idx = j + 1;
            while idx < len {
                let ex = s1 + s2;
                let str = ex.to_string();
                let bytes = str.as_bytes();
                if idx + bytes.len() <= len && &s[idx..idx + bytes.len()] == bytes {
                    s1 = s2;
                    s2 = ex;
                    idx += bytes.len();
                } else {
                    break;
                }
            }
            if idx == len {
                return true;
            }
        }
    }
    false
}


fn main() {
    assert_eq!(is_additive_number(String::from("112358")), true);
    assert_eq!(is_additive_number(String::from("199100199")), true);
}
