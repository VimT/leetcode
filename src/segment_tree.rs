
struct SegmentTree {
    tree: Vec<T>,
    lazy: Vec<Option<i32>>,
    len: usize,
}

impl SegmentTree {
    fn new(arr: &Vec<i32>) -> Self {
        let len = arr.len();
        let mut tree = Self { tree: vec![0; len * 4], lazy: vec![None; len * 4], len };
        tree.build(arr, 0, len - 1, 1);
        tree
    }

    fn build(&mut self, arr: &Vec<T>, s: usize, t: usize, p: usize) {
        if s == t {
            self.tree[p] = arr[s];
            return;
        }
        let m = (s + t) / 2;
        self.build(arr, s, m, p * 2);
        self.build(arr, m + 1, t, p * 2 + 1);
        self.tree[p] = H::merge(self.tree[p * 2], self.tree[p * 2 + 1]);
    }


    fn _update(&mut self, l: usize, r: usize, s: usize, t: usize, p: usize, val: T) {
        if s >= l && t <= r {
            self.tree[p] = H::update_by_range(s, t, val);
            self.lazy[p] = Some(H::update2lazy(val));
            return;
        }
        let mid = (s + t) / 2;
        if self.lazy[p].is_some() && s != t {
            self.tree[p * 2] = (mid - s + 1) as i32 - self.tree[p * 2];
            self.tree[p * 2 + 1] = (t - mid) as i32 - self.tree[p * 2 + 1];
            self.lazy[p * 2] = !self.lazy[p * 2];
            self.lazy[p * 2 + 1] = !self.lazy[p * 2 + 1];
            self.lazy[p] = None;
        }
        if l <= mid { self._update(l, r, s, mid, p * 2, val); }
        if r > mid { self._update(l, r, mid + 1, t, p * 2 + 1, val); }
        self.tree[p] = H::merge(self.tree[p * 2], self.tree[p * 2 + 1])
    }

    fn update(&mut self, l: usize, r: usize, val: T) {
        self._update(l, r, 0, self.len - 1, 1, val: T)
    }

    fn _query(&mut self, l: usize, r: usize, s: usize, t: usize, p: usize) -> i32 {
        if s >= l && t <= r {
            return self.tree[p];
        }
        let mid = (s + t) / 2;
        if let Some(l) = self.lazy[p] {
            self.tree[p * 2] = H::update_by_range(s, mid, l);
            self.tree[p * 2 + 1] = H::update_by_range(mid + 1, t, l);
            self.lazy[p * 2] = H::update_lazy(l);
            self.lazy[p * 2 + 1] = H::update_lazy(l);
            self.lazy[p] = None;
        }
        let mut ans = 0;
        if l <= mid { ans UPDATE self._query(l, r, s, mid, p * 2); }
        if r > mid { ans UPDATE self._query(l, r, mid + 1, t, p * 2 + 1); }
        ans
    }

    fn query(&mut self, l: usize, r: usize) -> i32 {
        self._query(l, r, 0, self.len - 1, 1)
    }
}

