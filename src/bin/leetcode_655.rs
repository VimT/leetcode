//! 输出二叉树


use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::{leetcode_tree, TreeNode};

pub fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<String>> {
    fn get_height(root: Option<Rc<RefCell<TreeNode>>>) -> usize {
        if root.is_none() { return 0; }
        1 + get_height(root.as_ref().unwrap().borrow().left.clone()).max(get_height(root.as_ref().unwrap().borrow().right.clone()))
    }
    fn fill(root: Option<Rc<RefCell<TreeNode>>>, table: &mut Vec<Vec<String>>, line: usize, l: usize, r: usize) {
        if root.is_none() { return; }
        let node = root.as_ref().unwrap().borrow();
        table[line][(l + r) / 2] = node.val.to_string();
        fill(node.left.clone(), table, line + 1, l, (l + r) / 2);
        fill(node.right.clone(), table, line + 1, (l + r + 1) / 2, r);
    }
    let height = get_height(root.clone());
    let mut result = vec![vec![String::new(); (1 << height) - 1]; height];
    fill(root, &mut result, 0, 0, (1 << height) - 1);
    result
}

fn main() {
    assert_eq!(print_tree(tree![1,2]), [["", "1", ""], ["2", "", ""]]);
    assert_eq!(print_tree(tree![1,2,3,null,4]), [["", "", "", "1", "", "", ""], ["", "2", "", "", "", "3", ""], ["", "", "4", "", "", "", ""]]);
}
