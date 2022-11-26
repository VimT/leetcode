//! 监控二叉树

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;

pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    /// 状态 a：root 必须放置摄像头的情况下，覆盖整棵树需要的摄像头数目。
    /// 状态 b：覆盖整棵树需要的摄像头数目，无论 root 是否放置摄像头。
    /// 状态 c：覆盖两棵子树需要的摄像头数目，无论节点 root 本身是否被监控到。
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
        if root.is_none() { return (i32::MAX / 2, 0, 0); }
        let node = root.as_ref().unwrap().borrow();
        let (la, lb, lc) = dfs(node.left.clone());
        let (ra, rb, rc) = dfs(node.right.clone());
        let a = lc + rc + 1;
        let b = a.min(la + rb).min(ra + lb);
        let c = a.min(lb + rb);
        (a, b, c)
    }
    dfs(root).1
}


fn main() {
    assert_eq!(min_camera_cover(tree![0]), 1);
    assert_eq!(min_camera_cover(tree![0,0,null,0,0]), 1);
    assert_eq!(min_camera_cover(tree![0,0,null,0,null,0,null,null,0]), 2);
}
