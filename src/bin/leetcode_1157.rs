//! 子数组中占绝大多数的元素

use std::collections::HashMap;

struct MajorityChecker {
    idx: HashMap<i32, Vec<i32>>,
}


impl MajorityChecker {
    fn new(arr: Vec<i32>) -> Self {
        let mut idx: HashMap<i32, Vec<i32>> = HashMap::new();
        for (i, &v) in arr.iter().enumerate() {
            idx.entry(v).or_default().push(i as i32);
        }
        Self { idx }
    }

    fn query(&self, left: i32, right: i32, threshold: i32) -> i32 {
        for (k, v) in &self.idx {
            let left_idx = v.binary_search(&left).unwrap_or_else(|x| x);
            let right_idx = v.binary_search(&right).map(|x| x + 1).unwrap_or_else(|x| x);
            if right_idx - left_idx >= threshold as usize {
                return *k;
            }
        }
        -1
    }
}

// ----------------------------------
/// 线段树，摩尔投票维护众数
struct MajorityChecker2 {
    data: Vec<i32>,
    tree: Vec<(i32, i32)>,
    dict: HashMap<i32, Vec<i32>>,
}


impl MajorityChecker2 {
    fn new(arr: Vec<i32>) -> Self {
        let mut dict: HashMap<i32, Vec<i32>> = HashMap::new();
        for (i, &v) in arr.iter().enumerate() {
            dict.entry(v).or_default().push(i as i32);
        }
        let len = arr.len();
        let mut result = Self { data: arr, tree: vec![(0, 0); len * 4], dict };
        result.build(0, 0, len - 1);
        result
    }

    fn build(&mut self, p: usize, s: usize, t: usize) {
        if s == t {
            self.tree[p] = (self.data[s], 1);
            return;
        }
        let mid = (s + t) / 2;
        self.build(p * 2 + 1, s, mid);
        self.build(p * 2 + 2, mid + 1, t);
        self.tree[p] = Self::merge(self.tree[p * 2 + 1], self.tree[p * 2 + 2]);
    }

    fn merge(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
        return if a.0 == b.0 {
            (a.0, a.1 + b.1)
        } else {
            if a.1 > b.1 {
                (a.0, a.1 - b.1)
            } else if a.1 < b.1 {
                (b.0, b.1 - a.1)
            } else {
                (0, 0)
            }
        };
    }

    fn tree_query(&self, l: usize, r: usize, s: usize, t: usize, p: usize) -> (i32, i32) {
        if s >= l && t <= r {
            return self.tree[p];
        }
        let mid = (s + t) / 2;
        if l > mid {
            return self.tree_query(l, r, mid + 1, t, p * 2 + 2);
        }
        if r <= mid {
            return self.tree_query(l, r, s, mid, p * 2 + 1);
        }
        Self::merge(self.tree_query(l, r, s, mid, p * 2 + 1), self.tree_query(l, r, mid + 1, t, p * 2 + 2))
    }

    fn query(&self, left: i32, right: i32, threshold: i32) -> i32 {
        let (k, cnt) = self.tree_query(left as usize, right as usize, 0, self.data.len() - 1, 0);
        if cnt <= 0 { return -1; }
        let left_idx = self.dict[&k].binary_search(&left).unwrap_or_else(|x| x);
        let right_idx = self.dict[&k].binary_search(&right).map(|x| x + 1).unwrap_or_else(|x| x);
        if right_idx - left_idx >= threshold as usize {
            return k;
        }
        -1
    }
}


fn main() {
    let mc = MajorityChecker::new(vec![1, 1, 2, 2, 1, 1]);
    assert_eq!(mc.query(0, 5, 4), 1); // 返回 1
    assert_eq!(mc.query(0, 3, 3), -1); // 返回 -1
    assert_eq!(mc.query(2, 3, 2), 2); // 返回 2

    let mc = MajorityChecker::new(vec![1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1]);
    assert_eq!(mc.query(3, 12, 6), 2);

    let mc = MajorityChecker2::new(vec![1, 1, 2, 2, 1, 1]);
    assert_eq!(mc.query(0, 5, 4), 1); // 返回 1
    assert_eq!(mc.query(0, 3, 3), -1); // 返回 -1
    assert_eq!(mc.query(2, 3, 2), 2); // 返回 2

    let mc = MajorityChecker2::new(vec![1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1]);
    assert_eq!(mc.query(3, 12, 6), 2);
}
