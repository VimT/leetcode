//! From https://github.com/EbTech/rust-algorithms/blob/master/src/range_query/static_arq.rs
//! SegmentVal 是一个 Monoid
//! 要满足：
//! 1. 结合律： op(a, op(b, c)) = op(op(a, b), c)
//! 2. 单位元： op(a, identity()) = op(identity(), a) = a
//! 有lazy时
//! 1. 结合律：apply(compose(f, g), a) = apply(f, apply(g, a))
//! 2. 分配律：apply(f, op(a, b), s+t) = op(apply(f, a, s), apply(f, b, t))

pub trait SegmentSpec {
    type ValType: Clone;
    type LazyType: Clone;
    fn op(a: &Self::ValType, b: &Self::ValType) -> Self::ValType;
    fn identity() -> Self::ValType;
    fn compose(f: &Self::LazyType, g: &Self::LazyType) -> Self::LazyType;
    fn apply(f: &Self::LazyType, a: &Self::ValType, size: i64) -> Self::ValType;
}

pub struct SegmentTree<T: SegmentSpec> {
    val: Vec<T::ValType>,
    lazy: Vec<Option<T::LazyType>>,
    len: usize,
}

impl<T: SegmentSpec> SegmentTree<T> {
    pub fn new(init_val: &[T::ValType]) -> Self {
        let len = init_val.len();
        let mut tree = Self::new_with_size(len);
        tree.build(init_val, 0, len - 1, 1);
        tree
    }
    pub fn new_with_size(len: usize) -> Self {
        Self { val: vec![T::identity(); len * 4], lazy: vec![None; len * 4], len }
    }
    fn build(&mut self, init_val: &[T::ValType], s: usize, t: usize, p: usize) {
        if s == t {
            self.val[p] = init_val[s].clone();
            return;
        }
        let m = (s + t) / 2;
        self.build(init_val, s, m, p * 2);
        self.build(init_val, m + 1, t, p * 2 + 1);
        self.val[p] = T::op(&self.val[p * 2], &self.val[p * 2 + 1]);
    }
    fn pushdown(&mut self, s: usize, t: usize, p: usize) {
        if let Some(ref f) = self.lazy[p].take() {
            let m = (s + t) / 2;
            self.val[p * 2] = T::apply(f, &self.val[p * 2], (m - s + 1) as i64);
            self.val[p * 2 + 1] = T::apply(f, &self.val[p * 2 + 1], (t - m) as i64);
            self.lazy[p * 2] = match self.lazy[p * 2].take() {
                Some(g) => Some(T::compose(f, &g)),
                None => Some(f.clone()),
            };
            self.lazy[p * 2 + 1] = match self.lazy[p * 2 + 1].take() {
                Some(g) => Some(T::compose(f, &g)),
                None => Some(f.clone()),
            };
        }
    }

    fn _update(&mut self, l: usize, r: usize, s: usize, t: usize, p: usize, f: &T::LazyType) {
        if l <= s && t <= r {
            self.val[p] = T::apply(f, &self.val[p], (t + 1 - s) as i64);
            self.lazy[p] = match self.lazy[p].take() {
                Some(g) => Some(T::compose(f, &g)),
                None => Some(f.clone()),
            };
            return;
        }
        self.pushdown(s, t, p);
        let m = (s + t) / 2;
        if l <= m { self._update(l, r, s, m, p * 2, f); }
        if m < r { self._update(l, r, m + 1, t, p * 2 + 1, f); }
        self.val[p] = T::op(&self.val[p * 2], &self.val[p * 2 + 1]);
    }

    pub fn update(&mut self, l: usize, r: usize, f: &T::LazyType) {
        assert!(l <= r);
        self._update(l, r, 0, self.len - 1, 1, f);
    }
    fn _query(&mut self, l: usize, r: usize, s: usize, t: usize, p: usize) -> T::ValType {
        if l <= s && t <= r {
            return self.val[p].clone();
        }
        self.pushdown(s, t, p);
        let m = (s + t) / 2;
        let mut ans = T::identity();
        if l <= m { ans = T::op(&ans, &self._query(l, r, s, m, p * 2)); }
        if m < r { ans = T::op(&ans, &self._query(l, r, m + 1, t, p * 2 + 1)); }
        ans
    }
    pub fn query(&mut self, l: usize, r: usize) -> T::ValType {
        assert!(l <= r);
        self._query(l, r, 0, self.len - 1, 1)
    }
}

#[derive(Default, Clone)]
pub struct DynamicSegmentTreeNode<T: SegmentSpec> {
    val: T::ValType,
    lazy: Option<T::LazyType>,
    l: usize,
    r: usize,
}

impl<T: SegmentSpec> DynamicSegmentTreeNode<T> {
    fn default() -> Self {
        Self { val: T::identity(), lazy: None, l: 0, r: 0 }
    }
    fn apply(&mut self, f: &T::LazyType, size: i64) {
        self.val = T::apply(f, &self.val, size);
        if size > 1 {
            self.lazy = Some(match self.lazy {
                Some(ref g) => T::compose(f, g),
                None => f.clone(),
            });
        }
    }
}

/// 动态开点线段树
pub struct DynamicSegmentTree<T: SegmentSpec> {
    nodes: Vec<DynamicSegmentTreeNode<T>>,
    mn: i64,
    mx: i64,
}

impl<T: SegmentSpec> DynamicSegmentTree<T> {
    pub fn new(len: usize, mn: i64, mx: i64) -> Self {
        let mut tree = Vec::with_capacity(len);
        tree.push(DynamicSegmentTreeNode::default());
        tree.push(DynamicSegmentTreeNode::default());
        Self { nodes: tree, mn, mx }
    }
    fn pushdown(&mut self, s: i64, t: i64, p: usize) {
        if self.nodes[p].l == 0 {
            self.nodes.push(DynamicSegmentTreeNode::default());
            self.nodes[p].l = self.nodes.len() - 1;
        }
        if self.nodes[p].r == 0 {
            self.nodes.push(DynamicSegmentTreeNode::default());
            self.nodes[p].r = self.nodes.len() - 1;
        }
        if let Some(ref f) = self.nodes[p].lazy.take() {
            let mid = (s + t) >> 1;
            let (l, r) = (self.nodes[p].l, self.nodes[p].r);
            self.nodes[l].apply(f, mid - s + 1);
            self.nodes[r].apply(f, t - mid);
        }
    }
    fn _query(&mut self, l: i64, r: i64, s: i64, t: i64, p: usize) -> T::ValType {
        if l <= s && t <= r {
            return self.nodes[p].val.clone();
        }
        self.pushdown(s, t, p);
        let mid = (s + t) >> 1;
        let mut result = T::identity();
        if l <= mid { result = T::op(&result, &self._query(l, r, s, mid, self.nodes[p].l)); }
        if mid < r { result = T::op(&result, &self._query(l, r, mid + 1, t, self.nodes[p].r)); }
        result
    }
    fn _update(&mut self, l: i64, r: i64, s: i64, t: i64, p: usize, f: &T::LazyType) {
        if l <= s && t <= r {
            self.nodes[p].apply(f, t + 1 - s);
            return;
        }
        self.pushdown(s, t, p);
        let mid = (s + t) >> 1;
        if l <= mid { self._update(l, r, s, mid, self.nodes[p].l, f); }
        if mid < r { self._update(l, r, mid + 1, t, self.nodes[p].r, f); }
        self.nodes[p].val = T::op(&self.nodes[self.nodes[p].l].val, &self.nodes[self.nodes[p].r].val);
    }
    pub fn query(&mut self, l: i64, r: i64) -> T::ValType {
        assert!(l <= r);
        self._query(l, r, self.mn, self.mx, 1)
    }
    pub fn update(&mut self, l: i64, r: i64, f: &T::LazyType) {
        assert!(l <= r);
        self._update(l, r, self.mn, self.mx, 1, f);
    }
}

pub trait SimpleSegmentSpec {
    type ValueType: Clone;
    fn identify() -> Self::ValueType;
    fn combine(a: &Self::ValueType, b: &Self::ValueType) -> Self::ValueType;
}

// 没有lazy，单点更新
pub struct SimpleSegmentTree<T: SimpleSegmentSpec> {
    val: Vec<T::ValueType>,
    len: usize,
}

impl<T: SimpleSegmentSpec> SimpleSegmentTree<T> {
    pub fn new(init_val: &[T::ValueType]) -> Self {
        let len = init_val.len();
        let mut tree = Self { val: vec![T::identify(); len * 4], len };
        tree.build(init_val, 0, len - 1, 1);
        tree
    }
    fn build(&mut self, init_val: &[T::ValueType], s: usize, t: usize, p: usize) {
        if s == t {
            self.val[p] = init_val[s].clone();
            return;
        }
        let m = (s + t) / 2;
        self.build(init_val, s, m, p * 2);
        self.build(init_val, m + 1, t, p * 2 + 1);
        self.val[p] = T::combine(&self.val[p * 2], &self.val[p * 2 + 1]);
    }

    fn _update(&mut self, pos: usize, s: usize, t: usize, p: usize, value: T::ValueType) {
        if s == t {
            self.val[p] = value;
            return;
        }
        let m = (s + t) / 2;
        if pos <= m { self._update(pos, s, m, p * 2, value); } else { self._update(pos, m + 1, t, p * 2 + 1, value); }
        self.val[p] = T::combine(&self.val[p * 2], &self.val[p * 2 + 1]);
    }

    pub fn update(&mut self, pos: usize, value: T::ValueType) {
        assert!(pos < self.len);
        self._update(pos, 0, self.len - 1, 1, value);
    }
    fn _query(&mut self, l: usize, r: usize, s: usize, t: usize, p: usize) -> T::ValueType {
        if l <= s && t <= r {
            return self.val[p].clone();
        }
        let m = (s + t) / 2;
        let mut ans = T::identify();
        if l <= m { ans = T::combine(&ans, &self._query(l, r, s, m, p * 2)); }
        if m < r { ans = T::combine(&ans, &self._query(l, r, m + 1, t, p * 2 + 1)); }
        ans
    }
    pub fn query(&mut self, l: usize, r: usize) -> T::ValueType {
        assert!(l <= r);
        assert!(r < self.len);
        self._query(l, r, 0, self.len - 1, 1)
    }
}
