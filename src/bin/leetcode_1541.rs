//! 平衡括号字符串的最少插入次数

pub fn min_insertions(s: String) -> i32 {
    let mut stk = vec![];
    let s = s.as_bytes();
    let mut result = 0;
    let mut i = 0;
    while i < s.len() {
        let ch = s[i];
        if ch == b')' {
            if i + 1 < s.len() && s[i + 1] == b')' {
                if stk.is_empty() {
                    result += 1;
                } else {
                    stk.pop().unwrap();
                }
                i += 1;
            } else {
                if stk.is_empty() {
                    result += 2;
                } else {
                    result += 1;
                    stk.pop().unwrap();
                }
            }
        } else {
            stk.push(ch);
        }
        i += 1;
    }
    result += stk.len() as i32 * 2;
    result
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("(()))(()))()())))")), 4);
        assert_eq!(func(String::from("(()))")), 1);
        assert_eq!(func(String::from("())")), 0);
        assert_eq!(func(String::from("))())(")), 3);
        assert_eq!(func(String::from("((((((")), 12);
        assert_eq!(func(String::from(")))))))")), 5);
    }
    test(min_insertions);
}
