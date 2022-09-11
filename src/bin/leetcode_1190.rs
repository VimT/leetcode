//! 反转每对括号间的子串

pub fn reverse_parentheses(s: String) -> String {
    let mut stack = vec![];
    for &ch in s.as_bytes() {
        if ch == b')' {
            let mut tmp = vec![];
            while *stack.last().unwrap() != b'(' {
                tmp.push(stack.pop().unwrap());
            }
            stack.pop();
            for ch in tmp {
                stack.push(ch);
            }
        } else {
            stack.push(ch);
        }
    }
    unsafe { String::from_utf8_unchecked(stack) }
}

/// 记位置，O(n)
pub fn reverse_parentheses2(s: String) -> String {
    let s = s.as_bytes();
    let mut stack = vec![];
    let len = s.len();
    let mut jump = vec![0; len];
    for i in 0..len {
        if s[i] == b'(' {
            stack.push(i);
        } else if s[i] == b')' {
            let j = stack.pop().unwrap();
            jump[i] = j;
            jump[j] = i;
        }
    }
    let mut result = Vec::with_capacity(len);
    let mut idx = 0;
    let mut pos = true;
    while idx < len {
        if s[idx] == b'(' || s[idx] == b')' {
            idx = jump[idx];
            pos = !pos;
        } else {
            result.push(s[idx]);
        }
        if pos { idx += 1; } else { idx -= 1; }
    }
    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    fn test(func: fn(s: String) -> String) {
        assert_eq!(func(String::from("(abcd)")), String::from("dcba"));
        assert_eq!(func(String::from("(u(love)i)")), String::from("iloveu"));
        assert_eq!(func(String::from("(ed(et(oc))el)")), String::from("leetcode"));
    }
    test(reverse_parentheses);
    test(reverse_parentheses2);
}
