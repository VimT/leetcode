//! 有效括号的嵌套深度

pub fn max_depth_after_split(seq: String) -> Vec<i32> {
    let s = seq.as_bytes();
    let mut stack = vec![];
    let len = s.len();
    let mut result = vec![0; len];
    for i in 0..len {
        if s[i] == b'(' {
            let idx = stack.len();
            stack.push(i);
            result[i] = (idx & 1) as i32;
        } else {
            let idx = stack.pop().unwrap();
            result[i] = result[idx];
        }
    }
    result
}

fn main() {
    fn test(func: fn(seq: String) -> Vec<i32>) {
        assert_eq!(func(String::from("(()())")), vec![0, 1, 1, 1, 1, 0]);
        assert_eq!(func(String::from("()(())()")), vec![0, 0, 0, 1, 1, 0, 0, 0]);
    }
    test(max_depth_after_split);
}
