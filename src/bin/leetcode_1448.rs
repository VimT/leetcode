//! 统计二叉树中好节点的数目

use leetcode::treenode::TreeNode;
use std::rc::Rc;

use std::cell::RefCell;

pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, mut cur_max: i32, result: &mut i32) {
        if root.is_none() { return; }
        let node = root.as_ref().unwrap().borrow();
        if node.val >= cur_max {
            *result += 1;
            cur_max = node.val;
        }
        dfs(node.left.clone(), cur_max, result);
        dfs(node.right.clone(), cur_max, result);
    }
    let mut result = 0;
    dfs(root, i32::MIN, &mut result);
    result
}

fn main() {
    use leetcode::tree;
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>) -> i32) {
        assert_eq!(func(tree![3,1,4,3,null,1,5]), 4);
        assert_eq!(func(tree![3,3,null,4,2]), 3);
        assert_eq!(func(tree![1]), 1);
    }
    test(good_nodes);
}
