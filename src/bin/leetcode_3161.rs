//! 物块放置查询

use std::collections::BTreeSet;

use leetcode::segment_tree::{SimpleSegmentSpec, SimpleSegmentTree};

/// 线段树
pub fn get_results(queries: Vec<Vec<i32>>) -> Vec<bool> {
    #[derive(Clone, Debug, Default)]
    struct Value {
        // 区间中间是否有分割
        mid_split: bool,
        // 区间左边是否有分割
        left_split: bool,
        // 左边的最大空位
        left: i32,
        // 中间的最大空位
        mid: i32,
        // 右边的最大空位
        right: i32,
    }

    impl Value {
        fn new() -> Self { return Self { mid_split: false, left_split: false, left: 1, mid: 0, right: 1 }; }
        fn split() -> Self { return Self { mid_split: false, left_split: true, left: 0, mid: 0, right: 1 }; }
        fn max(&self) -> i32 { return self.left.max(self.mid).max(self.right); }
    }
    enum Spec {}
    impl SimpleSegmentSpec for Spec {
        type ValueType = Value;
        fn identify() -> Value {
            Value::default()
        }

        fn combine(a: &Value, b: &Value) -> Value {
            Value {
                mid_split: b.left_split || a.mid_split || b.mid_split,
                left_split: a.left_split,
                left: a.left + if !a.mid_split && !b.left_split { b.left } else { 0 },
                mid: a.mid.max(b.mid).max(
                    if b.left_split {
                        a.right.max(b.left)
                    } else {
                        a.right + b.left
                    }),
                right: b.right + if !b.mid_split && !b.left_split { a.right } else { 0 },
            }
        }
    }

    let mx = queries.iter().map(|x| x[1]).max().unwrap() as usize + 1;
    let mut tree = SimpleSegmentTree::<Spec>::new(&vec![Value::new(); mx]);
    queries.into_iter().filter_map(|q| {
        let x = q[1] as usize;
        match q[0] {
            1 => {
                tree.update(x, Value::split());
                None
            }
            2 => {
                let value = tree.query(0, x - 1);
                // println!("{q:?} {value:?}");
                Some(value.max() >= q[2])
            }
            _ => unreachable!()
        }
    }).collect()
}

/// 简化：平衡树+线段树
pub fn get_results2(queries: Vec<Vec<i32>>) -> Vec<bool> {
    let mx = queries.iter().map(|x| x[1]).max().unwrap() as usize + 1;
    enum Max {}
    impl SimpleSegmentSpec for Max {
        type ValueType = usize;
        fn identify() -> usize { return 0; }
        fn combine(&a: &usize, &b: &usize) -> usize { return a.max(b); }
    }
    // 线段树维护区间内最大的空隙
    let mut tree = SimpleSegmentTree::<Max>::new(&vec![0; mx + 1]);
    let mut sl: BTreeSet<usize> = BTreeSet::new();
    sl.insert(0);
    sl.insert(mx);
    queries.into_iter().filter_map(|q| {
        let x = q[1] as usize;
        let pre = *sl.range(..x).last().unwrap(); // x左侧最近障碍物的位置

        match q[0] {
            1 => {
                let nxt = *sl.range(x + 1..).next().unwrap(); // x右侧最接近障碍物的位置
                tree.update(x, x - pre);
                tree.update(nxt, nxt - x);
                sl.insert(x);
                None
            }
            2 => {
                let value = tree.query(0, pre); // [0, pre] 中最大的d
                let max_gap = value.max(x - pre);
                Some(max_gap >= q[2] as usize)
            }
            _ => unreachable!()
        }
    }).collect()
}

fn main() {
    fn test(func: fn(queries: Vec<Vec<i32>>) -> Vec<bool>) {
        assert_eq!(func(vec![vec![1, 28], vec![2, 86, 82], vec![2, 18, 121], vec![2, 147, 101], vec![2, 41, 65], vec![2, 3, 74], vec![2, 74, 141], vec![2, 13, 143], vec![2, 135, 135], vec![2, 81, 54], vec![1, 99], vec![2, 73, 122], vec![2, 35, 120], vec![2, 142, 69], vec![2, 102, 118], vec![1, 38], vec![1, 72], vec![2, 52, 15], vec![1, 59], vec![2, 101, 46], vec![1, 132], vec![1, 114], vec![2, 70, 30], vec![2, 120, 37], vec![1, 9], vec![2, 15, 39], vec![1, 30], vec![1, 84], vec![1, 34], vec![1, 97], vec![1, 91], vec![1, 69], vec![2, 150, 84], vec![1, 130], vec![2, 131, 27], vec![2, 122, 101], vec![1, 23], vec![2, 111, 141], vec![2, 9, 119], vec![2, 29, 102], vec![1, 118], vec![2, 14, 79], vec![1, 105], vec![2, 98, 148], vec![2, 89, 74], vec![2, 61, 54], vec![2, 97, 20], vec![2, 141, 66], vec![2, 7, 78], vec![2, 127, 12]]), vec![false, false, true, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true]);
        assert_eq!(func(vec![vec![1, 4], vec![2, 4, 4]]), vec![true]);
        assert_eq!(func(vec![vec![2, 1, 2]]), vec![false]);
        assert_eq!(func(vec![vec![1, 7], vec![2, 7, 6], vec![1, 2], vec![2, 7, 5], vec![2, 7, 6]]), vec![true, true, false]);
        assert_eq!(func(vec![vec![1, 2], vec![2, 3, 3], vec![2, 3, 1], vec![2, 2, 2]]), vec![false, true, true]);
    }
    test(get_results);
    test(get_results2);
}
