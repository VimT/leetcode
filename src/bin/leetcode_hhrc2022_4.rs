//! 补给覆盖

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;

/// 树形DP，自底向上
/// 返回 (自己覆盖，自己或子节点覆盖，自己或父节点覆盖)
pub fn min_supply_station_number(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn walk(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
        if root.is_none() { return (i32::MAX / 2, 0, 0); }
        let node = root.as_ref().unwrap().borrow();
        let (l0, l1, l2) = walk(node.left.clone());
        let (r0, r1, r2) = walk(node.right.clone());
        let ret0 = l2 + r2 + 1;
        let ret1 = ret0.min(l0 + r1).min(l1 + r0);
        let ret2 = ret0.min(l1 + r1);
        (ret0, ret1, ret2)
    }

    let result = walk(root);
    result.1
}

fn main() {
    assert_eq!(min_supply_station_number(tree![0,0,0]), 1);
    assert_eq!(min_supply_station_number(tree![0,0,0,null,null,null,0]), 2);
}
