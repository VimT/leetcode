//! 面试题 04.03. 特定深度节点链表

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::linknode::ListNode;
use leetcode::treenode::{TreeNode, vec_to_tree};

fn vec_to_link(l: Vec<i32>) -> Option<Box<ListNode>> {
    if l.len() == 0 { return None; }
    let mut head = ListNode::new(l[0]);
    let mut p = &mut head;
    for &i in &l[1..] {
        p.next = Some(Box::new(ListNode::new(i)));
        p = p.next.as_mut().unwrap();
    }
    Some(Box::new(head))
}

pub fn list_of_depth(tree: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<Box<ListNode>>> {
    if tree.is_none() { return vec![]; }
    let mut q = vec![];
    q.push(tree);
    let mut result = vec![];
    while !q.is_empty() {
        let v: Vec<i32> = q.iter().map(|x| x.as_ref().unwrap().borrow().val).collect();
        result.push(vec_to_link(v));
        let mut newq = vec![];
        for node in q {
            let n = node.as_ref().unwrap().borrow();
            if n.left.is_some() {
                newq.push(n.left.clone());
            }
            if n.right.is_some() {
                newq.push(n.right.clone());
            }
        }
        q = newq;
    }
    result
}

fn main() {
    println!("{:?}", list_of_depth(vec_to_tree(vec![1, 2, 4, 8, 0, 0, 0, 5, 0, 0, 3, 0, 7])));
}
