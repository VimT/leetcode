//! 层数最深叶子节点的和

use leetcode::treenode::TreeNode;
use std::rc::Rc;

use std::cell::RefCell;

pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut q = vec![root];
    loop {
        let mut nq = Vec::with_capacity(q.len() * 2);
        let mut sum = 0;
        for node in q {
            let node = node.as_ref().unwrap().borrow();
            sum += node.val;
            if node.left.is_some() {
                nq.push(node.left.clone());
            }
            if node.right.is_some() {
                nq.push(node.right.clone());
            }
        }
        if nq.is_empty() {
            return sum;
        }
        q = nq;
    }
}

fn main() {
    use leetcode::tree;
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>) -> i32) {
        assert_eq!(func(tree![1,2,3,4,5,null,6,7,null,null,null,null,8]), 15);
        assert_eq!(func(tree![6,7,8,2,7,1,3,9,null,1,4,null,null,null,5]), 19);
    }
    test(deepest_leaves_sum);
}
