//! 同位字符串连接的最小长度

pub fn min_anagram_length(s: String) -> i32 {
    let s = s.as_bytes();
    let len = s.len();
    for i in 1..=len / 2 {
        if len % i == 0 {
            let group = len / i;
            let mut first = [0; 26];
            for j in 0..i {
                first[(s[j] - b'a') as usize] += 1;
            }
            let mut same = true;
            for j in 1..group {
                let mut second = [0; 26];
                for k in 0..i {
                    second[(s[j * i + k] - b'a') as usize] += 1;
                }
                if second != first {
                    same = false;
                    break;
                }
            }
            if same { return i as i32; }
        }
    }
    len as i32
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("abba")), 2);
        assert_eq!(func(String::from("cdef")), 4);
    }
    test(min_anagram_length);
}
