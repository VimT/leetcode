//! 将数组分成最小总代价的子数组 II


use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::collections::hash_map::Entry;
use leetcode::multi_set::{BtreeMultiSet, TreapMultiSet};

// 两个有序map维护滑动窗口的前n个最小值
pub fn minimum_cost(nums: Vec<i32>, k: i32, dist: i32) -> i64 {
    struct DualMulti {
        k: usize,
        // 前k个最小元素和
        sum: i64,
        st1: BtreeMultiSet<i64>,
        st2: BtreeMultiSet<i64>,
    }
    impl DualMulti {
        fn new(k: usize) -> Self {
            Self { k, sum: 0, st1: BtreeMultiSet::new(), st2: BtreeMultiSet::new() }
        }

        fn adjust(&mut self) {
            while self.st1.len() < self.k && self.st2.len() > 0 {
                let x = self.st2.pop_first().unwrap();
                self.st1.insert(x);
                self.sum += x;
            }
            while self.st1.len() > self.k {
                let x = self.st1.pop_last().unwrap();
                self.st2.insert(x);
                self.sum -= x;
            }
        }
        fn add(&mut self, x: i64) {
            if self.st2.len() > 0 && x >= self.st2.first().copied().unwrap() {
                self.st2.insert(x);
            } else {
                self.st1.insert(x);
                self.sum += x;
            }
            self.adjust();
        }
        fn remove(&mut self, x: i64) {
            if self.st1.remove(x) {
                self.sum -= x;
            } else {
                self.st2.remove(x);
            }
            self.adjust();
        }
    }

    let len = nums.len();
    let dist = dist as usize;
    let mut dm = DualMulti::new(k as usize - 1);
    for i in 1..=dist + 1 {
        dm.add(nums[i] as i64);
    }
    let mut result = dm.sum;
    for i in dist + 2..len {
        dm.add(nums[i] as i64);
        dm.remove(nums[i - dist - 1] as i64);
        result = result.min(dm.sum);
    }
    result + nums[0] as i64
}

/// 类似480的做法，双堆+延迟删除。
/// 堆50ms，multiset130ms
pub fn minimum_cost2(nums: Vec<i32>, k: i32, dist: i32) -> i64 {
    #[derive(Debug)]
    struct DualHeap {
        // 大顶堆，存最小的k的元素
        h1: BinaryHeap<i32>,
        // 小顶堆，存大于k的元素
        h2: BinaryHeap<Reverse<i32>>,
        removed: HashMap<i32, usize>,
        k: usize,
        sum: i64,
        h1_size: usize,
        h2_size: usize,
    }

    impl DualHeap {
        pub fn with_capacity(k: usize, n: usize) -> Self {
            DualHeap { h1: BinaryHeap::with_capacity(n), h2: BinaryHeap::with_capacity(n), removed: HashMap::with_capacity(n), k, sum: 0, h1_size: 0, h2_size: 0 }
        }
        fn map_remove(&mut self, x: i32) {
            if let Entry::Occupied(mut x) = self.removed.entry(x) {
                *x.get_mut() -= 1;
                if *x.get() <= 0 { x.remove(); }
            }
        }
        fn purge_h1(&mut self) {
            while !self.h1.is_empty() && self.removed.contains_key(self.h1.peek().unwrap()) {
                let top = self.h1.pop().unwrap();
                self.map_remove(top);
            }
        }
        fn purge_h2(&mut self) {
            while !self.h2.is_empty() && self.removed.contains_key(&self.h2.peek().unwrap().0) {
                let Reverse(top) = self.h2.pop().unwrap();
                self.map_remove(top);
            }
        }
        pub fn adjust(&mut self) {
            while self.h1_size < self.k && self.h2_size > 0 {
                let Reverse(x) = self.h2.pop().unwrap();
                self.h1.push(x);
                self.h2_size -= 1;
                self.h1_size += 1;
                self.sum += x as i64;
                self.purge_h2();
            }
            while self.h1_size > self.k {
                let x = self.h1.pop().unwrap();
                self.h2.push(Reverse(x));
                self.h1_size -= 1;
                self.h2_size += 1;
                self.sum -= x as i64;
                self.purge_h1();
            }
        }

        pub fn add(&mut self, x: i32) {
            if self.h2_size != 0 && x >= self.h2.peek().unwrap().0 {
                self.h2.push(Reverse(x));
                self.h2_size += 1;
            } else {
                self.h1.push(x);
                self.h1_size += 1;
                self.sum += x as i64;
            }
            self.adjust();
        }

        pub fn remove(&mut self, x: i32) {
            *self.removed.entry(x).or_default() += 1;
            if x <= *self.h1.peek().unwrap() {
                self.h1_size -= 1;
                self.sum -= x as i64;
                self.purge_h1();
            } else {
                self.h2_size -= 1;
                self.purge_h2();
            }
            self.adjust();
        }
    }

    let len = nums.len();
    let dist = dist as usize;
    let mut dh = DualHeap::with_capacity(k as usize - 1, len);
    for i in 1..=dist + 1 {
        dh.add(nums[i]);
    }
    let mut result = dh.sum;
    for i in dist + 2..len {
        dh.add(nums[i]);
        dh.remove(nums[i - dist - 1]);
        result = result.min(dh.sum);
    }
    result + nums[0] as i64
}

// 两个有序map维护滑动窗口的前n个最小值
pub fn minimum_cost3(nums: Vec<i32>, k: i32, dist: i32) -> i64 {
    #[derive(Debug)]
    struct DualMulti {
        k: usize,
        // 前k个最小元素和
        sum: i64,
        st1: TreapMultiSet<i64>,
        st2: TreapMultiSet<i64>,
    }
    impl DualMulti {
        fn new(k: usize) -> Self {
            Self { k, sum: 0, st1: TreapMultiSet::new(), st2: TreapMultiSet::new() }
        }

        fn adjust(&mut self) {
            while self.st1.len() < self.k && self.st2.len() > 0 {
                let x = self.st2.pop_first().unwrap();
                self.st1.insert(x);
                self.sum += x;
            }
            while self.st1.len() > self.k {
                let x = self.st1.pop_last().unwrap();
                self.st2.insert(x);
                self.sum -= x;
            }
        }
        fn add(&mut self, x: i64) {
            if self.st2.len() != 0 && x >= self.st2.first().copied().unwrap() {
                self.st2.insert(x);
            } else {
                self.st1.insert(x);
                self.sum += x;
            }
            self.adjust();
        }
        fn remove(&mut self, x: i64) {
            if self.st1.remove(&x) {
                self.sum -= x;
            } else {
                self.st2.remove(&x);
            }
            self.adjust();
        }
    }

    let len = nums.len();
    let dist = dist as usize;
    let mut dm = DualMulti::new(k as usize - 1);
    for i in 1..=dist + 1 {
        dm.add(nums[i] as i64);
    }
    let mut result = dm.sum;
    for i in dist + 2..len {
        dm.add(nums[i] as i64);
        dm.remove(nums[i - dist - 1] as i64);
        result = result.min(dm.sum);
    }
    result + nums[0] as i64
}


fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32, dist: i32) -> i64) {
        assert_eq!(func(vec![10, 1, 2, 2, 2, 1], 4, 3), 15);
        assert_eq!(func(vec![67054, 312975, 261918, 147468, 351792, 160481, 40686, 229301, 404409, 155799, 88433, 228396, 212142, 2047, 350482, 163356, 152688, 494449, 248950, 109811, 315894, 377855, 316737, 273181, 161892, 168702, 135940, 16503, 298677, 55131, 126026, 247455, 187384, 291920, 49503], 5, 8), 314217);
        assert_eq!(func(vec![2, 5, 3, 5, 7, 4, 3], 3, 3), 9);
        assert_eq!(func(vec![1, 6, 5, 7, 8, 7, 5], 5, 4), 25);
        assert_eq!(func(vec![1, 6, 3, 5], 3, 2), 9);
        assert_eq!(func(vec![1, 3, 2, 6, 4, 2], 3, 3), 5);
        assert_eq!(func(vec![1, 5, 3, 7], 3, 1), 9);
        assert_eq!(func(vec![10, 8, 18, 9], 3, 1), 36);
    }
    test(minimum_cost);
    test(minimum_cost2);
    test(minimum_cost3);
}
