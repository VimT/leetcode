//! 数组中的峰值


use leetcode::bit_index_tree::{BITree, BitVal};

/// 树状数组
pub fn count_of_peaks(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    enum Spec {}
    impl BitVal for Spec {
        type ValType = i32;
        fn identity() -> Self::ValType { 0 }
        fn op(f: &Self::ValType, g: &Self::ValType) -> Self::ValType { f + g }
    }
    let len = nums.len();
    let mut tree = BITree::<Spec>::new(len);
    for i in 1..len - 1 {
        if nums[i] > nums[i - 1] && nums[i] > nums[i + 1] {
            tree.update(i, &1);
        }
    }

    queries.iter().filter_map(|q| {
        match q[0] {
            1 => {
                let (l, r) = (q[1] as usize + 1, q[2] as usize - 1);
                if l <= r { Some(tree.query_prefix(r + 1) - tree.query_prefix(l)) } else { Some(0) }
            }
            2 => {
                let (i, val) = (q[1] as usize, q[2]);
                for i in i.saturating_sub(1).max(1)..(i + 2).min(len - 1) {
                    if nums[i] > nums[i - 1] && nums[i] > nums[i + 1] {
                        tree.update(i, &-1);
                    }
                }
                nums[i] = val;
                for i in i.saturating_sub(1).max(1)..(i + 2).min(len - 1) {
                    if nums[i] > nums[i - 1] && nums[i] > nums[i + 1] {
                        tree.update(i, &1);
                    }
                }
                None
            }
            _ => unreachable!()
        }
    }).collect()
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32>) {
        assert_eq!(func(vec![7, 10, 7], vec![vec![1, 2, 2], vec![2, 0, 6], vec![1, 0, 2]]), vec![0, 1]);
        assert_eq!(func(vec![3, 1, 4, 2, 5], vec![vec![2, 3, 4], vec![1, 0, 4]]), vec![0]);
        assert_eq!(func(vec![4, 1, 4, 2, 1, 5], vec![vec![2, 2, 4], vec![1, 0, 2], vec![1, 0, 4]]), vec![0, 1]);
    }
    test(count_of_peaks);
}
