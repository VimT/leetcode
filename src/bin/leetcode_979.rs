//! 在二叉树中分配硬币

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;

/**
 * 从后序遍历的第一个叶子节点开始，假设自己有x个金币，剩余x-1个金币都还给父节点，x-1可能为负数、0、正数
 * x-1 < 0说明不够金币，需要从父节点获得，因此子节点有|x-1|个入方向的操作，次数加上|x-1|
 * x-1 == 0说明刚好，无需与父节点有金币的交换，次数加0
 * x-1 > 0 说明有多余的金币，需要交给父节点，因此子节点有x-1个出方向的操作，次数加上|x-1|
 */
pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, result: &mut i32) -> i32 {
        if root.is_none() { return 0; }
        let mut node = root.as_ref().unwrap().borrow_mut();
        if node.left.is_some() {
            node.val += dfs(node.left.clone(), result);
        }
        if node.right.is_some() {
            node.val += dfs(node.right.clone(), result);
        }
        *result += (node.val - 1).abs();
        node.val - 1
    }
    let mut result = 0;
    dfs(root, &mut result);
    result
}


fn main() {
    assert_eq!(distribute_coins(tree![3,0,0]), 2);
    assert_eq!(distribute_coins(tree![0,3,0]), 3);
}
