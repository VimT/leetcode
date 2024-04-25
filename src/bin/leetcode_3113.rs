//! 边界元素是最大值的子数组数目

use std::collections::HashMap;
use leetcode::segment_tree::{SegmentSpec, SegmentTree};

/// 线段树
pub fn number_of_subarrays(nums: Vec<i32>) -> i64 {
    let len = nums.len();
    let mut result = 0;
    let mut pos: HashMap<i32, Vec<usize>> = HashMap::new();
    struct Max {}
    impl SegmentSpec for Max {
        type ValType = i32;
        type LazyType = i32;
        fn op(&a: &Self::ValType, &b: &Self::ValType) -> Self::ValType { a.max(b) }
        fn identity() -> Self::ValType { i32::MIN }
        fn compose(&f: &Self::LazyType, g: &Self::LazyType) -> Self::LazyType { f.max(*g) }
        fn apply(&f: &Self::LazyType, a: &Self::ValType, _: i64) -> Self::ValType { f.max(*a) }
    }
    let mut st = SegmentTree::<Max>::new(&nums);
    for i in 0..len {
        pos.entry(nums[i]).or_default().push(i);
    }
    for (num, s) in pos {
        let mut i = 0;
        while i < s.len() {
            let start = i;
            while i < s.len() && st.query(s[start], s[i]) == num {
                result += i + 1 - start;
                i += 1;
            }
        }
    }
    result as i64
}

/// 单调栈。对于 [1,2,1] 来说，遍历到 2 时，1就不会与右边的1组成子数组了，所以可以直接跳过。
pub fn number_of_subarrays2(nums: Vec<i32>) -> i64 {
    let mut result = nums.len();
    let mut st = vec![(i32::MAX, 0)];  // (value, count)
    for x in nums {
        while x > st.last().unwrap().0 {
            st.pop().unwrap();
        }
        if x == st.last().unwrap().0 {
            result += st.last().unwrap().1;
            st.last_mut().unwrap().1 += 1;
        } else {
            st.push((x, 1));
        }
    }

    result as i64
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i64) {
        assert_eq!(func(vec![1, 4, 3, 2, 3, 2]), 7);
        assert_eq!(func(vec![1, 4, 3, 3, 2]), 6);
        assert_eq!(func(vec![3, 3, 3]), 6);
        assert_eq!(func(vec![1]), 1);
    }
    test(number_of_subarrays);
    test(number_of_subarrays2);
}
