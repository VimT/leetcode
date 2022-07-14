//! 赎金信

pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut c1 = [0; 26];
    let mut c2 = [0; 26];
    for &ch in ransom_note.as_bytes() {
        c1[(ch - b'a') as usize] += 1;
    }
    for &ch in magazine.as_bytes() {
        c2[(ch - b'a') as usize] += 1;
    }
    for i in 0..26 {
        if c1[i] > c2[i] {
            return false;
        }
    }
    true
}

fn main() {
    fn test(func: fn(ransom_note: String, magazine: String) -> bool) {
        assert_eq!(func(String::from("a"), String::from("b")), false);
        assert_eq!(func(String::from("aa"), String::from("ab")), false);
        assert_eq!(func(String::from("aa"), String::from("aab")), true);
    }
    test(can_construct);
}
