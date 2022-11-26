//! 恢复二叉搜索树
use std::cell::RefCell;
use std::collections::VecDeque;
use std::mem::swap;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;

type Node = Option<Rc<RefCell<TreeNode>>>;

/// 转换成数组，找数组不对的位置
pub fn recover_tree(root: &mut Node) {
    fn inner(node: Node, vec: &mut Vec<Node>) {
        if node.is_none() { return; }
        inner(node.as_ref().unwrap().borrow().left.clone(), vec);
        vec.push(node.clone());
        inner(node.as_ref().unwrap().borrow().right.clone(), vec);
    }
    let mut arr = vec![];
    inner(root.clone(), &mut arr);
    let mut p1 = None;
    let mut p2 = None;
    for i in 0..arr.len() - 1 {
        // 这个循环很巧妙。p1是第一个不对的位置，p2是第二个不对的位置
        if arr[i].as_ref().unwrap().borrow().val > arr[i + 1].as_ref().unwrap().borrow().val {
            p2 = arr[i + 1].clone();
            if p1.is_none() {
                p1 = arr[i].clone();
            }
        }
    }
    swap(&mut p1.as_ref().unwrap().borrow_mut().val, &mut p2.as_ref().unwrap().borrow_mut().val);
}

/// 迭代中序遍历
/// 使用栈迭代中序遍历：尽可能左走，然后往右走一步，重复直到结束
pub fn recover_tree_iter(root: &mut Node) {
    let mut stack = VecDeque::new();
    let mut p = root.clone();
    let mut x: Node = None;
    let mut y: Node = None;
    let mut pre: Node = None;
    while !stack.is_empty() || p.is_some() {
        while p.is_some() {
            stack.push_back(p.clone());
            let tmp = p.as_ref().unwrap().borrow().left.clone();
            p = tmp;
        }
        p = stack.pop_back().unwrap();
        if pre.is_some() && p.as_ref().unwrap().borrow().val < pre.as_ref().unwrap().borrow().val {
            y = p.clone();
            match x {
                Some(_) => break,
                None => x = pre.clone()
            }
        }
        pre = p.clone();
        let tmp = p.as_ref().unwrap().borrow().right.clone();
        p = tmp;
    }
    swap(&mut x.as_ref().unwrap().borrow_mut().val, &mut y.as_ref().unwrap().borrow_mut().val);
}

/// 递归中序遍历
pub fn recover_tree_iter_dg(root: &mut Node) {
    fn inner(node: Node, pre: &mut Node, x: &mut Node, y: &mut Node) {
        if let Some(n) = node {
            inner(n.borrow().left.clone(), pre, x, y);
            if pre.is_some() && n.borrow().val < pre.as_ref().unwrap().borrow().val {
                *y = Some(n.clone());
                if x.is_none() {
                    *x = pre.clone();
                } else {
                    return;
                }
            }
            *pre = Some(n.clone());
            inner(n.borrow().right.clone(), pre, x, y);
        }
    }
    let mut x: Node = None;
    let mut y: Node = None;
    let mut pre: Node = None;
    inner(root.clone(), &mut pre, &mut x, &mut y);
    swap(&mut x.as_ref().unwrap().borrow_mut().val, &mut y.as_ref().unwrap().borrow_mut().val);
}

type RefNode<'a> = Option<&'a Rc<RefCell<TreeNode>>>;

/// 递归中序遍历，纯引用
pub fn recover_tree_iter_dg_ref(root: &mut Node) {
    fn inner(node: RefNode, pre: &mut Node, x: &mut Node, y: &mut Node) {
        if let Some(n) = node {
            inner(n.borrow().left.as_ref(), pre, x, y);
            if pre.is_some() && n.borrow().val < pre.as_ref().unwrap().borrow().val {
                *y = Some(n.clone());
                if x.is_none() {
                    *x = pre.clone();
                } else {
                    return;
                }
            }
            *pre = Some(n.clone());
            inner(n.borrow().right.as_ref(), pre, x, y);
        }
    }
    let mut x: Node = None;
    let mut y: Node = None;
    let mut pre: Node = None;
    inner(root.as_ref(), &mut pre, &mut x, &mut y);
    swap(&mut x.as_ref().unwrap().borrow_mut().val, &mut y.as_ref().unwrap().borrow_mut().val);
}

/// 莫里斯中序遍历 O(1) 空间复杂度
/// 总是报错，迷
pub fn recover_tree_morris(root: &mut Node) {
    let mut x: Node = None;
    let mut y: Node = None;
    let mut p = root.clone();
    let mut pre: Node = None;
    let mut pp: Node;
    while p.is_some() {
        if p.as_ref().unwrap().borrow().left.is_none() {
            // 到了左叶节点
            if pre.is_some() && p.as_ref().unwrap().borrow().val < pre.as_ref().unwrap().borrow().val {
                y = p.clone();
                if x.is_none() {
                    x = pre.clone();
                }
            }
            pre = p.clone();
            let tmp = p.as_ref().unwrap().borrow().right.clone();
            p = tmp;
        } else {
            // 设置pp为 当前节点 左子树的最右一个节点
            pp = p.as_ref().unwrap().borrow().left.clone();
            while pp.as_ref().unwrap().borrow().right.is_some() && pp.as_ref().unwrap().borrow().right != p {
                let tmp = pp.as_ref().unwrap().borrow().right.clone();
                pp = tmp;
            }

            if pp.as_ref().unwrap().borrow().right.is_none() {
                // 如果最右节点为空,则关联到当前节点,即当前节点为 最右节点 的后继, 最右节点 为 当前节点 的前序
                pp.as_ref().unwrap().borrow_mut().right = p.clone();
                let tmp = p.as_ref().unwrap().borrow().left.clone();
                p = tmp;
            } else {
                // 最右节点已经设置过,则说明左子树已经遍历完毕,进入右子树
                if pre.is_some() && p.as_ref().unwrap().borrow().val < pre.as_ref().unwrap().borrow().val {
                    y = p.clone();
                    if x.is_none() {
                        x = pre.clone();
                    }
                }
                pre = p.clone();
                let tmp = p.as_ref().unwrap().borrow().right.clone();
                p = tmp;
            }
        }
    }
    swap(&mut x.as_ref().unwrap().borrow_mut().val, &mut y.as_ref().unwrap().borrow_mut().val);
}


fn main() {
    fn test(func: fn(root: &mut Node)) {
        let help = |mut root: Node| {
            func(&mut root);
            root
        };
        assert_eq!(help(tree![1,3,null,null,2]), tree![3,1,null,null,2]);
        assert_eq!(help(tree![3,1,4,null,null,2]), tree![2,1,4,null,null,3]);
    }
    test(recover_tree);
    test(recover_tree_iter);
    test(recover_tree_iter_dg);
    test(recover_tree_iter_dg_ref);
    // test(recover_tree_morris);
}
