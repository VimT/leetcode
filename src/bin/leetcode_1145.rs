//! 二叉树着色游戏

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;

pub fn btree_game_winning_move(root: Option<Rc<RefCell<TreeNode>>>, n: i32, x: i32) -> bool {
    let mut left_cnt = 0;
    let mut right_cnt = 0;
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, x: i32, left_cnt: &mut i32, right_cnt: &mut i32) -> i32 {
        if root.is_none() { return 0; }
        let node = root.as_ref().unwrap().borrow();
        let left = dfs(node.left.clone(), x, left_cnt, right_cnt);
        let right = dfs(node.right.clone(), x, left_cnt, right_cnt);
        if node.val == x {
            *left_cnt = left;
            *right_cnt = right;
        }
        left + right + 1
    }
    dfs(root, x, &mut left_cnt, &mut right_cnt);
    let parent_cnt = n - left_cnt - right_cnt - 1;
    parent_cnt > n / 2 || left_cnt > n / 2 || right_cnt > n / 2
}

fn main() {
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>, n: i32, x: i32) -> bool) {
        assert_eq!(func(tree![1,2,3,4,5,6,7,8,9,10,11], 11, 3), true);
        assert_eq!(func(tree![1,2,3], 3, 1), false);
    }
    test(btree_game_winning_move);
}
