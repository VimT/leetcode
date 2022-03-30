//! 迷你语法分析器

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

pub fn deserialize(s: String) -> NestedInteger {
    fn dfs(s: &[u8], i: &mut usize) -> NestedInteger {
        if s[*i] == b'[' {
            *i += 1;
            let mut arr = vec![];
            while s[*i] != b']' {
                arr.push(dfs(s, i));
                if s[*i] == b',' {
                    *i += 1;
                }
            }
            *i += 1;
            NestedInteger::List(arr)
        } else {
            let mut num = 0;
            let neg = s[*i] == b'-';
            if neg { *i += 1; }
            while *i < s.len() && s[*i].is_ascii_digit() {
                num = num * 10 + (s[*i] - b'0') as i32;
                *i += 1;
            }
            if neg { num = -num; }
            NestedInteger::Int(num)
        }
    }
    dfs(s.as_bytes(), &mut 0)
}

fn main() {
    assert_eq!(deserialize(String::from("-3")), NestedInteger::Int(-3));
    assert_eq!(deserialize(String::from("324")), NestedInteger::Int(324));
    assert_eq!(deserialize(String::from("[123,[456,[789]]]")), NestedInteger::List(vec![NestedInteger::Int(123), NestedInteger::List(vec![NestedInteger::Int(456), NestedInteger::List(vec![NestedInteger::Int(789)])])]));
}
