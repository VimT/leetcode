//! 灵神的 treap go 实现


use std::cmp::Ordering;

pub struct TreapNode<K, V = ()> {
    lr: [Option<Box<TreapNode<K, V>>>; 2],
    key: K,
    val: V,
    priority: u32,
    size: usize,
}

impl<K: Ord, V> TreapNode<K, V> {
    fn new(key: K, val: V) -> Self {
        Self { lr: [None, None], key, priority: rand::random(), val, size: 1 }
    }
    fn maintain(&mut self) {
        self.size = 1 + self.lr[0].as_ref().map_or(0, |x| x.size) + self.lr[1].as_ref().map_or(0, |x| x.size);
    }
    // 旋转，并维护子树大小。d=0：左旋，返回右儿子 d=1：右旋，返回左儿子
    //         A                 C
    //        / \               / \
    //       B  C    ---->     A   E
    //         / \            / \
    //        D   E          B   D
    fn rotate(mut self: Box<Self>, d: usize) -> Box<Self> {
        let mut x = self.lr[d ^ 1].take().unwrap();
        self.lr[d ^ 1] = x.lr[d].take();
        self.maintain();
        x.lr[d] = Some(self);
        x.maintain();
        x
    }
}

#[derive(Default)]
pub struct Treap<K: Ord, V = ()> {
    root: Option<Box<TreapNode<K, V>>>,
}

impl<K: Ord + Clone, V> Treap<K, V> {
    pub fn new() -> Self {
        Self { root: None }
    }
    fn _insert(o: Option<Box<TreapNode<K, V>>>, key: K, val: V) -> Box<TreapNode<K, V>> {
        return if let Some(mut o) = o {
            let cmp = key.cmp(&o.key);
            if cmp == Ordering::Equal {
                o.val = val;
            } else {
                let d = (cmp == Ordering::Greater) as usize;
                o.lr[d] = Some(Self::_insert(o.lr[d].take(), key, val));
                if o.lr[d].as_ref().unwrap().priority > o.priority {
                    o = o.rotate(d ^ 1);
                }
            }
            o.maintain();
            o
        } else {
            Box::new(TreapNode::new(key, val))
        };
    }
    fn _remove(o: Option<Box<TreapNode<K, V>>>, key: &K, found: &mut Option<V>) -> Option<Box<TreapNode<K, V>>> {
        match o {
            None => None,
            Some(mut o) => {
                let cmp = key.cmp(&o.key);
                if cmp == Ordering::Equal {
                    if o.lr[1].is_none() {
                        let result = o.lr[0].take();
                        _ = found.insert(o.val);
                        return result;
                    }
                    if o.lr[0].is_none() {
                        let result = o.lr[1].take();
                        _ = found.insert(o.val);
                        return result;
                    }
                    let d = (o.lr[0].as_ref().unwrap().priority > o.lr[1].as_ref().unwrap().priority) as usize;
                    o = o.rotate(d);
                    o.lr[d] = Self::_remove(o.lr[d].take(), key, found);
                } else {
                    let d = (cmp == Ordering::Greater) as usize;
                    o.lr[d] = Self::_remove(o.lr[d].take(), key, found);
                }
                o.maintain();
                Some(o)
            }
        }
    }
    pub fn insert(&mut self, key: K, val: V) {
        let root = self.root.take();
        self.root = Some(Self::_insert(root, key, val));
    }
    pub fn contains(&self, key: &K) -> bool {
        let mut o = self.root.as_ref();
        while let Some(x) = o {
            let cmp = key.cmp(&x.key);
            if cmp == Ordering::Equal {
                return true;
            }
            o = x.lr[(cmp == Ordering::Greater) as usize].as_ref();
        }
        false
    }
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let root = self.root.take();
        let mut found = None;
        self.root = Self::_remove(root, key, &mut found);
        found
    }
    pub fn get(&self, key: &K) -> Option<&V> {
        let mut o = self.root.as_ref();
        while let Some(x) = o {
            let cmp = key.cmp(&x.key);
            if cmp == Ordering::Equal { return Some(&x.val); }
            o = x.lr[(cmp == Ordering::Greater) as usize].as_ref();
        }
        None
    }
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let mut o = self.root.as_mut();
        while let Some(x) = o {
            let cmp = key.cmp(&x.key);
            if cmp == Ordering::Equal { return Some(&mut x.val); }
            o = x.lr[(cmp == Ordering::Greater) as usize].as_mut();
        }
        None
    }
    pub fn len(&self) -> usize {
        self.root.as_ref().map_or(0, |x| x.size)
    }
    pub fn first(&self) -> Option<(&K, &V)> {
        if self.root.is_none() { return None; }
        let mut o = self.root.as_ref().unwrap();
        while let Some(x) = o.lr[0].as_ref() {
            o = x;
        }
        Some((&o.key, &o.val))
    }
    pub fn first_mut(&mut self) -> Option<(&K, &mut V)> {
        if self.root.is_none() { return None; }
        let mut o = self.root.as_mut().unwrap();
        while let Some(x) = o.lr[0].as_mut() {
            o = x;
        }
        Some((&o.key, &mut o.val))
    }
    pub fn last(&self) -> Option<(&K, &V)> {
        if self.root.is_none() { return None; }
        let mut o = self.root.as_ref().unwrap();
        while let Some(x) = o.lr[1].as_ref() {
            o = x;
        }
        Some((&o.key, &o.val))
    }
    pub fn last_mut(&mut self) -> Option<(&K, &mut V)> {
        if self.root.is_none() { return None; }
        let mut o = self.root.as_mut().unwrap();
        while let Some(x) = o.lr[1].as_mut() {
            o = x;
        }
        Some((&o.key, &mut o.val))
    }
    pub fn pop_first(&mut self) -> Option<(K, V)> {
        if let Some(key) = self.first().map(|x| x.0).cloned() {
            let val = self.remove(&key).unwrap();
            return Some((key, val));
        }
        None
    }
    pub fn pop_last(&mut self) -> Option<(K, V)> {
        if let Some(key) = self.last().map(|x| x.0).cloned() {
            let val = self.remove(&key).unwrap();
            return Some((key, val));
        }
        None
    }
    pub fn prev(&self, key: &K) -> Option<&K> {
        let mut o = self.root.as_ref();
        let mut ans = None;
        while let Some(x) = o {
            let cmp = key.cmp(&x.key);
            if cmp == Ordering::Greater {
                ans = Some(&x.key);
                o = x.lr[1].as_ref();
            } else {
                o = x.lr[0].as_ref();
            }
        }
        ans
    }
    pub fn next(&self, key: &K) -> Option<&K> {
        let mut o = self.root.as_ref();
        let mut ans = None;
        while let Some(x) = o {
            let cmp = key.cmp(&x.key);
            if cmp == Ordering::Less {
                ans = Some(&x.key);
                o = x.lr[0].as_ref();
            } else {
                o = x.lr[1].as_ref();
            }
        }
        ans
    }
    /// 返回 < key 的元素个数
    pub fn rank(&self, key: &K) -> usize {
        let mut o = self.root.as_ref();
        let mut ans = 0;
        while let Some(x) = o {
            let cmp = key.cmp(&x.key);
            match cmp {
                Ordering::Less => o = x.lr[0].as_ref(),
                Ordering::Equal => { break; }
                Ordering::Greater => {
                    ans += x.lr[0].as_ref().map_or(0, |x| x.size);
                    o = x.lr[1].as_ref();
                }
            }
        }
        ans
    }
    /// 返回第 k 小的元素 （k从0开始）
    pub fn kth(&self, k: usize) -> Option<&K> {
        if k >= self.len() { return None; }
        let mut o = self.root.as_ref();
        while let Some(x) = o {
            let left_size = x.lr[0].as_ref().map_or(0, |x| x.size);
            match k.cmp(&left_size) {
                Ordering::Less => o = x.lr[0].as_ref(),
                Ordering::Equal => return Some(&x.key),
                Ordering::Greater => o = x.lr[1].as_ref()
            }
        }
        None
    }
}


mod debug {
    use std::fmt::{Debug, DebugMap};
    use super::*;

    impl<K: Ord + Debug, V: Debug> Debug for Treap<K, V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            fn dfs<K: Debug + Ord, V: Debug>(node: Option<&Box<TreapNode<K, V>>>, f: &mut DebugMap) -> std::fmt::Result {
                if let Some(node) = node {
                    dfs(node.lr[0].as_ref(), f)?;
                    f.entry(&node.key, &node.val);
                    dfs(node.lr[1].as_ref(), f)?;
                }
                Ok(())
            }

            dfs(self.root.as_ref(), &mut f.debug_map())
        }
    }
}
