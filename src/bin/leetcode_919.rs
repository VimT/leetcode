//! 完全二叉树插入器

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;

struct CBTInserter {
    root: Option<Rc<RefCell<TreeNode>>>,
    queue: VecDeque<Option<Rc<RefCell<TreeNode>>>>,
}

impl CBTInserter {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut q = VecDeque::new();
        q.push_back(root.clone());
        loop {
            let node = q.front().unwrap().clone();
            let nb = node.as_ref().unwrap().borrow();
            if nb.left.is_some() {
                q.push_back(nb.left.clone());
            }
            if nb.right.is_some() {
                q.push_back(nb.right.clone());
            }
            if nb.left.is_none() || nb.right.is_none() {
                return Self { root, queue: q };
            }
            q.pop_front().unwrap();
        }
    }

    fn insert(&mut self, v: i32) -> i32 {
        let new_node = Some(Rc::new(RefCell::new(TreeNode::new(v))));
        let parent = self.queue.front().unwrap();
        let val = parent.as_ref().unwrap().borrow().val;

        // 完全二叉树的队首就是队尾的父节点
        if parent.as_ref().unwrap().borrow().left.is_none() {
            parent.as_ref().unwrap().borrow_mut().left = new_node.clone();
        } else {
            parent.as_ref().unwrap().borrow_mut().right = new_node.clone();
            self.queue.pop_front();
        }
        self.queue.push_back(new_node);
        val
    }

    fn get_root(&self) -> Option<Rc<RefCell<TreeNode>>> {
        self.root.clone()
    }
}

fn main() {
    let mut ci = CBTInserter::new(tree![1]);
    assert_eq!(ci.insert(2), 1);
    assert_eq!(ci.get_root(), tree![1,2]);

    ci = CBTInserter::new(tree![1,2,3,4,5,6]);
    assert_eq!(ci.insert(7), 3);
    assert_eq!(ci.insert(8), 4);
    assert_eq!(ci.get_root(), tree![1,2,3,4,5,6,7,8])
}