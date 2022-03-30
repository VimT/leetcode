//! 绝对差不超过限制的最长连续子数组


use std::collections::{BTreeSet, VecDeque};

/// 需要得到区间的最小值和最大值，可以想到 线段树或者平衡树, 单调队列
pub fn longest_subarray_btree(nums: Vec<i32>, limit: i32) -> i32 {
    let mut ans = 0;
    let mut left = 0;
    let mut right = 0;
    let len = nums.len();
    let mut tree = BTreeSet::new();
    while right < len {
        tree.insert((nums[right], right));
        while tree.range(..).last().unwrap().0 - tree.range(..).next().unwrap().0 > limit {
            tree.remove(&(nums[left], left));
            left += 1;
        }
        ans = ans.max(right + 1 - left);
        right += 1;
    }
    ans as i32
}

pub fn longest_subarray_avl(nums: Vec<i32>, limit: i32) -> i32 {
    use leetcode::avl::AvlTreeSet;
    let mut ans = 0;
    let mut left = 0;
    let mut right = 0;
    let len = nums.len();
    let mut tree = AvlTreeSet::new();
    while right < len {
        tree.insert((nums[right], right));
        while tree.last().unwrap().0 - tree.first().unwrap().0 > limit {
            tree.remove(&(nums[left], left));
            left += 1;
        }
        ans = ans.max(right + 1 - left);
        right += 1;
    }
    ans as i32
}

/// best
pub fn longest_subarray_monotone_queue(nums: Vec<i32>, limit: i32) -> i32 {
    let mut ans = 0;
    let mut left = 0;
    let mut right = 0;
    let len = nums.len();
    let mut less_queue = VecDeque::new();
    let mut more_queue = VecDeque::new();
    while right < len {
        while !less_queue.is_empty() && nums[right] > *less_queue.back().unwrap() {
            less_queue.pop_back();
        }
        less_queue.push_back(nums[right]);
        while !more_queue.is_empty() && nums[right] < *more_queue.back().unwrap() {
            more_queue.pop_back();
        }
        more_queue.push_back(nums[right]);
        while !less_queue.is_empty() && !more_queue.is_empty() && less_queue.front().unwrap() - more_queue.front().unwrap() > limit {
            if nums[left] == *less_queue.front().unwrap() {
                less_queue.pop_front().unwrap();
            }
            if nums[left] == *more_queue.front().unwrap() {
                more_queue.pop_front().unwrap();
            }
            left += 1;
        }
        ans = ans.max(right - left + 1);
        right += 1;
    }
    ans as i32
}


#[derive(Debug, Clone)]
struct SegmentTree<'a, T> {
    min: Vec<T>,
    max: Vec<T>,
    a: &'a Vec<T>,
}

impl<'a> SegmentTree<'a, i32> {
    pub fn new(arr: &'a Vec<i32>) -> Self {
        let mut result = SegmentTree {
            max: vec![0; 4 * arr.len()],
            min: vec![0; 4 * arr.len()],
            a: arr,
        };
        result.build(0, arr.len() - 1, 0);
        result
    }

    fn build(&mut self, s: usize, t: usize, p: usize) {
        if s == t {
            self.min[p] = self.a[s];
            self.max[p] = self.a[s];
            return;
        }
        let m = (s + t) / 2;
        self.build(s, m, p * 2 + 1);
        self.build(m + 1, t, p * 2 + 2);
        self.min[p] = self.min[p * 2 + 1].min(self.min[(p * 2) + 2]);
        self.max[p] = self.max[p * 2 + 1].max(self.max[(p * 2) + 2]);
        // println!("{}-{} p is {}, min is {}, max is {}", s, t,p, self.min[p], self.max[p]);
    }

    /// [l, r]: query range, [s,t]: current node range, p: current node idx
    pub fn ask(&self, l: usize, r: usize, s: usize, t: usize, p: usize) -> (i32, i32) {
        if l <= s && r >= t {
            return (self.min[p], self.max[p]);
        }
        let m = (s + t) / 2;
        let mut min = i32::MAX;
        let mut max = i32::MIN;

        if l <= m {
            let (m1, m2) = self.ask(l, r, s, m, p * 2 + 1);
            min = m1.min(min);
            max = m2.max(max);
        }
        if r > m {
            let (m1, m2) = self.ask(l, r, m + 1, t, p * 2 + 2);
            min = m1.min(min);
            max = m2.max(max);
        }
        // println!("query: {}-{} p is {}, min is {}, max is {}", s, t, p, min, max);
        (min, max)
    }
}

pub fn longest_subarray_segment_tree(nums: Vec<i32>, limit: i32) -> i32 {
    let mut ans = 0;
    let mut left = 0;
    let mut right = 0;
    let len = nums.len();
    let tree = SegmentTree::new(&nums);
    while right < len {
        while left < right {
            let (min, max) = tree.ask(left, right, 0, len - 1, 0);
            if max - min <= limit {
                break;
            }
            left += 1;
        }
        ans = ans.max(right + 1 - left);
        right += 1;
    }
    ans as i32
}

struct SparseTable {
    min: Vec<Vec<i32>>,
    max: Vec<Vec<i32>>,
}

impl SparseTable {
    fn new(array: Vec<i32>) -> Self {
        let len = array.len();
        let m = (len as f64).log2() as usize;
        let mut min = vec![vec![0; m + 1]; len + 2];
        let mut max = vec![vec![0; m + 1]; len + 2];
        for i in 1..=len {
            min[i][0] = array[i - 1];
            max[i][0] = array[i - 1];
        }
        for i in 1..=m {
            for j in 1..=len + 1 - (1 << i) {
                min[j][i] = min[j][i - 1].min(min[j + (1 << (i - 1))][i - 1]);
                max[j][i] = max[j][i - 1].max(max[j + (1 << (i - 1))][i - 1]);
            }
        }
        SparseTable { min, max }
    }

    fn ask(&self, l: usize, r: usize) -> (i32, i32) {
        let k = ((r - l + 1) as f64).log2() as usize;
        return (self.min[l][k].min(self.min[r + 1 - (1 << k)][k]), self.max[l][k].max(self.max[r + 1 - (1 << k)][k]));
    }
}

/// sparse table, find minimum and maximum in O(1)
pub fn longest_subarray_binary_search(nums: Vec<i32>, limit: i32) -> i32 {
    let len = nums.len();
    let mut left = 1;
    let mut right = len;
    let st = SparseTable::new(nums);
    let mut ans = 0;
    while left <= right {
        let mid = left + (right - left) / 2;
        let mut ok = false;
        for i in mid..=len {
            let (min, max) = st.ask(i - mid + 1, i);
            if max - min <= limit {
                ok = true;
                ans = mid;
                break;
            }
        }
        if ok {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    ans as i32
}


pub fn longest_subarray_st_range(nums: Vec<i32>, limit: i32) -> i32 {
    let mut ans = 0;
    let mut left = 0;
    let mut right = 0;
    let len = nums.len();
    let st = SparseTable::new(nums);
    while right < len {
        while left < right {
            let (min, max) = st.ask(left + 1, right + 1);
            if max - min <= limit {
                break;
            }
            left += 1;
        }
        ans = ans.max(right + 1 - left);
        right += 1;
    }
    ans as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, limit: i32) -> i32) {
        assert_eq!(func(vec![8, 2, 4, 7], 4), 2);
        assert_eq!(func(vec![10, 1, 2, 4, 7, 2], 5), 4);
        assert_eq!(func(vec![4, 2, 2, 2, 4, 4, 2, 2], 0), 3);
    }
    test(longest_subarray_btree);
    test(longest_subarray_avl);
    test(longest_subarray_monotone_queue);
    test(longest_subarray_segment_tree);
    test(longest_subarray_binary_search);
    test(longest_subarray_st_range);
}
