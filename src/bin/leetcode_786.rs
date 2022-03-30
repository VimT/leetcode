//! 第 K 个最小的素数分数

use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
    let len = arr.len();
    let mut frac = Vec::with_capacity(len * len);
    for i in 0..len {
        for j in i + 1..len {
            frac.push((arr[i], arr[j]));
        }
    }
    frac.sort_unstable_by(|x, y| (x.0 * y.1).cmp(&(x.1 * y.0)));
    let k = k as usize;
    vec![frac[k - 1].0, frac[k - 1].1]
}

/// 合并k个有序数组
pub fn kth_smallest_prime_fraction_priority_queue(arr: Vec<i32>, k: i32) -> Vec<i32> {
    #[derive(PartialEq, Eq)]
    struct Fen<'a> {
        arr: &'a Vec<i32>,
        zi: usize,
        mu: usize,
    }

    impl<'a> Fen<'a> {
        pub fn new(arr: &'a Vec<i32>, zi: usize, mu: usize) -> Self {
            Fen { arr, zi, mu }
        }
    }
    impl<'a> PartialOrd for Fen<'a> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            (self.arr[self.mu] * other.arr[other.zi]).partial_cmp(&(self.arr[self.zi] * other.arr[other.mu]))
        }
    }
    impl<'a> Ord for Fen<'a> {
        fn cmp(&self, other: &Self) -> Ordering {
            (self.arr[self.mu] * other.arr[other.zi]).cmp(&(self.arr[self.zi] * other.arr[other.mu]))
        }
    }

    let len = arr.len();
    let mut q = BinaryHeap::new();
    for j in 1..len {
        q.push(Fen::new(&arr, 0, j));
    }
    for _ in 1..k {
        let top = q.pop().unwrap();
        if top.zi + 1 < top.mu {
            q.push(Fen::new(&arr, top.zi + 1, top.mu));
        }
    }
    let result = q.pop().unwrap();
    vec![arr[result.zi], arr[result.mu]]
}

pub fn kth_smallest_prime_fraction_binary_search(arr: Vec<i32>, k: i32) -> Vec<i32> {
    let len = arr.len();
    let mut left = 0.0;
    let mut right = 1.0;
    loop {
        let mid = (left + right) / 2.0;
        let mut i = -1;
        let mut count = 0;
        let mut x = 0;
        let mut y = 1;
        for j in 1..len {
            while (arr[(i + 1) as usize] as f64 / arr[j] as f64) < mid {
                i += 1;
                if arr[i as usize] * y > arr[j] * x {
                    x = arr[i as usize];
                    y = arr[j];
                }
            }
            count += i + 1;
        }
        if count == k {
            return vec![x, y];
        }
        if count < k {
            left = mid;
        } else {
            right = mid;
        }
    }
}

fn main() {
    assert_eq!(kth_smallest_prime_fraction_binary_search(vec![1, 7, 23, 29, 47], 8), vec![23, 47]);
    assert_eq!(kth_smallest_prime_fraction_binary_search(vec![1, 2, 3, 5], 3), vec![2, 5]);
    assert_eq!(kth_smallest_prime_fraction_binary_search(vec![1, 7], 1), vec![1, 7]);
}
