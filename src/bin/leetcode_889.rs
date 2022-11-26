//! 根据前序和后序遍历构造二叉树

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;

pub fn construct_from_pre_post(preorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn dfs(preorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() { return None; }
        let mid = preorder[0];
        let mut node = TreeNode::new(mid);
        if preorder.len() > 1 {
            let left_first = preorder[1];
            // 取一个长度就行了
            let left_len = postorder.iter().position(|x| *x == left_first).unwrap() + 1;
            node.left = dfs(&preorder[1..=left_len], &postorder[..left_len]);
            node.right = dfs(&preorder[left_len + 1..], &postorder[left_len..postorder.len() - 1]);
        }
        Some(Rc::new(RefCell::new(node)))
    }
    dfs(&preorder, &postorder)
}


fn main() {
    assert_eq!(construct_from_pre_post(vec![2, 1, 3], vec![3, 1, 2]), tree![2,1,null,3]);
    assert_eq!(construct_from_pre_post(vec![1, 2, 4, 5, 3, 6, 7], vec![4, 5, 2, 6, 7, 3, 1]), tree![1,2,3,4,5,6,7]);
    assert_eq!(construct_from_pre_post(vec![1], vec![1]), tree![1]);
}
