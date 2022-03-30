//! 从二叉树一个节点到另一个节点每一步的方向

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::treenode::{TreeNode, vec_to_tree};

pub fn get_directions(root: Option<Rc<RefCell<TreeNode>>>, start_value: i32, dest_value: i32) -> String {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, start_value: i32, dest_value: i32, cur: &mut Vec<u8>, start_road: &mut Vec<u8>, dest_road: &mut Vec<u8>) {
        if root.is_none() { return; }
        let node = root.as_ref().unwrap().borrow();
        if node.val == start_value {
            *start_road = cur.clone();
        } else if node.val == dest_value {
            *dest_road = cur.clone();
        }
        if node.left.is_some() {
            cur.push(b'L');
            dfs(node.left.clone(), start_value, dest_value, cur, start_road, dest_road);
            cur.pop();
        }
        if node.right.is_some() {
            cur.push(b'R');
            dfs(node.right.clone(), start_value, dest_value, cur, start_road, dest_road);
            cur.pop();
        }
    }
    let mut start_road = vec![];
    let mut dest_road = vec![];
    dfs(root, start_value, dest_value, &mut vec![], &mut start_road, &mut dest_road);
    let mut i = 0;
    while i < start_road.len() && i < dest_road.len() && start_road[i] == dest_road[i] {
        i += 1;
    }
    let mut result = vec![b'U'; start_road.len() - i];
    result.extend_from_slice(&dest_road[i..]);
    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    assert_eq!(get_directions(vec_to_tree(vec![5, 1, 3, 0, 0, 0, 2, 6, 0, 0, 4]), 3, 6), "UURL");
    assert_eq!(get_directions(vec_to_tree(vec![2, 1]), 2, 1), "L");
}
