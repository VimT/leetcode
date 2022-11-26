//! 有序链表转换二叉搜索树

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::link;
use leetcode::linknode::ListNode;
use leetcode::tree;
use leetcode::treenode::TreeNode;

/// 获取一次长度就可以，不用转数组
pub fn sorted_list_to_bst(mut head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut arr = vec![];
    while head.is_some() {
        arr.push(head.as_ref().unwrap().val);
        head = head.unwrap().next.take();
    }
    fn build_tree(arr: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if arr.len() == 0 { return None; }
        let mid = arr.len() / 2;
        let mut node = TreeNode::new(arr[mid]);
        node.left = build_tree(&arr[..mid]);
        node.right = build_tree(&arr[mid + 1..]);
        return Some(Rc::new(RefCell::new(node)));
    }

    build_tree(&arr)
}

fn main() {
    assert_eq!(sorted_list_to_bst(link![-10,-3,0,5,9]), tree![0,-3,9,-10,null,5]);
    assert_eq!(sorted_list_to_bst(link![]), tree![]);
}
