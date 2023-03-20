//! LCP 32. 批量处理任务

use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, BTreeMap, HashMap, HashSet};
use leetcode::graph::Graph;

/// 按照task[1]从小到大排序，贪心选择每个task的后面时时间段（后面的task可以复用）
/// 问题转化成如何维护哪些区间已经使用，每个task花 period - 区间内已经用了的 时间。
/// 做法1：暴力维护这个区间列表
pub fn process_tasks(mut tasks: Vec<Vec<i32>>) -> i32 {
    tasks.sort_unstable_by_key(|x| (x[1], x[0]));
    let mut ranges: Vec<(i32, i32)> = vec![];
    for task in tasks {
        let (start, end, mut period) = (task[0], task[1], task[2]);
        if ranges.is_empty() || ranges.last().unwrap().1 < start {
            ranges.push((end - period + 1, end));
        } else {
            let mut i = ranges.len();
            // 前面的执行的区间 是本区间的子集 （right <= end && left >= start)
            while period > 0 && i > 0 {
                let (left, right) = ranges[i - 1];
                if left >= start {
                    period -= right - left + 1;
                } else { break; }
                i -= 1;
            }
            // 前面执行的区间是 本区间的交集
            if period > 0 && i > 0 {
                let (left, right) = ranges[i - 1];
                if right >= start && left < start {
                    period -= right - start + 1;
                }
            }
            // 插入本区间并区间合并
            if period > 0 {
                let mut now = end - period + 1;
                while !ranges.is_empty() && now <= ranges.last().unwrap().1 {
                    let (left, right) = ranges.pop().unwrap();
                    now = left - (right - now + 1);
                }
                ranges.push((now, end));
            }
        }
    }
    let mut result = 0;
    for (left, right) in ranges {
        result += right - left + 1;
    }
    result
}

/// 栈+二分查找 （时间最优）
/// 可以理解为对暴力的优化
pub fn process_tasks1(mut tasks: Vec<Vec<i32>>) -> i32 {
    tasks.sort_unstable_by_key(|x| x[1]);
    let mut stack = Vec::with_capacity(tasks.len());  // 维护在使用的区间1
    stack.push((-2, -2, 0));  // 闭区间的左右端点，栈底到栈顶的区间长度和
    for task in tasks {
        let (start, end, mut d) = (task[0], task[1], task[2]);
        let idx = stack.binary_search_by(|x| x.0.cmp(&start).then(Ordering::Greater)).unwrap_err();  // bisect_left(start)
        let (_, r, s) = stack[idx-1];
        d -= stack.last().unwrap().2 - s;  // 去掉运行中的时间点
        if start <= r { // start在区间st[i]内
            d -= r - start + 1;  // 去掉运行中的时间点
        }
        if d <= 0 { continue; }
        while end - stack.last().unwrap().1 <= d { // 剩余的d填充区间后缀
            let (l, r, _) = stack.pop().unwrap();
            d += r - l + 1;  // 合并区间
        }
        stack.push((end - d + 1, end, stack.last().unwrap().2 + d));
    }
    stack.last().unwrap().2
}

/// 做法2：用线段树维护已使用区间
pub fn process_tasks2(mut tasks: Vec<Vec<i32>>) -> i32 {
    #[derive(Default)]
    struct Node {
        sum: i32,
        lazy: bool,
        l: usize,
        r: usize,
    }
    // 动态开点线段树
    struct Seg {
        tree: Vec<Node>,
    }
    impl Seg {
        fn push_down(&mut self, s: usize, t: usize, p: usize) {
            if self.tree[p].l == 0 {
                self.tree.push(Node::default());
                self.tree[p].l = self.tree.len() - 1;
            }
            if self.tree[p].r == 0 {
                self.tree.push(Node::default());
                self.tree[p].r = self.tree.len() - 1;
            }
            if self.tree[p].lazy {
                let mid = (s + t) / 2;
                let l = self.tree[p].l;
                self.tree[l].sum = (mid - s + 1) as i32;
                self.tree[l].lazy = true;
                let r = self.tree[p].r;
                self.tree[r].sum = (t - mid) as i32;
                self.tree[r].lazy = true;
                self.tree[p].lazy = false;
            }
        }
        fn query(&mut self, l: usize, r: usize, s: usize, t: usize, p: usize) -> i32 {
            if self.tree[p].sum == 0 { return 0; }
            if s >= l && t <= r {
                return self.tree[p].sum;
            }
            self.push_down(s, t, p);
            let mid = (s + t) / 2;
            let mut result = 0;
            if l <= mid { result += self.query(l, r, s, mid, self.tree[p].l); }
            if r > mid { result += self.query(l, r, mid + 1, t, self.tree[p].r); }
            result
        }
        // 对区间[l,r]的后缀+suffix个时间点，线段树二分+线段树更新
        fn update(&mut self, l: usize, r: usize, s: usize, t: usize, p: usize, suffix: &mut i32) {
            let size = (t - s + 1) as i32;
            if self.tree[p].sum == size { return; }  // 这个区间已经满了
            if s >= l && t <= r && size - self.tree[p].sum <= *suffix {  // 这个区间可以全放下
                *suffix -= size - self.tree[p].sum;
                self.tree[p].sum = size;
                self.tree[p].lazy = true;
                return;
            }
            let mid = (s + t) / 2;
            self.push_down(s, t, p);
            if r > mid { self.update(l, r, mid + 1, t, self.tree[p].r, suffix); } // 先放右区间
            if l <= mid && *suffix > 0 { self.update(l, r, s, mid, self.tree[p].l, suffix); }
            self.tree[p].sum = self.tree[self.tree[p].l].sum + self.tree[self.tree[p].r].sum;
        }
        fn new(n: usize) -> Self {
            let mut tree: Vec<Node> = Vec::with_capacity(n);
            tree.push(Node { sum: 0, lazy: false, l: 0, r: 0 });
            tree.push(Node { sum: 0, lazy: false, l: 0, r: 0 });
            Self { tree }
        }
    }
    tasks.sort_unstable_by_key(|x| x[1]);
    let mut seg = Seg::new(tasks.len() * 32);
    let max = tasks.last().unwrap()[1] as usize + 1;
    for task in tasks {
        let (start, end, d) = (task[0] as usize, task[1] as usize, task[2]);
        let mut suffix = d - seg.query(start, end, 0, max, 1); // d - 已有的
        if suffix > 0 {
            seg.update(start, end, 0, max, 1, &mut suffix); // 新增时间点
        }
    }
    seg.tree[1].sum
}

/// 做法3：用玛朵莉树维护已使用区间
/// 超时。
pub fn process_tasks3(mut tasks: Vec<Vec<i32>>) -> i32 {
    struct ODT {
        m: BTreeMap<i32, (i32, i32)>,
    }
    impl ODT {
        // 从x处分裂
        fn split(&mut self, x: i32) {
            let (&l, &(r, v)) = self.m.range(..=x).last().unwrap();
            if l == x { return; }
            self.m.remove(&l);
            self.m.insert(l, (x - 1, v));
            self.m.insert(x, (r, v));
        }
        // 区间赋值，可以让ODT大小下降
        fn assign(&mut self, l: i32, r: i32, v: i32) {
            self.split(r + 1);
            self.split(l);
            let will_remove: Vec<i32> = self.m.range(l..=r).map(|x| *x.0).collect();
            for i in will_remove {
                self.m.remove(&i);
            }
            self.m.insert(l, (r, v));
        }
        fn query(&mut self, l: i32, r: i32) -> i32 {
            self.split(l);
            self.split(r + 1);
            let mut result = 0;
            for (&s, &(t, v)) in self.m.range(l..=r) {
                result += (t - s + 1) * v;
            }
            result
        }
        fn update(&mut self, l: i32, r: i32, mut suffix: i32) {
            self.split(l);
            self.split(r + 1);
            let mut start = 0;
            for (&s, (t, v)) in self.m.range(l..=r).rev() {
                if *v == 0 {
                    let size = *t - s + 1;
                    if suffix >= size {
                        suffix -= size;
                        start = s;
                        if suffix == 0 { break; }
                    } else {
                        start = *t + 1 - suffix;
                        break;
                    }
                }
            }
            self.assign(start, r, 1);
        }
    }
    tasks.sort_unstable_by_key(|x| x[1]);
    let mut odt = ODT { m: BTreeMap::new() };
    let max = tasks.last().unwrap()[1] + 1;
    odt.m.insert(0, (max, 0));
    for task in tasks {
        let (start, end, d) = (task[0], task[1], task[2]);
        let suffix = d - odt.query(start, end); // d - 已有的
        if suffix > 0 {
            odt.update(start, end, suffix); // 新增时间点
        }
    }
    odt.query(0, max)
}


/// 做法4：用小顶堆维护 已经开始，但还没有启动的任务。（最晚启动时间，结束时间）
/// 为了比较谁需要更早启动，入队时 最晚启动时间-res，出队时最晚启动时间+res，这样得到入队-出队这段时间运行了多少 （妙啊）
pub fn process_tasks4(mut tasks: Vec<Vec<i32>>) -> i32 {
    tasks.push(vec![1e9 as i32 + 1, 1e9 as i32 + 1, 1]);
    let mut has_run_time = 0;
    let mut heap: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::with_capacity(tasks.len());
    tasks.sort_unstable();
    for task in tasks {
        let (start, end, period) = (task[0], task[1], task[2]);
        while !heap.is_empty() && heap.peek().unwrap().0.0 + has_run_time < start {
            let Reverse((latest_start_time, end_time)) = *heap.peek().unwrap();
            if latest_start_time + has_run_time >= end_time {
                heap.pop().unwrap();
            } else {
                has_run_time += end_time.min(start) - (latest_start_time + has_run_time); // 运行到end/start
            }
        }
        heap.push(Reverse((end - period + 1 - has_run_time, end + 1)));
    }
    has_run_time
}

/// 差分约束：符合 xj - xi <= wk 形式的不等式组，可以使用差分约束方法求解
/// 这道题：存在着差分约束关系。建图，求不等式组的最小解，用spfa求单源最长路
pub fn process_tasks5(tasks: Vec<Vec<i32>>) -> i32 {
    // 离散化
    let mut nums = HashSet::new();
    for task in &tasks {
        nums.insert(task[0]);
        nums.insert(task[1] + 1);
    }
    let mut nums: Vec<i32> = nums.into_iter().collect();
    nums.sort_unstable();
    let len = nums.len();
    let map: HashMap<i32, usize> = nums.iter().cloned().zip(0..).collect();
    let mut g = Graph::new(len, len * 3);
    for task in tasks {
        let u = map[&task[0]];
        let v = map[&(task[1] + 1)];
        g.add(u, v, task[2]);  // Xv - Xu >= period
    }
    for i in 1..len {
        g.add(i - 1, i, 0);
        g.add(i, i - 1, nums[i - 1] - nums[i]);
    }
    // 求单源最长路+栈优化
    fn spfa(g: &Graph<i32>, len: usize) -> Option<Vec<i32>> {
        let mut dist = vec![0; len];
        let mut q: Vec<usize> = (0..len).rev().collect();
        let mut count = vec![0; len];
        let mut inq = vec![true; len];
        while !q.is_empty() {
            let u = q.pop().unwrap();
            inq[u] = false;
            for (v, &weight) in g.neigh(u) {
                if dist[u] + weight > dist[v] {
                    dist[v] = dist[u] + weight;
                    count[v] = count[u] + 1;
                    if count[v] >= len + 1 {
                        return None;
                    }
                    if !inq[v] {
                        inq[v] = true;
                        q.push(v);
                    }
                }
            }
        }
        Some(dist)
    }
    if let Some(dist) = spfa(&g, len) {
        return *dist.last().unwrap();
    }
    -1
}

fn main() {
    fn test(func: fn(tasks: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![1, 3, 2], vec![2, 5, 3], vec![5, 6, 2]]), 4);
        assert_eq!(func(vec![vec![0, 0, 1], vec![1, 1, 1], vec![2, 2, 1]]), 3);
        assert_eq!(func(vec![vec![10, 42, 6], vec![47, 81, 35], vec![38, 58, 13], vec![39, 48, 4], vec![12, 62, 24], vec![54, 73, 1], vec![59, 96, 34], vec![65, 91, 20]]), 54);
        assert_eq!(func(vec![vec![2, 3, 1], vec![5, 5, 1], vec![5, 6, 2]]), 3);
    }
    test(process_tasks);
    test(process_tasks1);
    test(process_tasks2);
    test(process_tasks3);
    test(process_tasks4);
    test(process_tasks5);
}
