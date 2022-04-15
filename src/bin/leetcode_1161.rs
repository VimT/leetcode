//! 最大层内元素和

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::{leetcode_tree, TreeNode};

pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max = root.as_ref().unwrap().borrow().val;
    let mut result = 1;
    let mut cur = 1;
    let mut q = vec![root];
    while !q.is_empty() {
        let mut nq = Vec::with_capacity(q.len() * 2);
        let mut sum = 0;
        for node in q {
            let n = node.as_ref().unwrap().borrow();
            sum += n.val;
            if n.left.is_some() {
                nq.push(n.left.clone());
            }
            if n.right.is_some() {
                nq.push(n.right.clone());
            }
        }
        if sum > max {
            max = sum;
            result = cur;
        }
        q = nq;
        cur += 1;
    }
    result
}

fn main() {
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>) -> i32) {
        assert_eq!(func(tree![1,7,0,7,-8,null,null]), 2);
        assert_eq!(func(tree![989,null,10250,98693,-89388,null,null,null,-32127]), 2);
    }
    test(max_level_sum);
}
