//! binary-search-tree-iterator

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::{TreeNode, leetcode_tree};

type Tree = Option<Rc<RefCell<TreeNode>>>;

struct BSTIterator {
    stack: VecDeque<Tree>,
}


impl BSTIterator {
    fn new(root: Tree) -> Self {
        let mut ans = BSTIterator { stack: VecDeque::new() };
        ans.leftmost_inorder(root);
        ans
    }

    fn leftmost_inorder(&mut self, mut root: Tree) {
        while root.is_some() {
            self.stack.push_back(root.clone());
            let tmp = root.as_ref().unwrap().borrow().left.clone();
            root = tmp;
        }
    }

    fn next(&mut self) -> i32 {
        let node = self.stack.pop_back().unwrap();
        let nw = node.as_ref().unwrap().borrow();
        if nw.right.is_some() {
            self.leftmost_inorder(nw.right.clone());
        }
        nw.val
    }

    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}

fn main() {
    let mut iterator = BSTIterator::new(tree![7, 3, 15, null, null, 9, 20]);
    assert_eq!(iterator.next(), 3);
    assert_eq!(iterator.next(), 7);
    assert_eq!(iterator.has_next(), true);
    assert_eq!(iterator.next(), 9);
    assert_eq!(iterator.has_next(), true);
    assert_eq!(iterator.next(), 15);
    assert_eq!(iterator.has_next(), true);
    assert_eq!(iterator.next(), 20);
    assert_eq!(iterator.has_next(), false);
}
