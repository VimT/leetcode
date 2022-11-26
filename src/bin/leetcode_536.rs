//! 从字符串生成二叉树

use leetcode::treenode::TreeNode;
use leetcode::tree;

use std::rc::Rc;
use std::cell::RefCell;

pub fn str2tree(s: String) -> Option<Rc<RefCell<TreeNode>>> {
    fn build(s: &[u8], idx: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
        if *idx >= s.len() { return None; }
        let mut neg = false;
        if s[*idx] == b'-' {
            neg = true;
            *idx += 1;
        }
        let mut num = 0;
        while *idx < s.len() && s[*idx].is_ascii_digit() {
            num = num * 10 + (s[*idx] - b'0') as i32;
            *idx += 1;
        }
        if neg { num = -num; }
        let mut node = TreeNode::new(num);
        if *idx < s.len() && s[*idx] == b'(' {
            *idx += 1;
            node.left = build(s, idx);
            assert_eq!(s[*idx], b')');
            *idx += 1;
        }
        if *idx < s.len() && s[*idx] == b'(' {
            *idx += 1;
            node.right = build(s, idx);
            assert_eq!(s[*idx], b')');
            *idx += 1;
        }
        Some(Rc::new(RefCell::new(node)))
    }
    build(s.as_bytes(), &mut 0)
}

fn main() {
    fn test(func: fn(s: String) -> Option<Rc<RefCell<TreeNode>>>) {
        assert_eq!(func(String::from("4(2(3)(1))(6(5))")), tree![4,2,6,3,1,5]);
        assert_eq!(func(String::from("4(2(3)(1))(6(5)(7))")), tree![4,2,6,3,1,5,7]);
        assert_eq!(func(String::from("-4(2(3)(1))(6(5)(7))")), tree![-4,2,6,3,1,5,7]);
    }
    test(str2tree);
}
