//! 二叉树的中序遍历

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;

type Node = Option<Rc<RefCell<TreeNode>>>;

pub fn inorder_traversal(root: Node) -> Vec<i32> {
    fn inner(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
        if root.is_none() { return; }
        inner(root.as_ref().unwrap().borrow().left.clone(), ans);
        ans.push(root.as_ref().unwrap().borrow().val);
        inner(root.as_ref().unwrap().borrow().right.clone(), ans);
    }
    let mut ans = vec![];
    inner(root, &mut ans);
    ans
}

pub fn inorder_traversal_stack(root: Node) -> Vec<i32> {
    let mut ans = vec![];
    let mut stack = VecDeque::new();
    let mut cur = root;
    while cur.is_some() || !stack.is_empty() {
        while cur.is_some() {
            stack.push_back(cur.clone());
            let tmp = cur.as_ref().unwrap().borrow().left.clone();
            cur = tmp;
        }
        cur = stack.pop_back().unwrap();
        ans.push(cur.as_ref().unwrap().borrow().val);
        let tmp = cur.as_ref().unwrap().borrow().right.clone();
        cur = tmp;
    }
    ans
}

/// 莫里斯遍历
/// 线索二叉树:
pub fn inorder_traversal_mls(root: Node) -> Vec<i32> {
    let mut ans = vec![];
    let mut cur = root;
    let mut pre;
    while cur.is_some() {
        if cur.as_ref().unwrap().borrow().left.is_none() {
            ans.push(cur.as_ref().unwrap().borrow().val);
            let tmp = cur.as_ref().unwrap().borrow().right.clone();
            cur = tmp;
        } else {
            pre = cur.as_ref().unwrap().borrow().left.clone();
            while pre.as_ref().unwrap().borrow().right.is_some() {
                let tmp = pre.as_ref().unwrap().borrow().right.clone();
                pre = tmp;
            }
            pre.as_ref().unwrap().borrow_mut().right = cur.clone();
            let tmp = cur.as_ref().unwrap().borrow_mut().left.take();
            cur = tmp;
        }
    }
    ans
}


/// 颜色标记法：新节点为白色，访问过的为灰色。出栈顺序为倒序，所以倒着入栈就可以
/// 遇到白色，标记为灰色，入栈 右节点，self，左节点
/// 遇到灰色，输出
/// 中序遍历：右中左
/// 前序遍历：右左中
/// 后序遍历：中右左
enum ColoredNode {
    White(Node),
    Gray(Node),
}

pub fn inorder_traversal_color(root: Node) -> Vec<i32> {
    let mut stack = VecDeque::new();
    let mut ans = vec![];
    stack.push_back(ColoredNode::White(root));
    while !stack.is_empty() {
        let node = stack.pop_back().unwrap();
        match node {
            ColoredNode::White(root) => {
                if root.is_none() { continue; }
                stack.push_back(ColoredNode::White(root.as_ref().unwrap().borrow().right.clone()));
                stack.push_back(ColoredNode::Gray(root.clone()));
                stack.push_back(ColoredNode::White(root.as_ref().unwrap().borrow().left.clone()));
            }
            ColoredNode::Gray(root) => {
                ans.push(root.as_ref().unwrap().borrow().val);
            }
        }
    }
    ans
}


fn main() {
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32>) {
        assert_eq!(func(tree![1,null,2,3]), vec![1, 3, 2]);
        assert_eq!(func(tree![]), vec![]);
        assert_eq!(func(tree![1]), vec![1]);
    }
    test(inorder_traversal);
    test(inorder_traversal_color);
    test(inorder_traversal_mls);
    test(inorder_traversal_stack);
}
