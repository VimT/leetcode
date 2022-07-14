//! 二叉树的最近公共祖先

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::{leetcode_tree, TreeNode};

pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, a: i32, b: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }
        let node = root.as_ref().unwrap().borrow();
        if node.val == a || node.val == b {
            drop(node);
            return root;
        }
        let left = dfs(node.left.clone(), a, b);
        let right = dfs(node.right.clone(), a, b);
        drop(node);

        return if left.is_some() && right.is_some() {
            root
        } else if left.is_some() {
            left
        } else if right.is_some() {
            right
        } else {
            None
        };
    }
    dfs(root, p.as_ref().unwrap().borrow().val, q.as_ref().unwrap().borrow().val)
}


struct UnionSet {
    f: HashMap<i32, i32>,
}

impl UnionSet {
    fn new() -> Self {
        UnionSet {
            f: HashMap::new()
        }
    }

    fn find(&mut self, x: i32) -> i32 {
        return if let Some(&v) = self.f.get(&x) {
            if v != x {
                let result = self.find(v);
                self.f.insert(x, result);
                result
            } else {
                x
            }
        } else {
            self.f.insert(x, x);
            x
        };
    }

    fn union(&mut self, x: i32, y: i32) {
        let xx = self.find(x);
        let yy = self.find(y);
        if xx != yy {
            self.f.insert(yy, xx);
        }
    }
}

pub fn lowest_common_ancestor_tarjan(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, us: &mut UnionSet, vis: &mut HashSet<i32>, query: &HashMap<i32, Vec<i32>>, result: &mut HashMap<i32, Vec<Option<i32>>>) {
        if root.is_none() { return; }
        let node = root.as_ref().unwrap().borrow();
        vis.insert(node.val);
        if node.left.is_some() {
            dfs(node.left.clone(), us, vis, query, result);
            us.union(node.val, node.left.as_ref().unwrap().borrow().val);
        }
        if node.right.is_some() {
            dfs(node.right.clone(), us, vis, query, result);
            us.union(node.val, node.right.as_ref().unwrap().borrow().val);
        }
        if let Some(queries) = query.get(&node.val) {
            result.insert(node.val, queries.iter().map(|&q| {
                if vis.contains(&q) {
                    return Some(us.find(q));
                }
                None
            }).collect());
        }
    }
    let mut us = UnionSet::new();
    let mut query = HashMap::new();
    let a = p.as_ref().unwrap().borrow().val;
    let b = q.as_ref().unwrap().borrow().val;
    query.insert(a, vec![b]);
    query.insert(b, vec![a]);
    let mut result = HashMap::new();
    dfs(root, &mut us, &mut HashSet::new(), &query, &mut result);
    return if let Some(v) = result[&a][0] {
        Some(Rc::new(RefCell::new(TreeNode::new(v))))
    } else {
        Some(Rc::new(RefCell::new(TreeNode::new(result[&b][0].unwrap()))))
    };
}


fn main() {
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>>) {
        assert_eq!(func(tree![3,5,1,6,2,0,8,null,null,7,4], tree![5], tree![1]).as_ref().unwrap().borrow().val, 3);
        assert_eq!(func(tree![3,5,1,6,2,0,8,null,null,7,4], tree![5], tree![4]).as_ref().unwrap().borrow().val, 5);
        assert_eq!(func(tree![1,2], tree![1], tree![2]).as_ref().unwrap().borrow().val, 1);
    }
    test(lowest_common_ancestor);
    test(lowest_common_ancestor_tarjan);
}
