//! 逐层排序二叉树所需的最少操作数目

use leetcode::treenode::TreeNode;
use leetcode::tree;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

pub fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut q = vec![root];
    let mut result = 0;
    while !q.is_empty() {
        let mut vals: Vec<i32> = q.iter().map(|x| x.as_ref().unwrap().borrow().val).collect();
        let mut sorted_vals = vals.clone();
        sorted_vals.sort_unstable();
        let map: HashMap<i32, usize> = sorted_vals.iter().enumerate().map(|(i, v)| (*v, i)).collect();
        for i in 0..vals.len() {
            loop { // !
                let should_i = map[&vals[i]];
                if should_i != i {
                    vals.swap(should_i, i);
                    result += 1;
                } else { break; }
            }
        }

        // 无序变有序求交换次数，另一种做法：
        // 先离散化，再对每个数，找环，操作次数=len-环数=每个环节点数-1
        // 例如：4 3 2 1，环有 4-1, 3-2，那么就是操作两次
        //      4 1 2 3，环是 4-3-2-1，那么就是操作3次

        let mut nq = Vec::with_capacity(q.len() * 2);
        for root in q {
            let node = root.as_ref().unwrap().borrow();
            if node.left.is_some() {
                nq.push(node.left.clone());
            }
            if node.right.is_some() {
                nq.push(node.right.clone());
            }
        }
        q = nq;
    }
    result
}

fn main() {
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>) -> i32) {
        assert_eq!(func(tree![332,463,103,417,150,409,41,135,129,117,474,263,null,328,456,347,167,383,null,null,422,493,489,275,72,null,null,425,89,null,null,162,18,null,null,null,null,363,290,106,260,468,null,null,null,432,null,323,null,null,null,null,null,null,36,null,null,302,190,null,280,null,null,null,null,488,null,null,null,null,446,null,null,null,null,null,75]), 24);
        assert_eq!(func(tree![1,4,3,7,6,8,5,null,null,null,null,9,null,10]), 3);
        assert_eq!(func(tree![1,3,2,7,6,5,4]), 3);
        assert_eq!(func(tree![1,2,3,4,5,6]), 0);
    }
    test(minimum_operations);
}
