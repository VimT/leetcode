//! 最长递增子序列 II


struct SegmentTree {
    mx: Vec<i32>,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        SegmentTree { mx: vec![0; n * 4] }
    }

    fn modify(&mut self, s: usize, t: usize, p: usize, i: usize, val: i32) {
        if s == t {
            self.mx[p] = val;
            return;
        }
        let mid = (s + t) / 2;
        if i <= mid {
            self.modify(s, mid, p * 2, i, val);
        } else {
            self.modify(mid + 1, t, p * 2 + 1, i, val);
        }
        self.mx[p] = self.mx[p * 2].max(self.mx[p * 2 + 1])
    }

    fn query(&self, l: usize, r: usize, s: usize, t: usize, p: usize) -> i32 {
        if l <= s && t <= r { return self.mx[p]; }
        let mut result = 0;
        let mid = (s + t) / 2;
        if l <= mid {
            result = self.query(l, r, s, mid, p * 2);
        }
        if mid < r {
            result = result.max(self.query(l, r, mid + 1, t, p * 2 + 1));
        }
        result
    }
}

/// LIS(300)问题有两种解法：
/// 1. 单调栈+二分优化
/// 2. 线段树
/// 本题有一个差值不超过 k 的约束，用线段树更好处理。
/// 线段树叶节点 f[i] 表示以i结尾的满足题目两个条件的子序列的最长长度
pub fn length_of_lis(nums: Vec<i32>, k: i32) -> i32 {
    let max = *nums.iter().max().unwrap() as usize;
    let mut tree = SegmentTree::new(max);
    for num in nums {
        if num == 1 {
            tree.modify(1, max, 1, 1, 1);
        } else {
            let r = 1 + tree.query((num - k).max(1) as usize, (num - 1) as usize, 1, max, 1);
            tree.modify(1, max, 1, num as usize, r);
        }
    }
    tree.mx[1]
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![4, 2, 1, 4, 3, 4, 5, 8, 15], 3), 5);
        assert_eq!(func(vec![7, 4, 5, 1, 8, 12, 4, 7], 5), 4);
        assert_eq!(func(vec![1, 5], 1), 1);
    }
    test(length_of_lis);
}
