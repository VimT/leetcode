//! 二叉搜索树的最近公共祖先

use leetcode::treenode::TreeNode;
use leetcode::tree;

use std::rc::Rc;
use std::cell::RefCell;

pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut a = root.clone();
    loop {
        if p.as_ref().unwrap().borrow().val < a.as_ref().unwrap().borrow().val && q.as_ref().unwrap().borrow().val < a.as_ref().unwrap().borrow().val {
            let tmp = a.as_ref().unwrap().borrow().left.clone();
            a = tmp;
        } else if p.as_ref().unwrap().borrow().val > a.as_ref().unwrap().borrow().val && q.as_ref().unwrap().borrow().val > a.as_ref().unwrap().borrow().val {
            let tmp = a.as_ref().unwrap().borrow().right.clone();
            a = tmp;
        } else {
            break;
        }
    }
    a
}

fn main() {
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>>) {
        assert_eq!(func(tree![6,2,8,0,4,7,9,null,null,3,5], tree![2], tree![8]).as_ref().unwrap().borrow().val, 6);
        assert_eq!(func(tree![6,2,8,0,4,7,9,null,null,3,5], tree![2], tree![4]).as_ref().unwrap().borrow().val, 2);
        assert_eq!(func(tree![2,1], tree![2], tree![1]).as_ref().unwrap().borrow().val, 2);
    }
    test(lowest_common_ancestor);
}
