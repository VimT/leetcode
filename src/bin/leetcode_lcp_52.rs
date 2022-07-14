//! 2022春季编程大赛：二叉搜索树染色

use std::cell::RefCell;
use std::collections::{HashMap, BTreeSet};
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::{leetcode_tree, TreeNode};

// merge是求和，更新是设置值，线段树
struct SegmentTree {
    tree: Vec<i32>,
    lazy: Vec<Option<i32>>,
    len: usize,
}

impl SegmentTree {
    fn new(len: usize) -> Self {
        Self { tree: vec![0; len * 4], lazy: vec![None; len * 4], len }
    }


    fn _update(&mut self, l: usize, r: usize, s: usize, t: usize, p: usize, value: i32) {
        if s >= l && t <= r {
            self.tree[p] = (t - s + 1) as i32 * value;
            self.lazy[p] = Some(value);
            return;
        }
        let mid = (s + t) / 2;
        if self.lazy[p].is_some() && s != t {
            let lazy = self.lazy[p].unwrap();
            self.tree[p * 2] = lazy * (mid - s + 1) as i32;
            self.tree[p * 2 + 1] = lazy * (t - mid) as i32;
            self.lazy[p * 2] = Some(lazy);
            self.lazy[p * 2 + 1] = Some(lazy);
            self.lazy[p] = None;
        }
        if l <= mid { self._update(l, r, s, mid, p * 2, value); }
        if r > mid { self._update(l, r, mid + 1, t, p * 2 + 1, value); }
        self.tree[p] = self.tree[p * 2] + self.tree[p * 2 + 1];
    }

    fn update(&mut self, l: usize, r: usize, d: i32) {
        self._update(l, r, 0, self.len - 1, 1, d)
    }

    fn _query(&mut self, l: usize, r: usize, s: usize, t: usize, p: usize) -> i32 {
        if s >= l && t <= r {
            return self.tree[p];
        }
        let mid = (s + t) / 2;
        if let Some(lazy) = self.lazy[p] {
            self.tree[p * 2] = lazy * (mid - s + 1) as i32;
            self.tree[p * 2 + 1] = lazy * (t - mid) as i32;
            self.lazy[p * 2] = Some(lazy);
            self.lazy[p * 2 + 1] = Some(lazy);
            self.lazy[p] = None;
        }
        let mut ans = 0;
        if l <= mid { ans += self._query(l, r, s, mid, p * 2); }
        if r > mid { ans += self._query(l, r, mid + 1, t, p * 2 + 1); }
        ans
    }

    fn query(&mut self, l: usize, r: usize) -> i32 {
        self._query(l, r, 0, self.len - 1, 1)
    }
}

/// 线段树做法
pub fn get_number(root: Option<Rc<RefCell<TreeNode>>>, ops: Vec<Vec<i32>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, arr: &mut Vec<i32>) {
        if root.is_none() { return; }
        let node = root.as_ref().unwrap().borrow();
        dfs(node.left.clone(), arr);
        arr.push(node.val);
        dfs(node.right.clone(), arr);
    }
    let mut arr = vec![];
    dfs(root, &mut arr);
    let mut vi = HashMap::new();
    for (i, &v) in arr.iter().enumerate() {
        vi.insert(v, i);
    }
    let len = arr.len();
    let mut tree = SegmentTree::new(len);
    for op in ops {
        tree.update(vi[&op[1]], vi[&op[2]], op[0]);
    }
    tree.query(0, len)
}

/// 记录差分位置，当前最后一次设置是否是 染红，如果是就+1
pub fn get_number2(root: Option<Rc<RefCell<TreeNode>>>, ops: Vec<Vec<i32>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, arr: &mut Vec<i32>) {
        if root.is_none() { return; }
        let node = root.as_ref().unwrap().borrow();
        dfs(node.left.clone(), arr);
        arr.push(node.val);
        dfs(node.right.clone(), arr);
    }
    let mut arr = vec![];
    dfs(root, &mut arr);
    let mut vi = HashMap::new();
    for (i, &v) in arr.iter().enumerate() {
        vi.insert(v, i);
    }
    let len = arr.len();
    let mut flag = vec![vec![]; len + 1];
    for (i, op) in ops.iter().enumerate() {
        let s = vi[&op[1]];
        let t = vi[&op[2]];
        flag[s].push(i as i32 + 1);
        flag[t + 1].push(-(i as i32 + 1));
    }
    let mut result = 0;
    let mut t = BTreeSet::new();
    for i in 0..len {
        for &j in &flag[i] {
            if j > 0 {
                t.insert(j);
            } else {
                t.remove(&-j);
            }
        }
        if !t.is_empty() && ops[*t.range(..).last().unwrap() as usize - 1][0] == 1 {
            result += 1;
        }
    }
    result
}

fn main() {
    fn test(func: fn(Option<Rc<RefCell<TreeNode>>>, ops: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(tree![4,2,7,1,null,5,null,null,null,null,6], vec![vec![0, 2, 2], vec![1, 1, 5], vec![0, 4, 5], vec![1, 5, 7]]), 5);
        assert_eq!(func(tree![1,null,2,null,3,null,4,null,5], vec![vec![1, 2, 4], vec![1, 1, 3], vec![0, 3, 5]]), 2);
    }
    test(get_number);
    test(get_number2);
}
