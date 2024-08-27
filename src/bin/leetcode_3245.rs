//! 交替组 III

use std::collections::BTreeSet;

struct BIT {
    tree: Vec<i32>,
}

impl BIT {
    fn new(n: usize) -> Self {
        BIT { tree: vec![0; n + 1] }
    }

    fn update(&mut self, mut pos: usize, val: i32) {
        while pos < self.tree.len() {
            self.tree[pos] += val;
            pos += pos & pos.wrapping_neg();
        }
    }

    fn query(&self, mut pos: usize) -> i32 {
        let mut ret = 0;
        while pos > 0 {
            ret += self.tree[pos];
            pos -= pos & pos.wrapping_neg();
        }
        ret
    }
}

/// https://leetcode.cn/problems/alternating-groups-iii/solutions/2868457/shu-zhuang-shu-zu-by-tsreaper-j2pg/
pub fn number_of_alternating_groups(mut colors: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = colors.len();
    let mut tree1 = BIT::new(n);
    let mut tree2 = BIT::new(n);

    let update = |tree1: &mut BIT, tree2: &mut BIT, l: usize, r: usize, pos: usize, k: i32| {
        let old = (r - l + n) % n;
        let old = if old == 0 { n } else { old };
        tree1.update(old, -k);
        tree2.update(old, -k * old as i32);

        let len = (pos - l + n) % n;
        let len = if len == 0 { n } else { len };
        tree1.update(len, k);
        tree2.update(len, k * len as i32);
        let len = (r + n - pos) % n;
        let len = if len == 0 { n } else { len };
        tree1.update(len, k);
        tree2.update(len, k * len as i32);
    };

    let mut st = BTreeSet::new();

    let ins = |st: &mut BTreeSet<usize>, tree1: &mut BIT, tree2: &mut BIT, pos: usize| {
        if st.is_empty() {
            st.insert(pos);
            tree1.update(n, 1);
            tree2.update(n, n as i32);
        } else {
            st.insert(pos);
            let l = *st.range(..pos).next_back().unwrap_or(st.iter().next_back().unwrap());
            let r = *st.range(pos + 1..).next().unwrap_or(st.iter().next().unwrap());
            update(tree1, tree2, l, r, pos, 1);
        }
    };

    let del = |st: &mut BTreeSet<usize>, tree1: &mut BIT, tree2: &mut BIT, pos: usize| {
        if st.len() == 1 {
            st.remove(&pos);
            tree1.update(n, -1);
            tree2.update(n, -(n as i32));
        } else {
            st.remove(&pos);
            let l = *st.range(..pos).next_back().unwrap_or(st.iter().next_back().unwrap());
            let r = *st.range(pos..).next().unwrap_or(st.iter().next().unwrap());
            update(tree1, tree2, l, r, pos, -1);
        }
    };

    for i in 0..n {
        if colors[i] == colors[(i + 1) % n] {
            ins(&mut st, &mut tree1, &mut tree2, i);
        }
    }

    let mut ans = Vec::new();
    for qry in queries {
        if qry[0] == 1 {
            if st.is_empty() {
                ans.push(n as i32);
            } else {
                let sm = tree2.query(n) - tree2.query(qry[1] as usize - 1);
                let cnt = tree1.query(n) - tree1.query(qry[1] as usize - 1);
                ans.push(sm - cnt * qry[1] + cnt);
            }
        } else {
            let idx = qry[1] as usize;
            let col = qry[2];
            let prv = (idx + n - 1) % n;
            let nxt = (idx + 1) % n;

            if colors[prv] == colors[idx] {
                del(&mut st, &mut tree1, &mut tree2, prv);
            }
            if colors[idx] == colors[nxt] {
                del(&mut st, &mut tree1, &mut tree2, idx);
            }

            colors[idx] = col;

            if colors[prv] == colors[idx] {
                ins(&mut st, &mut tree1, &mut tree2, prv);
            }
            if colors[idx] == colors[nxt] {
                ins(&mut st, &mut tree1, &mut tree2, idx);
            }
        }
    }
    ans
}

fn main() {
    fn test(func: fn(colors: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32>) {
        assert_eq!(func(vec![0, 1, 0, 1], vec![vec![1, 3], vec![2, 2, 1], vec![1, 3], vec![1, 3]]), vec![4, 1, 1]);
        assert_eq!(func(vec![0, 1, 1, 0, 1], vec![vec![2, 1, 0], vec![1, 4]]), vec![2]);
        assert_eq!(func(vec![0, 0, 1, 0, 1, 1], vec![vec![1, 3], vec![2, 3, 0], vec![1, 5]]), vec![2, 0]);
    }
    test(number_of_alternating_groups);
}
