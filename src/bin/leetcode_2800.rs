//! 包含三个字符串的最短字符串

pub fn minimum_string(a: String, b: String, c: String) -> String {
    fn merge(a: &str, b: &str) -> String {
        if a.len() > b.len() && a.contains(b) {
            return a.to_string();
        }
        if b.len() > a.len() && b.contains(a) {
            return b.to_string();
        }
        // 可以使用kmp优化
        for i in (0..=a.len().min(b.len())).rev() {
            if a[a.len() - i..] == b[..i] {
                return a.to_string() + &b[i..];
            }
        }
        return a.to_string() + b;
    }

    fn merge2(a: &str, b: &str, c: &str) -> String {
        merge(&merge(a, b), c)
    }

    let a = &a;
    let b = &b;
    let c = &c;
    let mut select = vec![];
    select.push(merge2(a, b, c));
    select.push(merge2(a, c, b));
    select.push(merge2(b, a, c));
    select.push(merge2(b, c, a));
    select.push(merge2(c, b, a));
    select.push(merge2(c, a, b));
    select.into_iter().map(|x| (x.len(), x)).min().unwrap().1
}

fn main() {
    fn test(func: fn(a: String, b: String, c: String) -> String) {
        assert_eq!(func(String::from("abc"), String::from("b"), String::from("c")), String::from("abc"));
        assert_eq!(func(String::from("abc"), String::from("bca"), String::from("aaa")), String::from("aaabca"));
        assert_eq!(func(String::from("ab"), String::from("ba"), String::from("aba")), String::from("aba"));
    }
    test(minimum_string);
}
