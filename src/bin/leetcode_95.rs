//! 二叉树的中序遍历

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::treenode::TreeNode;

type Node = Option<Rc<RefCell<TreeNode>>>;

pub fn num_trees(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![1; n + 1];
    for i in 2..=n {
        let mut tmp = 0;
        for mid in 0..i {
            tmp += dp[mid] * dp[i - mid - 1];
        }
        dp[i] = tmp;
    }
    dp[n]
}

pub fn generate_trees(n: i32) -> Vec<Node> {
    fn inner(values: &[i32]) -> Vec<Node> {
        if values.len() == 0 { return vec![None]; }
        let mut ans = vec![];
        for i in 0..values.len() {
            let root_val = values[i];
            let left = inner(&values[..i]);
            let right = inner(&values[i + 1..]);
            for l in left {
                for r in &right {
                    let mut tmp = TreeNode::new(root_val);
                    tmp.left = l.clone();
                    tmp.right = r.clone();
                    ans.push(Some(Rc::new(RefCell::new(tmp))));
                }
            }
        }
        ans
    }
    if n == 0 { return vec![]; }
    inner(&(1..=n).collect::<Vec<i32>>())
}

fn main() {
    for i in 1..=10 {
        println!("{}", num_trees(i));
    }
    for i in 1..=4 {
        println!("{:?}", generate_trees(i));
    }
}