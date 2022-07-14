//! 按字典序排列最小的等效字符串

pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
    let mut m: Vec<usize> = (0..26).collect();
    fn find(m: &mut Vec<usize>, x: usize) -> usize {
        return if m[x] == x {
            x
        } else {
            m[x] = find(m, m[x]);
            m[x]
        };
    }

    for (&a, &b) in s1.as_bytes().iter().zip(s2.as_bytes()) {
        let mut aa = find(&mut m, (a - b'a') as usize);
        let mut bb = find(&mut m, (b - b'a') as usize);
        if aa != bb {
            if aa > bb {
                std::mem::swap(&mut aa, &mut bb);
            }
            m[bb] = aa;
        }
    }

    unsafe {
        String::from_utf8_unchecked(base_str.as_bytes().iter().map(|x| find(&mut m, (*x - b'a') as usize) as u8 + b'a').collect())
    }
}

fn main() {
    fn test(func: fn(s1: String, s2: String, base_str: String) -> String) {
        assert_eq!(func(String::from("parker"), String::from("morris"), String::from("parser")), String::from("makkek"));
        assert_eq!(func(String::from("hello"), String::from("world"), String::from("hold")), String::from("hdld"));
        assert_eq!(func(String::from("leetcode"), String::from("programs"), String::from("sourcecode")), String::from("aauaaaaada"));
    }
    test(smallest_equivalent_string);
}
