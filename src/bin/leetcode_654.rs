//! 最大二叉树


use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;

struct SegmentTree<'a> {
    max: Vec<usize>,
    nums: &'a Vec<i32>,
}

impl<'a> SegmentTree<'a> {
    fn new(nums: &'a Vec<i32>) -> Self {
        let mut tree = Self { max: vec![0; 4 * nums.len()], nums };
        tree.build(0, nums.len() - 1, 0);
        tree
    }

    fn build(&mut self, s: usize, t: usize, p: usize) {
        if s == t {
            self.max[p] = s;
            return;
        }
        let m = (s + t) / 2;
        self.build(s, m, p * 2 + 1);
        self.build(m + 1, t, p * 2 + 2);
        self.max[p] = if self.nums[self.max[p * 2 + 1]] > self.nums[self.max[p * 2 + 2]] { self.max[p * 2 + 1] } else { self.max[p * 2 + 2] };
    }

    fn ask_inner(&self, l: usize, r: usize, s: usize, t: usize, p: usize) -> usize {
        if l <= s && r >= t {
            return self.max[p];
        }
        let m = (s + t) / 2;
        let mut max = l;
        if l <= m {
            let sub = self.ask_inner(l, r, s, m, p * 2 + 1);
            if self.nums[sub] > self.nums[max] {
                max = sub;
            }
        }
        if r > m {
            let sub = self.ask_inner(l, r, m + 1, t, p * 2 + 2);
            if self.nums[sub] > self.nums[max] {
                max = sub;
            }
        }
        max
    }

    fn ask(&self, l: usize, r: usize) -> usize {
        self.ask_inner(l, r, 0, self.nums.len() - 1, 0)
    }
}

pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let tree = SegmentTree::new(&nums);
    fn dfs(nums: &Vec<i32>, tree: &SegmentTree, l: usize, r: usize) -> Option<Rc<RefCell<TreeNode>>> {
        let max_idx = tree.ask(l, r);
        let mut node = TreeNode::new(nums[max_idx]);
        if max_idx > l { node.left = dfs(nums, tree, l, max_idx - 1) };
        if max_idx < r { node.right = dfs(nums, tree, max_idx + 1, r) };
        Some(Rc::new(RefCell::new(node)))
    }
    dfs(&nums, &tree, 0, nums.len() - 1)
}

fn main() {
    assert_eq!(construct_maximum_binary_tree(vec![1]), tree![1]);
    assert_eq!(construct_maximum_binary_tree(vec![3, 2, 1, 6, 0, 5]), tree![6,3,5,null,2,0,null,null,1]);
    assert_eq!(construct_maximum_binary_tree(vec![3, 2, 1]), tree![3,null,2,null,1]);
}
