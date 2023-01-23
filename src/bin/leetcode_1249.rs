//! 移除无效的括号

pub fn min_remove_to_make_valid(s: String) -> String {
    let mut stack = vec![];
    for (i, &ch) in s.as_bytes().iter().enumerate() {
        if ch == b'(' {
            stack.push(i);
        } else if ch == b')' {
            if !stack.is_empty() && s.as_bytes()[*stack.last().unwrap()] == b'(' {
                stack.pop();
            } else { stack.push(i); }
        }
    }
    stack.push(s.len());
    let mut result = String::with_capacity(s.len());
    let mut start = 0;
    for pos in stack {
        result += &s[start..pos];
        start = pos + 1;
    }
    result
}

fn main() {
    fn test(func: fn(s: String) -> String) {
        assert_eq!(func(String::from("lee(t(c)o)de)")), String::from("lee(t(c)o)de"));
        assert_eq!(func(String::from("a)b(c)d")), String::from("ab(c)d"));
        assert_eq!(func(String::from("))((")), String::from(""));
    }
    test(min_remove_to_make_valid);
}
