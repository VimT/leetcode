use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

#[derive(PartialEq, Eq, Default, Ord, PartialOrd)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl Debug for TreeNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}, {:?}, {:?})", self.val, self.left, self.right))
    }
}


pub fn leetcode_tree(input: &str) -> Option<Rc<RefCell<TreeNode>>> {
    let input: String = input.trim_matches(|x| x == '[' || x == ']').chars().filter(|x| !x.is_ascii_whitespace()).collect();
    if input.is_empty() { return None; }
    let nodes: Vec<Option<i32>> = input.split(',').map(|x| if x == "null" { None } else { Some(x.parse::<i32>().unwrap()) }).collect();
    if nodes.is_empty() || nodes[0].is_none() { return None; }
    let root = Some(Rc::new(RefCell::new(TreeNode::new(nodes[0].unwrap()))));
    let mut q = vec![root.clone()];
    let mut i = 1;
    let len = nodes.len();
    while i < len {
        let mut newq = vec![];
        for j in 0..q.len() {
            if let Some(v) = q[j].as_mut() {
                let mut node = v.borrow_mut();
                node.left = if i < len && nodes[i].is_some() { Some(Rc::new(RefCell::new(TreeNode::new(nodes[i].unwrap())))) } else { None };
                i += 1;
                node.right = if i < len && nodes[i].is_some() { Some(Rc::new(RefCell::new(TreeNode::new(nodes[i].unwrap())))) } else { None };
                i += 1;
                if node.left.is_some() {
                    newq.push(node.left.clone());
                }
                if node.right.is_some() {
                    newq.push(node.right.clone());
                }
            }
        }
        q = newq;
    }
    root
}

#[test]
fn test_equal() {
    let n1 = vec![1, 3, 0, 0, 2];
    let n2 = n1.clone();
    let n3 = vec![1, 3, 0, 0, 2];
    println!("{}", n1 == n2);
    println!("{}", n1 == n3);
}
