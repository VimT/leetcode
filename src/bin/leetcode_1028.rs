//! 从先序遍历还原二叉树

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::{leetcode_tree, TreeNode};

pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
    fn dfs(traversal: &[u8], idx: &mut usize, cur_level: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if *idx == traversal.len() { return None; }
        let mut num = 0;
        while *idx < traversal.len() && traversal[*idx] != b'-' {
            num = num * 10 + (traversal[*idx] - b'0') as i32;
            *idx += 1;
        }
        let mut node = TreeNode::new(num);
        let mut next_level_idx = *idx;
        while next_level_idx < traversal.len() && traversal[next_level_idx] == b'-' {
            next_level_idx += 1;
        }
        let mut next_level = next_level_idx - *idx;
        if next_level <= cur_level {
            return Some(Rc::new(RefCell::new(node)));
        }
        *idx = next_level_idx;
        let child = dfs(traversal, idx, next_level);
        node.left = child;
        next_level_idx = *idx;
        while next_level_idx < traversal.len() && traversal[next_level_idx] == b'-' {
            next_level_idx += 1;
        }
        next_level = next_level_idx - *idx;
        if next_level <= cur_level {
            return Some(Rc::new(RefCell::new(node)));
        }
        *idx = next_level_idx;
        let child2 = dfs(traversal, idx, next_level);
        node.right = child2;
        Some(Rc::new(RefCell::new(node)))
    }
    dfs(traversal.as_bytes(), &mut 0, 0)
}

pub fn recover_from_preorder_stack(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
    let s = traversal.as_bytes();
    let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![];
    let mut i = 0;
    let len = s.len();
    while i < len {
        let mut level = 0;
        while i < len && s[i] == b'-' {
            level += 1;
            i += 1;
        }
        let mut num = 0;
        while i < len && s[i] != b'-' {
            num = num * 10 + (s[i] - b'0') as i32;
            i += 1;
        }
        let node = Rc::new(RefCell::new(TreeNode::new(num)));
        if level == stack.len() {
            if !stack.is_empty() {
                stack.last().unwrap().borrow_mut().left = Some(node.clone());
            }
        } else {
            // !
            while level != stack.len() {
                stack.pop();
            }
            stack.last().unwrap().borrow_mut().right = Some(node.clone());
        }
        stack.push(node);
    }
    Some(stack.first().unwrap().clone())
}

fn main() {
    fn test(func: fn(traversal: String) -> Option<Rc<RefCell<TreeNode>>>) {
        assert_eq!(func(String::from("1-2--3--4-5--6--7")), tree![1,2,5,3,4,6,7]);
        assert_eq!(func(String::from("1-2--3---4-5--6---7")), tree![1,2,5,3,null,6,null,4,null,7]);
        assert_eq!(func(String::from("1-401--349---90--88")), tree![1,401,null,349,88,90]);
    }
    test(recover_from_preorder);
    test(recover_from_preorder_stack);
}
