//! 在受污染的二叉树中查找元素

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use leetcode::treenode::TreeNode;
use leetcode::tree;

struct FindElements {
    set: HashSet<i32>,
}

impl FindElements {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, val: i32, map: &mut HashSet<i32>) {
            if root.is_none() { return; }
            map.insert(val);
            dfs(root.as_ref().unwrap().borrow().left.clone(), val * 2 + 1, map);
            dfs(root.as_ref().unwrap().borrow().right.clone(), val * 2 + 2, map);
        }
        let mut set = HashSet::new();
        dfs(root.clone(), 0, &mut set);
        Self { set }
    }

    fn find(&self, target: i32) -> bool {
        self.set.contains(&target)
    }
}

fn main() {
    let fe = FindElements::new(tree![-1,-1,-1,-1,-1]);
    assert_eq!(fe.find(1), true); // return True
    assert_eq!(fe.find(3), true); // return True
    assert_eq!(fe.find(5), false); // return False
}
