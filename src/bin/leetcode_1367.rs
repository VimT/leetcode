//! 二叉树中的链表

use leetcode::linknode::ListNode;
use leetcode::treenode::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;

pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn dfs(link: &Vec<i32>, idx: &Vec<usize>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        for &i in idx {
            if i == link.len() { return true; }
        }
        if root.is_none() { return false; }
        let node = root.as_ref().unwrap().borrow();
        let mut next_idx: Vec<usize> = idx.into_iter().filter(|&&x| link[x] == node.val).map(|x| x + 1).collect();
        if node.val == link[0] { next_idx.push(1); }
        dfs(link, &next_idx, node.left.clone()) || dfs(link, &next_idx, node.right.clone())
    }
    let mut link = vec![];
    let mut p = &head;
    while !p.is_none() {
        link.push(p.as_ref().unwrap().val);
        p = &p.as_ref().unwrap().next;
    }
    dfs(&link, &vec![], root)
}

/// 不转成vec做
pub fn is_sub_path2(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn dfs2(head: Option<&Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if head.is_none() { return true; }
        if root.is_none() { return false; }
        let node = root.as_ref().unwrap().borrow();
        node.val == head.unwrap().val && (dfs2(head.unwrap().next.as_ref(), node.left.clone()) || dfs2(head.unwrap().next.as_ref(), node.right.clone()))
    }
    fn dfs1(head: Option<&Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() || head.is_none() { return false; }
        let node = root.as_ref().unwrap().borrow();
        if node.val == head.unwrap().val && dfs2(head, root.clone()) {
            return true;
        }
        dfs1(head, node.left.clone()) || dfs1(head, node.right.clone())
    }
    dfs1(head.as_ref(), root)
}

fn main() {
    use leetcode::tree;
    use leetcode::link;
    fn test(func: fn(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool) {
        assert_eq!(func(link![4,2,8], tree![1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]), true);
        assert_eq!(func(link![1,4,2,6], tree![1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]), true);
        assert_eq!(func(link![1,4,2,6,8], tree![1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]), false);
    }
    test(is_sub_path);
    test(is_sub_path2);
}
