//! 序列顺序查询
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::mem::{replace, swap};

#[derive(Debug, PartialEq, Clone)]
pub struct AvlNode<T: Ord> {
    pub value: T,
    pub left: AvlTree<T>,
    pub right: AvlTree<T>,
    pub height: usize,
    pub children: usize,
}

pub type AvlTree<T> = Option<Box<AvlNode<T>>>;

impl<'a, T: 'a + Ord> AvlNode<T> {
    pub fn left_height(&self) -> usize {
        self.left.as_ref().map_or(0, |left| left.height)
    }

    pub fn right_height(&self) -> usize {
        self.right.as_ref().map_or(0, |right| right.height)
    }

    pub fn update_height(&mut self) {
        self.height = 1 + self.left_height().max(self.right_height());
        self.update_children();
    }

    pub fn left_children(&self) -> usize {
        self.left.as_ref().map_or(0, |left| left.children + 1)
    }

    pub fn right_children(&self) -> usize {
        self.right.as_ref().map_or(0, |right| right.children + 1)
    }

    pub fn update_children(&mut self) {
        self.children = self.left_children() + self.right_children();
    }

    pub fn balance_factor(&self) -> i8 {
        let left_height = self.left_height();
        let right_height = self.right_height();

        if left_height >= right_height {
            (left_height - right_height) as i8
        } else {
            -((right_height - left_height) as i8)
        }
    }

    pub fn rotate_left(&mut self) -> bool {
        if self.right.is_none() {
            return false;
        }

        let right_node = self.right.as_mut().unwrap();
        let right_left_tree = right_node.left.take();
        let right_right_tree = right_node.right.take();

        let mut new_left_tree = replace(&mut self.right, right_right_tree);
        swap(&mut self.value, &mut new_left_tree.as_mut().unwrap().value);
        let left_tree = self.left.take();

        let new_left_node = new_left_tree.as_mut().unwrap();
        new_left_node.right = right_left_tree;
        new_left_node.left = left_tree;
        self.left = new_left_tree;

        if let Some(node) = self.left.as_mut() {
            node.update_height();
        }

        self.update_height();

        true
    }

    pub fn rotate_right(&mut self) -> bool {
        if self.left.is_none() {
            return false;
        }

        let left_node = self.left.as_mut().unwrap();
        let left_right_tree = left_node.right.take();
        let left_left_tree = left_node.left.take();

        let mut new_right_tree = replace(&mut self.left, left_left_tree);
        swap(&mut self.value, &mut new_right_tree.as_mut().unwrap().value);
        let right_tree = self.right.take();

        let new_right_node = new_right_tree.as_mut().unwrap();
        new_right_node.left = left_right_tree;
        new_right_node.right = right_tree;
        self.right = new_right_tree;

        if let Some(node) = self.right.as_mut() {
            node.update_height();
        }

        self.update_height();

        true
    }

    pub fn rebalance(&mut self) -> bool {
        match self.balance_factor() {
            -2 => {
                let right_node = self.right.as_mut().unwrap();

                if right_node.balance_factor() == 1 {
                    right_node.rotate_right();
                }

                self.rotate_left();

                true
            }

            2 => {
                let left_node = self.left.as_mut().unwrap();

                if left_node.balance_factor() == -1 {
                    left_node.rotate_left();
                }

                self.rotate_right();

                true
            }
            _ => false,
        }
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct AvlTreeSet<T: Ord> {
    root: AvlTree<T>,
}

impl<'a, T: 'a + Ord> Default for AvlTreeSet<T> {
    fn default() -> Self {
        Self { root: None }
    }
}

impl<'a, T: 'a + Ord> AvlTreeSet<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, value: T) -> bool {
        let mut prev_ptrs = Vec::<*mut AvlNode<T>>::new();
        let mut current_tree = &mut self.root;

        while let Some(current_node) = current_tree {
            prev_ptrs.push(&mut **current_node);

            match current_node.value.cmp(&value) {
                Ordering::Less => current_tree = &mut current_node.right,
                Ordering::Equal => {
                    return false;
                }
                Ordering::Greater => current_tree = &mut current_node.left,
            }
        }

        *current_tree = Some(Box::new(AvlNode {
            value,
            left: None,
            right: None,
            height: 1,
            children: 0,
        }));

        for node_ptr in prev_ptrs.into_iter().rev() {
            let node = unsafe { &mut *node_ptr };
            node.update_height();
            node.rebalance();
        }

        true
    }

    pub fn get_n(&self, mut n: usize) -> Option<&T> {
        let mut current_tree = &self.root;
        while let Some(current_node) = current_tree {
            let left_children = current_node.left_children();
            if left_children > n {
                current_tree = &current_node.left;
            } else if left_children < n {
                n -= left_children + 1;
                current_tree = &current_node.right;
            } else {
                return Some(&current_node.value);
            }
        }
        None
    }
}

struct SORTracker {
    tree: AvlTreeSet<(i32, String)>,
    cur: usize,
}

impl SORTracker {
    fn new() -> Self {
        Self {
            tree: AvlTreeSet::new(),
            cur: 0,
        }
    }

    fn add(&mut self, name: String, score: i32) {
        self.tree.insert((-score, name));
    }

    fn get(&mut self) -> String {
        self.cur += 1;
        self.tree.get_n(self.cur - 1).unwrap().1.clone()
    }
}

struct SORTrackerHeap {
    /* 小根堆存放i-1个最大的值, 大根堆堆顶存放第i个最大值 */
    min_heap: BinaryHeap<(i32, String)>,
    max_heap: BinaryHeap<(i32, Reverse<String>)>,
}

impl SORTrackerHeap {
    fn new() -> Self {
        Self {
            min_heap: BinaryHeap::new(),
            max_heap: BinaryHeap::new(),
        }
    }

    fn add(&mut self, name: String, score: i32) {
        self.min_heap.push((-score, name));
        let (s, n) = self.min_heap.pop().unwrap();
        self.max_heap.push((-s, Reverse(n)));
    }

    fn get(&mut self) -> String {
        let (s, Reverse(n)) = self.max_heap.pop().unwrap();
        let result = n.clone();
        self.min_heap.push((-s, n));
        result
    }
}


fn main() {
    let mut tracker = SORTrackerHeap::new(); // 初始化系统
    tracker.add("bradford".to_string(), 2); // 添加 name="bradford" 且 score=2 的景点。
    tracker.add("branford".to_string(), 3); // 添加 name="branford" 且 score=3 的景点。
    assert_eq!(tracker.get(), "branford");              // 从好带坏的景点为：branford ，bradford 。
    // 注意到 branford 比 bradford 好，因为它的 评分更高 (3 > 2) 。
    // 这是第 1 次调用 get() ，所以返回最好的景点："branford" 。
    tracker.add("alps".to_string(), 2);     // 添加 name="alps" 且 score=2 的景点。
    assert_eq!(tracker.get(), "alps");              // 从好到坏的景点为：branford, alps, bradford 。
    // 注意 alps 比 bradford 好，虽然它们评分相同，都为 2 。
    // 这是因为 "alps" 字典序 比 "bradford" 小。
    // 返回第 2 好的地点 "alps" ，因为当前为第 2 次调用 get() 。
    tracker.add("orland".to_string(), 2);   // 添加 name="orland" 且 score=2 的景点。
    assert_eq!(tracker.get(), "bradford");              // 从好到坏的景点为：branford, alps, bradford, orland 。
    // 返回 "bradford" ，因为当前为第 3 次调用 get() 。
    tracker.add("orlando".to_string(), 3);  // 添加 name="orlando" 且 score=3 的景点。
    assert_eq!(tracker.get(), "bradford");              // 从好到坏的景点为：branford, orlando, alps, bradford, orland 。
    // 返回 "bradford".
    tracker.add("alpine".to_string(), 2);   // 添加 name="alpine" 且 score=2 的景点。
    assert_eq!(tracker.get(), "bradford");              // 从好到坏的景点为：branford, orlando, alpine, alps, bradford, orland 。
    // 返回 "bradford" 。
    assert_eq!(tracker.get(), "orland");              // 从好到坏的景点为：branford, orlando, alpine, alps, bradford, orland 。
    // 返回 "orland" 。
}
