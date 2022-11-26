//! 二叉树的堂兄弟节点

use std::rc::Rc;
use std::cell::RefCell;
use leetcode::tree;
use leetcode::treenode::TreeNode;

pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32, depth: i32, father: i32, xd: &mut i32, yd: &mut i32, xf: &mut i32, yf: &mut i32) {
        if root.is_none() { return; }
        let node = root.as_ref().unwrap().borrow();
        if node.val == x {
            *xd = depth;
            *xf = father;
        }
        if node.val == y {
            *yd = depth;
            *yf = father;
        }
        if node.left.is_some() {
            dfs(node.left.clone(), x, y, depth + 1, node.val, xd, yd, xf, yf);
        }
        if node.right.is_some() {
            dfs(node.right.clone(), x, y, depth + 1, node.val, xd, yd, xf, yf);
        }
    }
    let mut x_depth = 0;
    let mut y_depth = 0;
    let mut x_father = -1;
    let mut y_father = -1;
    dfs(root, x, y, 0, -1, &mut x_depth, &mut y_depth, &mut x_father, &mut y_father);
    x_depth == y_depth && (x_father != y_father)
}


fn main() {
    assert_eq!(is_cousins(tree![1,2,3,4], 4, 3), false);
    assert_eq!(is_cousins(tree![1,2,3,null,4,null,5], 5, 4), true);
    assert_eq!(is_cousins(tree![1,2,3,null,4], 2, 3), false);
}
