//! 寻找重复的子树

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;
use leetcode::unorder;

pub fn find_duplicate_subtrees(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, map: &mut HashMap<Vec<i32>, (i32, Option<Rc<RefCell<TreeNode>>>)>) -> Vec<i32> {
        if let Some(node) = root.as_ref() {
            let mut result = vec![node.borrow().val];
            let left = dfs(node.borrow().left.clone(), map);
            let right = dfs(node.borrow().right.clone(), map);
            result.extend_from_slice(&left);
            result.extend_from_slice(&right);
            if map.contains_key(&result) {
                map.get_mut(&result).unwrap().0 += 1;
            } else {
                map.insert(result.clone(), (1, root.clone()));
            }
            return result;
        }
        vec![i32::MAX]
    }
    let mut map = HashMap::new();
    dfs(root, &mut map);
    let mut result = vec![];
    for (_, v) in map {
        if v.0 > 1 {
            result.push(v.1.clone());
        }
    }
    result
}

fn main() {
    assert_eq!(unorder(find_duplicate_subtrees(tree![1,2,3,4,null,2,4,null,null,4])), vec![tree![2,4], tree![4]]);
    assert_eq!(unorder(find_duplicate_subtrees(tree![2,1,1])), vec![tree![1]]);
    assert_eq!(unorder(find_duplicate_subtrees(tree![2,2,2,3,null,3,null])), vec![tree![2,3], tree![3]]);
}
