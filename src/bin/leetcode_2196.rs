//! 根据描述创建二叉树

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;

pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut in_degree = HashMap::new();
    let mut node = HashMap::new();
    for desc in descriptions {
        let (parent, child, is_left) = (desc[0], desc[1], desc[2]);
        let c = node.entry(child).or_insert(Some(Rc::new(RefCell::new(TreeNode::new(child))))).clone();
        let mut p = node.entry(parent).or_insert(Some(Rc::new(RefCell::new(TreeNode::new(parent))))).as_ref().unwrap().borrow_mut();
        if is_left == 1 { p.left = c.clone(); } else { p.right = c.clone(); }
        *in_degree.entry(child).or_insert(0) += 1;
        *in_degree.entry(parent).or_insert(0) += 0;
    }
    for (k, v) in in_degree {
        if v == 0 {
            return node[&k].clone();
        }
    }
    unreachable!()
}

fn main() {
    assert_eq!(create_binary_tree(vec![vec![20, 15, 1], vec![20, 17, 0], vec![50, 20, 1], vec![50, 80, 0], vec![80, 19, 1]]), tree![50,20,80,15,17,19]);
    assert_eq!(create_binary_tree(vec![vec![1, 2, 1], vec![2, 3, 0], vec![3, 4, 1]]), tree![1,2,null,null,3,4]);
}
