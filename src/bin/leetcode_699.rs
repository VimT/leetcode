//! 掉落的方块


use std::collections::{BTreeSet, HashMap};

/// 给后面的方块加高度
pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
    let len = positions.len();
    let mut qans = vec![0; len];
    for i in 0..len {
        let left = positions[i][0];
        let size = positions[i][1];
        let right = left + size;
        qans[i] += size;
        for j in i + 1..len {
            let left2 = positions[j][0];
            let size2 = positions[j][1];
            let right2 = left2 + size2;
            // 两线段是否相交
            if left2 < right && left < right2 {
                qans[j] = qans[j].max(qans[i]);
            }
        }
    }
    let mut result = Vec::with_capacity(len);
    let mut cur = -1;
    for x in qans {
        cur = cur.max(x);
        result.push(cur);
    }
    result
}

/// 坐标压缩，把离散的坐标 聚集. 然后问题就是区间最大值，和更新区间
pub fn falling_squares_compress(positions: Vec<Vec<i32>>) -> Vec<i32> {
    let len = positions.len();
    let mut set = BTreeSet::new();
    for pos in &positions {
        set.insert(pos[0]);
        set.insert(pos[0] + pos[1] - 1);
    }
    let axis: Vec<i32> = set.into_iter().collect();
    let mut m = HashMap::new();
    for i in 0..axis.len() {
        m.insert(axis[i], i);
    }
    let mut height = vec![0; axis.len()];
    let mut cur_max = 0;
    let mut result = Vec::with_capacity(len);
    for pos in positions {
        let l = m[&pos[0]];
        let r = m[&(pos[0] + pos[1] - 1)];
        let h = *height[l..=r].iter().max().unwrap() + pos[1];
        for i in l..=r {
            height[i] = h;
        }
        cur_max = cur_max.max(h);
        result.push(cur_max);
    }
    result
}

struct SegmentTree {
    tree: Vec<i32>,
    lazy: Vec<i32>,
    len: usize,
}

impl SegmentTree {
    fn new(len: usize) -> Self {
        Self { tree: vec![0; len * 4], lazy: vec![0; len * 4], len }
    }

    fn push_down(&mut self, p: usize) {
        self.tree[p * 2] = self.tree[p * 2].max(self.lazy[p]);
        self.tree[p * 2 + 1] = self.tree[p * 2 + 1].max(self.lazy[p]);
        self.lazy[p * 2] = self.lazy[p];
        self.lazy[p * 2 + 1] = self.lazy[p];
        self.lazy[p] = 0;
    }

    fn _update(&mut self, l: usize, r: usize, s: usize, t: usize, p: usize, d: i32) {
        if s >= l && t <= r {
            self.tree[p] = self.tree[p].max(d);
            self.lazy[p] = self.tree[p].max(d);
            return;
        }
        let mid = (s + t) / 2;
        if self.lazy[p] > 0 && s != t {
            self.push_down(p);
        }
        if l <= mid { self._update(l, r, s, mid, p * 2, d); }
        if r > mid { self._update(l, r, mid + 1, t, p * 2 + 1, d); }
        self.tree[p] = self.tree[p * 2].max(self.tree[p * 2 + 1]);
    }

    fn update(&mut self, l: usize, r: usize, d: i32) {
        self._update(l, r, 0, self.len - 1, 1, d)
    }

    fn _query(&mut self, l: usize, r: usize, s: usize, t: usize, p: usize) -> i32 {
        if s >= l && t <= r {
            return self.tree[p];
        }
        if self.lazy[p] > 0 {
            self.push_down(p);
        }
        let mid = (s + t) / 2;
        let mut ans = 0;
        if l <= mid { ans = ans.max(self._query(l, r, s, mid, p * 2)); }
        if r > mid { ans = ans.max(self._query(l, r, mid + 1, t, p * 2 + 1)); }
        ans
    }

    fn query(&mut self, l: usize, r: usize) -> i32 {
        self._query(l, r, 0, self.len - 1, 1)
    }
}

struct ZkwSegmentTree {
    tree: Vec<i32>,
    len: usize,
    // lazy数组，惰性传播，更新的时候只更新上面的节点，求值的时候才向下传播（push）
    lazy: Vec<i32>,
    height: i32,
}

impl ZkwSegmentTree {
    fn new(len: usize) -> Self {
        let mut height = 1;
        while (1 << height) < len { height += 1; }
        // 为什么这里是×2而不是×4，因为本线段树是一个zkw二叉树。
        // 二叉树的构造有两种，一种是自顶向下递归的，这种非叶子结点就是满二叉树，但是叶子节点 不是满的，所以叶子节点为2×n，整个树要4×n
        // 另一种是zkw, 自底向上的，构造出来整个树就是满二叉树。
        // 示例可以看 知乎 “关于线段树的数组到底是开2N还是4N”
        Self { tree: vec![0; len * 2], len, lazy: vec![0; len], height }
    }

    fn apply(&mut self, x: usize, val: i32) {
        self.tree[x] = self.tree[x].max(val);
        if x < self.len {
            self.lazy[x] = self.lazy[x].max(val);
        }
    }

    /// 向上更新所有子树
    fn pull(&mut self, mut x: usize) {
        while x > 1 {
            x >>= 1;
            self.tree[x] = self.tree[x * 2].max(self.tree[x * 2 + 1]);
            self.tree[x] = self.tree[x].max(self.lazy[x]);
        }
    }

    /// lazy 下推
    fn push(&mut self, x: usize) {
        for h in (1..=self.height).rev() {
            let y = x >> h;
            if self.lazy[y] > 0 {
                self.apply(y * 2, self.lazy[y]);
                self.apply(y * 2 + 1, self.lazy[y]);
            }
            self.lazy[y] = 0;
        }
    }

    fn update(&mut self, mut l: usize, mut r: usize, val: i32) {
        l += self.len; // 找在tree的叶子节点index
        r += self.len;
        let l0 = l;
        let r0 = r;
        while l <= r {
            if l & 1 == 1 { //  为什么l&1==1才apply？不好理解。。
                self.apply(l, val);
                l += 1;
            }
            if r & 1 == 0 {
                self.apply(r, val);
                r -= 1;
            }
            l >>= 1;  // 上移一层
            r >>= 1;
        }
        self.pull(l0);
        self.pull(r0);
    }

    fn query(&mut self, mut l: usize, mut r: usize) -> i32 {
        l += self.len;
        r += self.len;
        let mut result = 0;
        self.push(l);
        self.push(r);
        while l <= r {
            if l & 1 == 1 {
                result = result.max(self.tree[l]);
                l += 1;
            }
            if r & 1 == 0 {
                result = result.max(self.tree[r]);
                r -= 1;
            }
            l >>= 1;
            r >>= 1;
        }
        result
    }
}

pub fn falling_squares_compress_segment_tree(positions: Vec<Vec<i32>>) -> Vec<i32> {
    let len = positions.len();
    let mut set = BTreeSet::new();
    for pos in &positions {
        set.insert(pos[0]);
        set.insert(pos[0] + pos[1] - 1);
    }
    let axis: Vec<i32> = set.into_iter().collect();
    let mut m = HashMap::new();
    for i in 0..axis.len() {
        m.insert(axis[i], i);
    }
    let mut cur_max = 0;
    let mut result = Vec::with_capacity(len);
    let mut tree = SegmentTree::new(axis.len());
    for pos in positions {
        let l = m[&pos[0]];
        let r = m[&(pos[0] + pos[1] - 1)];
        let h = tree.query(l, r) + pos[1];
        tree.update(l, r, h);
        cur_max = cur_max.max(h);
        result.push(cur_max);
    }
    result
}


fn main() {
    fn test(func: fn(positions: Vec<Vec<i32>>) -> Vec<i32>) {
        assert_eq!(func(vec![vec![1, 2], vec![2, 3], vec![6, 1]]), vec![2, 5, 5]);
        assert_eq!(func(vec![vec![100, 100], vec![200, 100]]), vec![100, 100]);
        assert_eq!(func(vec![vec![3, 2], vec![8, 3], vec![1, 4], vec![8, 10], vec![9, 3]]), vec![2, 3, 6, 13, 16]);
        assert_eq!(func(vec![vec![3, 2], vec![9, 8], vec![4, 4]]), vec![2, 8, 8]);
        assert_eq!(func(vec![vec![100, 100], vec![200, 100]]), vec![100, 100]);
        assert_eq!(func(vec![vec![4, 9], vec![8, 8], vec![6, 8], vec![8, 2], vec![1, 2]]), vec![9, 17, 25, 27, 27]);
        assert_eq!(func(vec![vec![9, 7], vec![1, 9], vec![3, 1]]), vec![7, 16, 17]);
        assert_eq!(func(vec![vec![1, 2], vec![2, 3], vec![6, 1]]), vec![2, 5, 5]);
    }
    test(falling_squares);
    test(falling_squares_compress);
    test(falling_squares_compress_segment_tree);
}
