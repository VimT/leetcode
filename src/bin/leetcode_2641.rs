//! 二叉树的堂兄弟节点 II

use leetcode::treenode::TreeNode;
use std::rc::Rc;

use std::cell::RefCell;

pub fn replace_value_in_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    root.as_ref().unwrap().borrow_mut().val = 0;
    let mut q = vec![root.clone().unwrap()];
    while !q.is_empty() {
        let mut nq = vec![];
        let mut next_level_sum = 0;
        for root in &q {
            let node = root.borrow();
            if node.left.is_some() {
                nq.push(node.left.clone().unwrap());
                next_level_sum += node.left.as_ref().unwrap().borrow().val;
            }
            if node.right.is_some() {
                nq.push(node.right.clone().unwrap());
                next_level_sum += node.right.as_ref().unwrap().borrow().val;
            }
        }
        for root in q {
            let children_sum = root.borrow().left.as_ref().map(|x| x.borrow().val).unwrap_or(0) + root.borrow().right.as_ref().map(|x| x.borrow().val).unwrap_or(0);
            if root.borrow().left.is_some() {
                root.borrow().left.as_ref().unwrap().borrow_mut().val = next_level_sum - children_sum;
            }
            if root.borrow().right.is_some() {
                root.borrow().right.as_ref().unwrap().borrow_mut().val = next_level_sum - children_sum;
            }
        }

        q = nq;
    }
    root
}

fn main() {
    use leetcode::tree;
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>>) {
        assert_eq!(func(tree![763,111,229,null,334,145,null,null,338,674,null,513,193,366,null,365,null,600,null,null,null,null,null,65,926,null,null,null,607]), tree![0,0,0,null,145,334,null,null,674,338,null,366,366,706,null,600,null,365,null,null,null,null,null,0,0,null,null,null,0]);
        assert_eq!(func(tree![5,4,9,1,10,null,7]), tree![0,0,0,7,7,null,11]);
        assert_eq!(func(tree![3,1,2]), tree![0,0,0]);
    }
    test(replace_value_in_tree);
}
