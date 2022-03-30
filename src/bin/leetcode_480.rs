//! 滑动窗口中位数


use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::collections::hash_map::Entry;

// 20ms
pub fn median_sliding_window_default(mut nums: Vec<i32>, k: i32) -> Vec<f64> {
    let k = k as usize;
    let len = nums.len();
    let mut ans = Vec::with_capacity(len);
    let mut h = vec![(0, 0); k];
    for i in 0..k {
        h[i] = (nums[i], i);
    }
    nums.push(0);
    h.sort_unstable();
    for i in k..=len {
        if k % 2 == 0 {
            ans.push((h[k / 2 - 1].0 as f64 + h[k / 2].0 as f64) / 2.0);
        } else {
            ans.push(h[k / 2].0 as f64);
        }
        let insert = h.binary_search(&(nums[i], i)).unwrap_or_else(|x| x);
        h.insert(insert, (nums[i], i));
        let pop = i - k;
        let pop_idx = h.binary_search(&(nums[pop], pop)).unwrap();
        h.remove(pop_idx);
    }
    ans
}

struct DualHeap {
    small: BinaryHeap<i32>,
    large: BinaryHeap<Reverse<i32>>,
    delayed: HashMap<i32, usize>,
    n: usize,
    small_size: i32,
    large_size: i32,
}

impl DualHeap {
    fn prune_small(&mut self) {
        while !self.small.is_empty() {
            if self.delayed.contains_key(self.small.peek().unwrap()) {
                let top = self.small.pop().unwrap();
                if let Entry::Occupied(mut x) = self.delayed.entry(top) {
                    *x.get_mut() -= 1;
                    if *x.get() <= 0 {
                        x.remove();
                    }
                }
            } else {
                return;
            }
        }
    }
    fn prune_large(&mut self) {
        while !self.large.is_empty() {
            if self.delayed.contains_key(&self.large.peek().unwrap().0) {
                let Reverse(top) = self.large.pop().unwrap();
                if let Entry::Occupied(mut x) = self.delayed.entry(top) {
                    *x.get_mut() -= 1;
                    if *x.get() <= 0 {
                        x.remove();
                    }
                }
            } else {
                return;
            }
        }
    }

    fn make_balance(&mut self) {
        if self.small_size > self.large_size + 1 {
            self.large.push(Reverse(self.small.pop().unwrap()));
            self.large_size += 1;
            self.small_size -= 1;
            self.prune_small();
        } else if self.small_size < self.large_size {
            self.small.push(self.large.pop().unwrap().0);
            self.small_size += 1;
            self.large_size -= 1;
            self.prune_large();
        }
    }

    pub fn with_capacity(n: usize) -> Self {
        DualHeap {
            small: BinaryHeap::with_capacity(n),
            large: BinaryHeap::with_capacity(n),
            delayed: HashMap::with_capacity(n),
            n,
            small_size: 0,
            large_size: 0,
        }
    }

    pub fn push(&mut self, ele: i32) {
        if self.small.is_empty() || &ele < self.small.peek().unwrap() {
            self.small.push(ele);
            self.small_size += 1;
        } else {
            self.large.push(Reverse(ele));
            self.large_size += 1;
        }
        self.make_balance();
    }

    pub fn pop(&mut self, ele: i32) {
        *self.delayed.entry(ele).or_insert(0) += 1;
        if &ele <= self.small.peek().unwrap() {
            self.small_size -= 1;
            if &ele == self.small.peek().unwrap() {
                self.prune_small();
            }
        } else {
            self.large_size -= 1;
            if ele == self.large.peek().unwrap().0 {
                self.prune_large();
            }
        }
        self.make_balance();
    }

    pub fn mid(&self) -> f64 {
        return if self.n & 1 == 0 {
            (*self.small.peek().unwrap() as f64 + self.large.peek().unwrap().0 as f64) / 2.0
        } else {
            *self.small.peek().unwrap() as f64
        };
    }
}

// 5ms
pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
    let len = nums.len();
    let k = k as usize;
    let mut dual = DualHeap::with_capacity(k);
    for i in 0..k {
        dual.push(nums[i]);
    }
    let mut ans = Vec::with_capacity(len);
    for i in k..=len {
        // println!("{}, {}, {:?}, {:?}, {:?}, {}, {}", i, dual.mid(), dual.small, dual.large, dual.delayed, dual.large_size, dual.small_size);
        ans.push(dual.mid());
        if i < len {
            dual.pop(nums[i - k]);
            dual.push(nums[i]);
        }
    }
    ans
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> Vec<f64>) {
        assert_eq!(func(vec![1, 3, -1, -3, 5, 3, 6, 7], 3), vec![1.00000, -1.00000, -1.00000, 3.00000, 5.00000, 6.00000]);
        assert_eq!(func(vec![1, 2, 3, 4, 2, 3, 1, 4, 2], 3), vec![2.00000, 3.00000, 3.00000, 3.00000, 2.00000, 3.00000, 2.00000]);
    }
    test(median_sliding_window);
    test(median_sliding_window_default);
}
