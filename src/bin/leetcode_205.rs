//! 同构字符串

pub fn is_isomorphic(s: String, t: String) -> bool {
    let s = s.as_bytes();
    let t = t.as_bytes();
    let mut map = vec![0; 256];
    let mut map2 = vec![0; 256];
    let len = s.len();
    for i in 0..len {
        let idx = (s[i] - b'a') as usize;
        let idx2 = (t[i] - b'a') as usize;
        if map[idx] != map2[idx2] {
            return false;
        }
        if map[idx] == 0 {
            map[idx] = i + 1;
            map2[idx2] = i + 1;
        }
    }
    return true;
}

fn main() {
    fn test(func: fn(s: String, t: String) -> bool) {
        assert_eq!(func(String::from("egg"), String::from("add")), true);
        assert_eq!(func(String::from("foo"), String::from("bar")), false);
        assert_eq!(func(String::from("paper"), String::from("title")), true);
    }
    test(is_isomorphic);
}
