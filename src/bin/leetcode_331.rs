//! 验证二叉树的前序序列化

use std::collections::VecDeque;

pub fn is_valid_serialization(preorder: String) -> bool {
    let preorder = preorder.as_bytes();
    let len = preorder.len();
    let mut s = VecDeque::new();
    s.push_back(1);
    let mut i = 0;
    while i < len {
        if s.is_empty() {
            return false;
        }
        if preorder[i] == b',' {
            i += 1;
        } else if preorder[i] == b'#' {
            *s.back_mut().unwrap() -= 1;
            if *s.back().unwrap() == 0 {
                s.pop_back();
            }
            i += 1;
        } else {
            while i < len && preorder[i] != b',' {
                i += 1;
            }
            *s.back_mut().unwrap() -= 1;
            if *s.back().unwrap() == 0 {
                s.pop_back();
            }
            s.push_back(2);
        }
    }
    s.is_empty()
}

fn main() {
    assert_eq!(is_valid_serialization(String::from("9,3,4,#,#,1,#,#,2,#,6,#,#")), true);
    assert_eq!(is_valid_serialization(String::from("1,#")), false);
    assert_eq!(is_valid_serialization(String::from("9,#,#,1")), false);
}
