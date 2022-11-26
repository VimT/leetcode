//! 上下翻转二叉树

use leetcode::treenode::TreeNode;
use leetcode::tree;

use std::rc::Rc;
use std::cell::RefCell;

pub fn upside_down_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, parent: Option<Rc<RefCell<TreeNode>>>, head: &mut Option<Rc<RefCell<TreeNode>>>) {
        if root.is_none() { return; }
        let left = root.as_ref().unwrap().borrow().left.clone();
        dfs(left, root.clone(), head);
        if head.is_none() {
            //head置为最左边的节点
            *head = root.clone();
        }
        if parent.is_some() {
            //父节点的left置为null，不会对遍历过程造成影响，因为左边已经全部遍历完成了
            parent.as_ref().unwrap().borrow_mut().left = None;
            //当前节点左节点置为父节点的右节点
            root.as_ref().unwrap().borrow_mut().left = parent.as_ref().unwrap().borrow_mut().right.take();
            //父节点的right置为null，不会对遍历过程造成影响，因为右节点已经在上层进行了记录
            parent.as_ref().unwrap().borrow_mut().right = None;
            //当前节点右节点置为父节点
            root.as_ref().unwrap().borrow_mut().right = parent;
        }
    }
    let mut result = None;
    dfs(root, None, &mut result);
    result
}

fn main() {
    assert_eq!(upside_down_binary_tree(tree![1,2,3,4,5]), tree![4,5,2,null,null,3,1]);
    assert_eq!(upside_down_binary_tree(tree![]), tree![]);
    assert_eq!(upside_down_binary_tree(tree![1]), tree![1]);
}
