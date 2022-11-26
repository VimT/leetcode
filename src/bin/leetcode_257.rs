//! 二叉树的所有路径

use leetcode::treenode::TreeNode;
use leetcode::tree;

use std::rc::Rc;
use std::cell::RefCell;

pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, cur: &mut Vec<u8>, ans: &mut Vec<String>) {
        if root.is_none() { return; }
        let vs = root.as_ref().unwrap().borrow().val.to_string();
        let cur_len = cur.len();
        cur.extend_from_slice(vs.as_bytes());
        if root.as_ref().unwrap().borrow().left.is_none() && root.as_ref().unwrap().borrow().right.is_none() {
            ans.push(unsafe { String::from_utf8_unchecked(cur.to_vec()) });
            unsafe { cur.set_len(cur_len); }
            return;
        }
        cur.push(b'-');
        cur.push(b'>');
        dfs(root.as_ref().unwrap().borrow().left.clone(), cur, ans);
        dfs(root.as_ref().unwrap().borrow().right.clone(), cur, ans);
        unsafe { cur.set_len(cur_len); }
    }
    let mut ans = vec![];
    dfs(root, &mut vec![], &mut ans);
    ans
}

fn main() {
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String>) {
        assert_eq!(func(tree![1,2,3,null,5]), vec!["1->2->5", "1->3"]);
        assert_eq!(func(tree![1]), vec!["1"]);
    }
    test(binary_tree_paths);
}
