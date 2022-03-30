//! 括号生成

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::treenode::{NodeTravel, TreeNode};

pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn inner(preorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.len() == 0 { return None; }
        let val = preorder[0];
        let mut node = TreeNode::new(val);
        let mut idx = 1;
        while idx < preorder.len() && preorder[idx] < val {
            idx += 1;
        }
        node.left = inner(&preorder[1..idx]);
        node.right = inner(&preorder[idx..]);
        Some(Rc::new(RefCell::new(node)))
    }
    inner(&preorder)
}

fn main() {
    assert_eq!(NodeTravel(bst_from_preorder(vec![8, 5, 1, 7, 10, 12])).preorder(), vec![8, 5, 1, 7, 10, 12]);

    println!("{}", largest_sum_after_k_negations(vec![2, -3, -1, 5, -4], 2));
}

pub fn largest_sum_after_k_negations(mut nums: Vec<i32>, mut k: i32) -> i32 {
    let len = nums.len();
    nums.sort_unstable_by_key(|x| -x.abs());
    for i in &mut nums {
        if *i < 0 {
            *i = -*i;
            k -= 1;
            if k == 0 {
                break;
            }
        }
    }
    if k > 0 && k & 1 == 1 {
        nums[len - 1] = -nums[len - 1];
    }
    nums.into_iter().sum()
}