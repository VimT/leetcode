//! 基本计算器 III

pub fn calculate(s: String) -> i32 {
    fn cal(s: &[u8], i: &mut usize) -> i32 {
        let mut stack = vec![];
        let mut cur = 0;
        let mut flag = b'+';
        while *i < s.len() {
            if s[*i].is_ascii_whitespace() {} else if s[*i].is_ascii_digit() {
                cur = cur * 10 + (s[*i] - b'0') as i32;
            } else if s[*i] == b'(' {
                *i += 1;
                cur = cal(s, i);
            } else {
                match flag {
                    b'+' => stack.push(cur),
                    b'-' => stack.push(-cur),
                    b'*' => {
                        let pop = stack.pop().unwrap();
                        stack.push(pop * cur);
                    }
                    b'/' => {
                        let pop = stack.pop().unwrap();
                        stack.push(pop / cur);
                    }
                    _ => panic!()
                }
                flag = s[*i];
                cur = 0;
                if flag == b')' { break; }
            }
            *i += 1;
        }
        let mut result = 0;
        while !stack.is_empty() {
            result += stack.pop().unwrap();
        }
        result
    }
    let mut s = s.into_bytes();
    s.push(b'?');
    cal(&s, &mut 0)
}


fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("2*(5+5*2)/3+(6/2+8)")), 21);
        assert_eq!(func(String::from("1+1")), 2);
        assert_eq!(func(String::from("6-4/2")), 4);
    }
    test(calculate);
}
