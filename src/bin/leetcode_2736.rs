//! 最大和查询

use std::cmp::Ordering;

/// 线段树，从小到达遍历
pub fn maximum_sum_queries(nums1: Vec<i32>, nums2: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let len = nums1.len();
    let mut n2: Vec<(i32, i32, usize)> = (0..len).map(|x| (nums2[x], nums1[x] + nums2[x], x)).collect();
    n2.sort_unstable();
    let sum: Vec<i32> = n2.iter().map(|x| x.1).collect();
    let mut st = SegmentTree::new(&sum);
    let mut oi2ni = vec![0; len];
    for i in 0..len {
        oi2ni[n2[i].2] = i;
    }
    let qlen = queries.len();
    let mut queries: Vec<(i32, i32, usize)> = queries.into_iter().zip(0..).map(|(x, i)| (x[0], x[1], i)).collect();
    let mut result = vec![-1; qlen];
    let mut n1: Vec<(i32, usize)> = nums1.into_iter().zip(0..).collect();
    n1.sort_unstable();
    let mut n1i = 0;
    queries.sort_unstable();
    for (x, y, i) in queries {
        while n1i < len && n1[n1i].0 < x {
            let oi = n1[n1i].1;
            let n2i = oi2ni[oi];
            st.update(n2i, -1);
            n1i += 1;
        }
        let start = n2.binary_search_by(|x| x.0.cmp(&y).then(Ordering::Greater)).unwrap_err();
        result[i] = st.query(start, len - 1);
    }
    result
}

/// 树状数组维护后缀最大值，从大到小遍历
pub fn maximum_sum_queries2(nums1: Vec<i32>, nums2: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let len = nums1.len();
    // 对 nums2[i], y 离散化
    let mut copy = Vec::with_capacity(len + queries.len());
    for &num in &nums2 {
        copy.push(num);
    }
    for q in &queries {
        copy.push(q[1]);
    }
    copy.sort_unstable();
    copy.dedup();

    // 对 nums1[i], x 降序排序
    let mut queries: Vec<(i32, i32, usize)> = queries.into_iter().zip(0..).map(|(x, i)| (x[0], x[1], i)).collect();
    queries.sort_unstable_by_key(|x| -x.0);
    let mut nums: Vec<(i32, i32)> = nums1.into_iter().zip(nums2).collect();
    nums.sort_unstable_by_key(|x| -x.0);

    let mut ni = 0;
    let mut result = vec![0; queries.len()];
    let mut bit = BitTree::new(copy.len());
    for (x, y, i) in queries {
        while ni < len && nums[ni].0 >= x {
            // (nums2[i], nums1[i] + nums2[i]) 加入到树状数组里
            bit.update(copy.binary_search(&nums[ni].1).unwrap() as i32 + 1, nums[ni].0 + nums[ni].1);
            ni += 1;
        }
        // 用树状数组求后缀最大值
        result[i] = bit.query(copy.binary_search(&y).unwrap() as i32 + 1);
    }
    result
}

/// 单调栈
pub fn maximum_sum_queries3(nums1: Vec<i32>, nums2: Vec<i32>, mut queries: Vec<Vec<i32>>) -> Vec<i32> {
    let len = nums1.len();
    let mut nums: Vec<(i32, i32)> = nums1.into_iter().zip(nums2).collect();
    nums.sort_unstable_by_key(|x| -x.0);
    for i in 0..queries.len() {
        queries[i].push(i as i32);
    }
    queries.sort_unstable_by_key(|x| -x[0]);
    let mut result = vec![-1; queries.len()];
    let mut st: Vec<(i32, i32)> = vec![];
    let mut i = 0;
    for q in queries {
        let (x, y, query_id) = (q[0], q[1], q[2] as usize);
        while i < len && nums[i].0 >= x {
            let (n1, n2) = nums[i];
            // n2 >= st[-1].0
            while !st.is_empty() && st.last().unwrap().1 <= n1 + n2 {
                st.pop();
            }
            if st.is_empty() || st.last().unwrap().0 < n2 {
                st.push((n2, n1 + n2));
            }
            i += 1;
        }
        let j = st.binary_search_by(|x| x.0.cmp(&y).then(Ordering::Greater)).unwrap_err();
        if j < st.len() {
            result[query_id] = st[j].1;
        }
    }
    result
}

/// 最大值，单点更新线段树
struct SegmentTree {
    tree: Vec<i32>,
    len: usize,
}

impl SegmentTree {
    fn new(arr: &Vec<i32>) -> Self {
        let len = arr.len();
        let mut tree = Self { tree: vec![0; len * 4], len };
        tree.build(arr, 0, len - 1, 1);
        tree
    }

    fn build(&mut self, arr: &Vec<i32>, s: usize, t: usize, p: usize) {
        if s == t {
            self.tree[p] = arr[s];
            return;
        }
        let m = (s + t) / 2;
        self.build(arr, s, m, p * 2);
        self.build(arr, m + 1, t, p * 2 + 1);
        self.tree[p] = self.tree[p * 2].max(self.tree[p * 2 + 1]);
    }

    fn _update(&mut self, i: usize, s: usize, t: usize, p: usize, val: i32) {
        if s == t && s == i {
            self.tree[p] = val;
            return;
        }
        let mid = (s + t) / 2;
        if i <= mid { self._update(i, s, mid, p * 2, val); } else { self._update(i, mid + 1, t, p * 2 + 1, val); }
        self.tree[p] = self.tree[p * 2].max(self.tree[p * 2 + 1]);
    }

    fn update(&mut self, i: usize, val: i32) {
        self._update(i, 0, self.len - 1, 1, val)
    }

    fn _query(&mut self, l: usize, r: usize, s: usize, t: usize, p: usize) -> i32 {
        if s >= l && t <= r {
            return self.tree[p];
        }
        let mid = (s + t) / 2;
        let mut ans = -1;
        if l <= mid { ans = ans.max(self._query(l, r, s, mid, p * 2)); }
        if r > mid { ans = ans.max(self._query(l, r, mid + 1, t, p * 2 + 1)); }
        ans
    }

    fn query(&mut self, l: usize, r: usize) -> i32 {
        self._query(l, r, 0, self.len - 1, 1)
    }
}

fn lowbit(x: i32) -> i32 {
    x & -x
}

/// 维护后缀最大值树状数组
struct BitTree {
    tree: Vec<i32>,
}

impl BitTree {
    fn new(n: usize) -> Self {
        Self { tree: vec![-1; n + 1] }
    }
    fn query(&mut self, mut x: i32) -> i32 {
        let mut result = -1;
        while x < self.tree.len() as i32 {
            result = result.max(self.tree[x as usize]);
            x += lowbit(x);
        }
        result
    }
    fn update(&mut self, mut x: i32, val: i32) {
        while x > 0 {
            self.tree[x as usize] = self.tree[x as usize].max(val);
            x -= lowbit(x);
        }
    }
}

fn main() {
    fn test(func: fn(nums1: Vec<i32>, nums2: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32>) {
        assert_eq!(func(vec![6], vec![50], vec![vec![79, 91]]), vec![-1]);
        assert_eq!(func(vec![4, 3, 1, 10], vec![2, 4, 9, 5], vec![vec![4, 1], vec![1, 3], vec![2, 5]]), vec![15, 15, 15]);
        assert_eq!(func(vec![4, 3, 1, 2], vec![2, 4, 9, 5], vec![vec![4, 1], vec![1, 3], vec![2, 5]]), vec![6, 10, 7]);
        assert_eq!(func(vec![3, 2, 5], vec![2, 3, 4], vec![vec![4, 4], vec![3, 2], vec![1, 1]]), vec![9, 9, 9]);
        assert_eq!(func(vec![2, 1], vec![2, 3], vec![vec![3, 3]]), vec![-1]);
    }
    test(maximum_sum_queries);
    test(maximum_sum_queries2);
    test(maximum_sum_queries3);
}
