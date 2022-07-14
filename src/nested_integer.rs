#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

impl NestedInteger {
    fn from_str_inner(s: &[u8], idx: &mut usize) -> NestedInteger {
        if s[*idx] == b'[' {
            let mut result = vec![];
            *idx += 1;
            loop {
                result.push(Self::from_str_inner(s, idx));
                if s[*idx] == b']' {
                    break;
                } else if s[*idx] == b',' {
                    *idx += 1;
                } else {
                    panic!("unknown char {:?}", s[*idx] as char)
                }
            }
            *idx += 1;
            NestedInteger::List(result)
        } else {
            let mut num = 0;
            while s[*idx].is_ascii_digit() {
                num = num * 10 + (s[*idx] - b'0') as i32;
                *idx += 1;
            }
            NestedInteger::Int(num)
        }
    }
    pub fn from_str(vec: &str) -> Vec<Self> {
        let mut s = Vec::with_capacity(vec.len() + 2);
        s.push(b'[');
        for &ch in vec.as_bytes() {
            if !ch.is_ascii_whitespace() {
                s.push(ch);
            }
        }
        s.push(b']');
        if let NestedInteger::List(list) = Self::from_str_inner(&s, &mut 0) {
            return list;
        }
        vec![]
    }
}