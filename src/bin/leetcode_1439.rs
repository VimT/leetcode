//! 有序矩阵中的第 k 个最小数组和

use std::collections::{BinaryHeap, HashSet};


/// bfs + 小顶堆。
pub fn kth_smallest(mat: Vec<Vec<i32>>, k: i32) -> i32 {
    let m = mat.len();
    let n = mat[0].len();
    let mut heap = BinaryHeap::with_capacity((k * k) as usize);
    let mut init = vec![0; m + 1];
    for i in 0..m {
        init[0] -= mat[i][0];
    }
    heap.push(init);
    let mut seen = HashSet::new();
    for _ in 1..k {
        let x = heap.pop().unwrap();
        for i in 0..m {
            if x[i + 1] + 1 < n as i32 {
                let mut next = x.clone();
                next[i + 1] += 1;
                next[0] -= mat[i][next[i + 1] as usize] - mat[i][next[i + 1] as usize - 1];
                if seen.insert(next.clone()) {
                    heap.push(next);
                }
            }
        }
    }
    -heap.pop().unwrap()[0]
}

/// 二分查找：数组和满足单调性，找<=target的刚好有k个的
pub fn kth_smallest2(mat: Vec<Vec<i32>>, k: i32) -> i32 {
    struct DFS<'a> {
        mat: &'a Vec<Vec<i32>>,
        k: i32,
        cnt: i32,
        target: i32,
    }
    impl<'a> DFS<'a> {
        fn dfs(&mut self, i: usize, sum: i32) {
            if i == self.mat.len() || sum > self.target || self.cnt > self.k { return; }
            self.dfs(i + 1, sum);
            for j in 1..self.mat[0].len() {
                let new_sum = sum + self.mat[i][j] - self.mat[i][0];
                if new_sum > self.target { break; } //剪枝
                self.cnt += 1;
                self.dfs(i + 1, new_sum);
            }
        }
    }
    let mut left = 0;
    let mut right = 0;
    let m = mat.len();
    let n = mat[0].len();
    for i in 0..m {
        left += mat[i][0];
        right += mat[i][n - 1];
    }
    let sum = left;
    while left < right {
        let mid = (left + right) / 2;
        let mut f = DFS { mat: &mat, k, cnt: 1, target: mid };
        f.dfs(0, sum);
        if f.cnt >= k {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}

fn main() {
    fn test(func: fn(mat: Vec<Vec<i32>>, k: i32) -> i32) {
        assert_eq!(func(vec![vec![1, 3, 11], vec![2, 4, 6]], 5), 7);
        assert_eq!(func(vec![vec![1, 3, 11], vec![2, 4, 6]], 9), 17);
        assert_eq!(func(vec![vec![1, 10, 10], vec![1, 4, 5], vec![2, 3, 6]], 7), 9);
    }
    test(kth_smallest);
    test(kth_smallest2);
}
