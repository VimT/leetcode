//! 合并多棵二叉搜索树

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;

pub fn can_merge(trees: Vec<Option<Rc<RefCell<TreeNode>>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut leave = HashSet::new();
    let mut map = HashMap::new();
    for tree in &trees {
        if let Some(v) = tree {
            let p = v.borrow();
            if let Some(left) = &p.left {
                leave.insert(left.borrow().val);
            }
            if let Some(right) = &p.right {
                leave.insert(right.borrow().val);
            }
            map.insert(p.val, v.clone());
        }
    }
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, map: &mut HashMap<i32, Rc<RefCell<TreeNode>>>, prev: &mut i32) -> bool {
        if root.is_none() { return true; }
        let mut p = root.as_ref().unwrap().borrow_mut();
        let mid = p.val;
        if p.left.is_none() && p.right.is_none() {
            if let Some(node) = map.remove(&mid) {
                p.left = node.borrow().left.clone();
                p.right = node.borrow().right.clone();
            }
        }
        if !dfs(p.left.clone(), map, prev) {
            return false;
        }
        if mid <= *prev {
            return false;
        }
        *prev = mid;
        dfs(p.right.clone(), map, prev)
    }

    for tree in trees {
        if !leave.contains(&tree.as_ref().unwrap().borrow().val) {
            map.remove(&tree.as_ref().unwrap().borrow().val);
            let result = dfs(tree.clone(), &mut map, &mut 0);
            return if result && map.is_empty() { tree } else { None };
        }
    }
    None
}


fn main() {
    assert_eq!(can_merge(vec![tree![2,1], tree![3,2,5], tree![5,4]]), tree![3,2,5,1,null,4]);
    assert_eq!(can_merge(vec![tree![5,3,8], tree![3,2,6]]), tree![]);
    assert_eq!(can_merge(vec![tree![5,4], tree![3]]), tree![]);
}
