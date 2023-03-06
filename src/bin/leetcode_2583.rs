//! 二叉树中的第 K 大层和

use leetcode::treenode::TreeNode;
use std::rc::Rc;

use std::cell::RefCell;

pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
    let mut q = vec![root];
    let mut sum = vec![];
    while !q.is_empty() {
        let mut nq = Vec::with_capacity(q.len() * 2);
        let mut c = 0;
        for root in q {
            let node = root.as_ref().unwrap().borrow();
            c += node.val as i64;
            if node.left.is_some() { nq.push(node.left.clone()); }
            if node.right.is_some() { nq.push(node.right.clone()); }
        }
        sum.push(c);
        q = nq;
    }
    sum.sort_unstable();
    let k = k as usize;
    if k > sum.len() { return -1; }
    sum[sum.len() - k]
}

fn main() {
    use leetcode::tree;
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64) {
        assert_eq!(func(tree![5,8,9,2,1,3,7,4,6], 2), 13);
        assert_eq!(func(tree![1,2,null,3], 1), 3);
    }
    test(kth_largest_level_sum);
}
