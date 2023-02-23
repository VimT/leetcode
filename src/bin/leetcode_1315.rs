//! 祖父节点值为偶数的节点和

use leetcode::treenode::TreeNode;
use std::rc::Rc;

use std::cell::RefCell;

pub fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, s: &mut Vec<i32>, result: &mut i32) {
        if root.is_none() { return; }
        let node = root.as_ref().unwrap().borrow();
        if s.len() > 1 && s[s.len() - 2] & 1 == 0 {
            *result += node.val;
        }
        s.push(node.val);
        dfs(node.left.clone(), s, result);
        dfs(node.right.clone(), s, result);
        s.pop();
    }
    let mut result = 0;
    dfs(root, &mut vec![], &mut result);
    result
}

fn main() {
    use leetcode::tree;
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>) -> i32) {
        assert_eq!(func(tree![6,7,8,2,7,1,3,9,null,1,4,null,null,null,5]), 18);
        assert_eq!(func(tree![1]), 0);
    }
    test(sum_even_grandparent);
}
