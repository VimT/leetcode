//! 找到 Alice 和 Bob 可以相遇的建筑

use std::collections::BinaryHeap;

/// 二维偏序问题，常见使用RMQ算法（线段树，ST表）解决，也可以想办法离线做（数组数组，最小堆，单调栈）
/// 在线做法 - 线段树二分
pub fn leftmost_building_queries(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    fn build(i: usize, l: usize, r: usize, arr: &Vec<i32>, tree: &mut Vec<i32>) {
        if l == r {
            tree[i] = if l < arr.len() { arr[l] } else { 0 };
            return;
        }
        let mid = (r + l) / 2;
        build(i * 2, l, mid, arr, tree);
        build(i * 2 + 1, mid + 1, r, arr, tree);
        tree[i] = tree[i * 2].max(tree[i * 2 + 1]);
    }
    fn ask(i: usize, l: usize, r: usize, ll: usize, rr: usize, tree: &Vec<i32>) -> i32 {
        if l == ll && rr == r {
            return tree[i];
        }
        let mid = (r + l) / 2;
        if rr <= mid {
            ask(i * 2, l, mid, ll, rr, tree)
        } else if mid < ll {
            ask(i * 2 + 1, mid + 1, r, ll, rr, tree)
        } else {
            ask(i * 2, l, mid, ll, mid, tree).max(ask(i * 2 + 1, mid + 1, r, mid + 1, rr, tree))
        }
    }
    let len = heights.len();
    let mut tree = vec![0; heights.len() * 4 + 1];
    build(1, 0, len - 1, &heights, &mut tree);
    queries.iter().map(|q| {
        let (mut a, mut b) = (q[0] as usize, q[1] as usize);
        if a > b { std::mem::swap(&mut a, &mut b); }
        if heights[b] > heights[a] || a == b {
            return b as i32;
        }
        let mut l = b;
        let mut r = len;
        let mx = heights[a].max(heights[b]);
        if ask(1, 0, len - 1, b, len - 1, &tree) <= mx {
            return -1;
        }
        while l < r {
            let mid = (l + r) / 2;
            let lc = ask(1, 0, len - 1, l, mid, &tree);
            if lc > mx {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        l as i32
    }).collect()
}


/// 在线做法 - ST表二分
pub fn leftmost_building_queries2(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let len = heights.len();

    struct SparseTable {
        max: Vec<Vec<i32>>,
    }

    impl SparseTable {
        fn new(array: &Vec<i32>) -> Self {
            let len = array.len();
            let m = 63 - len.leading_zeros() as usize;  // len.log2()
            let mut max = vec![vec![0; m + 1]; len + 2];
            for i in 1..=len {
                max[i][0] = array[i - 1];
            }
            for i in 1..=m {
                for j in 1..=len + 1 - (1 << i) {
                    max[j][i] = max[j][i - 1].max(max[j + (1 << (i - 1))][i - 1]);
                }
            }
            SparseTable { max }
        }

        fn ask(&self, l: usize, r: usize) -> i32 {
            let x = r + 1 - l;
            let k = 63 - x.leading_zeros() as usize;  // x.log2()
            self.max[l + 1][k].max(self.max[r + 2 - (1 << k)][k])
        }
    }

    let st = SparseTable::new(&heights);
    queries.iter().map(|q| {
        let (mut a, mut b) = (q[0] as usize, q[1] as usize);
        if a > b { std::mem::swap(&mut a, &mut b); }
        if heights[b] > heights[a] || a == b {
            return b as i32;
        }
        let mut l = b;
        let mut r = len;
        let mx = heights[a].max(heights[b]);
        if st.ask(b, len - 1) <= mx {
            return -1;
        }
        while l < r {
            let mid = (l + r) / 2;
            let lc = st.ask(l, mid);
            if lc > mx {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        l as i32
    }).collect()
}


/// 离线做法 - 树状数组
pub fn leftmost_building_queries3(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    // 树状数组，后缀最小值
    fn lowbit(x: i32) -> i32 {
        x & -x
    }
    fn query(tree: &Vec<i32>, mut x: i32) -> i32 {
        let mut result = i32::MAX;
        while x < tree.len() as i32 {
            result = result.min(tree[x as usize]);
            x += lowbit(x);
        }
        result
    }
    fn update(tree: &mut Vec<i32>, mut x: i32, val: i32) {
        while x > 0 {
            tree[x as usize] = tree[x as usize].min(val);
            x -= lowbit(x);
        }
    }

    let len = heights.len();
    let mut tree = vec![i32::MAX; len + 1];

    let mut result = vec![-1; queries.len()];
    let mut q = Vec::with_capacity(queries.len());
    for (i, query) in queries.into_iter().enumerate() {
        let (mut a, mut b) = (query[0] as usize, query[1] as usize);
        if a > b { std::mem::swap(&mut a, &mut b); }
        if a == b || heights[a] < heights[b] {
            result[i] = b as i32;
        } else {
            q.push((heights[a], i, b));
        }
    }
    q.sort_unstable();
    let mut hi: Vec<(i32, usize)> = heights.into_iter().zip(0..).collect();
    hi.sort_unstable();
    let mut j = q.len();
    for (height, i) in hi.into_iter().rev() {
        while j > 0 && q[j - 1].0 == height {
            let (_, qi, b) = q[j - 1];
            let x = query(&tree, b as i32 + 1);  // 当前树状数组存的都是 > height的，查询 >= b 位置的最小位置。
            result[qi] = if x == i32::MAX { -1 } else { x };
            j -= 1;
        }
        update(&mut tree, i as i32 + 1, i as i32);  // 更新 >= i 的最小index
    }
    result
}

/// 离线做法 - 最小堆
/// query 从小到大排序，从左到右遍历 height。遍历过程中把 query 用最小堆存起来。如果当前 height > 堆顶，则找到答案。
pub fn leftmost_building_queries4(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result = vec![-1; queries.len()];
    let mut left = vec![vec![]; heights.len()];
    for (i, query) in queries.into_iter().enumerate() {
        let (mut a, mut b) = (query[0] as usize, query[1] as usize);
        if a > b { std::mem::swap(&mut a, &mut b); }
        if a == b || heights[a] < heights[b] {
            result[i] = b as i32;
        } else {
            left[b].push((heights[a], i));
        }
    }
    let mut heap: BinaryHeap<(i32, usize)> = BinaryHeap::new();
    for (i, height) in heights.into_iter().enumerate() {
        while !heap.is_empty() && -heap.peek().unwrap().0 < height {
            result[heap.pop().unwrap().1] = i as i32;
        }
        for &(qh, qi) in &left[i] {
            heap.push((-qh, qi));
        }
    }
    result
}


/// 离线做法 - 单调栈
pub fn leftmost_building_queries5(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result = vec![-1; queries.len()];
    let mut pos = vec![vec![]; heights.len()];
    for (i, query) in queries.into_iter().enumerate() {
        let (mut a, mut b) = (query[0] as usize, query[1] as usize);
        if a > b { std::mem::swap(&mut a, &mut b); }
        if a == b || heights[a] < heights[b] {
            result[i] = b as i32;
        } else {
            pos[b].push((heights[a], i));
        }
    }
    let mut st: Vec<(i32, i32)> = vec![(i32::MAX, -1)];  // 单调减
    for (i, height) in heights.into_iter().enumerate().rev() {
        while st.last().unwrap().0 <= height {
            st.pop().unwrap();
        }
        for &(qh, qi) in &pos[i] {
            let x = st.binary_search_by(|x| x.0.cmp(&qh).then(std::cmp::Ordering::Less).reverse()).unwrap_err();  // 找 > qh 的最小位置。x==0说明没有找到。
            result[qi] = st[x - 1].1;
        }
        st.push((height, i as i32));
    }
    result
}

fn main() {
    fn test(func: fn(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32>) {
        assert_eq!(func(vec![5, 3, 8, 2, 6, 1, 4, 6], vec![vec![0, 7], vec![3, 5], vec![5, 2], vec![3, 0], vec![1, 6]]), vec![7, 6, -1, 4, 6]);
        assert_eq!(func(vec![6, 4, 8, 5, 2, 7], vec![vec![0, 1], vec![0, 3], vec![2, 4], vec![3, 4], vec![2, 2]]), vec![2, 5, -1, 5, 2]);
        assert_eq!(func(vec![1, 2, 1, 2, 1, 2], vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![0, 5], vec![1, 0], vec![1, 1], vec![1, 2], vec![1, 3], vec![1, 4], vec![1, 5], vec![2, 0], vec![2, 1], vec![2, 2], vec![2, 3], vec![2, 4], vec![2, 5], vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 3], vec![3, 4], vec![3, 5], vec![4, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![4, 4], vec![4, 5], vec![5, 0], vec![5, 1], vec![5, 2], vec![5, 3], vec![5, 4], vec![5, 5]]), vec![0, 1, 3, 3, 5, 5, 1, 1, -1, -1, -1, -1, 3, -1, 2, 3, 5, 5, 3, -1, 3, 3, -1, -1, 5, -1, 5, -1, 4, 5, 5, -1, 5, -1, 5, 5]);
    }
    test(leftmost_building_queries);
    test(leftmost_building_queries2);
    test(leftmost_building_queries3);
    test(leftmost_building_queries4);
    test(leftmost_building_queries5);
}
