//! 好叶子节点对的数量

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;

pub fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, distance: usize) -> (Vec<i32>, i32) {
        let mut depths = vec![0; distance + 1];
        let node = root.as_ref().unwrap().borrow();
        let is_leaf = node.left.is_none() && node.right.is_none();
        if is_leaf {
            depths[0] = 1;
            return (depths, 0);
        }
        let mut left_depths = vec![0; distance + 1];
        let mut right_depths = vec![0; distance + 1];
        let mut left_count = 0;
        let mut right_count = 0;
        if node.left.is_some() {
            let r = dfs(node.left.clone(), distance);
            left_depths = r.0;
            left_count = r.1;
        }
        if node.right.is_some() {
            let r = dfs(node.right.clone(), distance);
            right_depths = r.0;
            right_count = r.1;
        }
        for i in 0..distance {
            depths[i + 1] += left_depths[i];
            depths[i + 1] += right_depths[i];
        }

        let mut cnt = 0;
        for i in 0..=distance {
            if i + 2 > distance { continue; }
            for j in 0..=distance - 2 - i {
                cnt += left_depths[i] * right_depths[j];
            }
        }
        return (depths, cnt + left_count + right_count);
    }
    dfs(root, distance as usize).1
}

fn main() {
    assert_eq!(count_pairs(tree![1,2,3,null,4], 3), 1);
    assert_eq!(count_pairs(tree![1,2,3,4,5,6,7], 3), 2);
    assert_eq!(count_pairs(tree![7,1,4,6,null,5,3,null,null,null,null,null,2], 3), 1);
}
