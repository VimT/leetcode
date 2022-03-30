//! binary-search-tree-iterator

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use leetcode::treenode::{TreeNode, vec_to_tree};

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

    /** @return the next smallest number */
    fn next(&mut self) -> i32 {
        let node = self.stack.pop_back().unwrap();
        let nw = node.as_ref().unwrap().borrow();
        if nw.right.is_some() {
            self.leftmost_inorder(nw.right.clone());
        }
        nw.val
    }

    /** @return whether we have a next smallest number */
    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}

fn main() {
    let mut iterator = BSTIterator::new(vec_to_tree(vec![7, 3, 0, 0, 15, 9, 0, 0, 20]));
    println!("{}", iterator.next());    // 3
    println!("{}", iterator.next());    // 7
    println!("{}", iterator.has_next()); // true
    println!("{}", iterator.next());    // 9
    println!("{}", iterator.has_next()); // true
    println!("{}", iterator.next());    // 15
    println!("{}", iterator.has_next()); // true
    println!("{}", iterator.next());    // 20
    println!("{}", iterator.has_next()); // false
}
