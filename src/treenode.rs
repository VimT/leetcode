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

type TNode = Option<Rc<RefCell<TreeNode>>>;

pub struct NodeTravel(pub TNode);

impl NodeTravel {
    pub fn inorder(&self) -> Vec<i32> {
        fn inner(node: TNode, ans: &mut Vec<i32>) {
            if let Some(n) = node {
                inner(n.borrow().left.clone(), ans);
                ans.push(n.borrow().val);
                inner(n.borrow().right.clone(), ans);
            }
        }
        let mut ans = vec![];
        inner(self.0.clone(), &mut ans);
        ans
    }

    pub fn preorder(&self) -> Vec<i32> {
        fn inner(node: TNode, ans: &mut Vec<i32>) {
            if let Some(n) = node {
                ans.push(n.borrow().val);
                inner(n.borrow().left.clone(), ans);
                inner(n.borrow().right.clone(), ans);
            }
        }
        let mut ans = vec![];
        inner(self.0.clone(), &mut ans);
        ans
    }

    pub fn postorder(&self) -> Vec<i32> {
        fn inner(node: TNode, ans: &mut Vec<i32>) {
            if let Some(n) = node {
                inner(n.borrow().left.clone(), ans);
                inner(n.borrow().right.clone(), ans);
                ans.push(n.borrow().val);
            }
        }
        let mut ans = vec![];
        inner(self.0.clone(), &mut ans);
        ans
    }
}

fn vec_to_tree_inner(arr: &Vec<i32>, idx: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
    if *idx >= arr.len() { return None; }
    let val = arr[*idx];
    if val == 0 { return None; }
    let mut root = TreeNode::new(val);
    *idx += 1;
    root.left = vec_to_tree_inner(arr, idx);
    *idx += 1;
    root.right = vec_to_tree_inner(arr, idx);
    Some(Rc::new(RefCell::new(root)))
}

pub fn vec_to_tree(arr: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    vec_to_tree_inner(&arr, &mut 0)
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
