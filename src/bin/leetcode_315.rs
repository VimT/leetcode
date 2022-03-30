//! 计算右侧小于当前元素的个数


use leetcode::algorithm;

/// 排序数组+二分查找
pub fn count_smaller_sort_bin_search(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut ans = vec![0; n];
    let mut h = vec![];

    for i in (0..n).rev() {
        let num = nums[i];
        let pos = h.binary_search(&num).unwrap_or_else(|x| x);
        ans[i] = pos as i32;
        h.insert(pos, num);
    }
    ans
}

/// 归并排序 在归并排序的过程中利用数组间已经有的大小关系
pub fn count_smaller_merge_sort(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![0; nums.len()];

    fn inner(mut nums: Vec<(i32, usize)>, ans: &mut Vec<i32>) -> Vec<(i32, usize)> {
        if nums.len() == 1 {
            return nums;
        }
        let mid = nums.len() / 2;
        let left = inner(nums[0..mid].to_vec(), ans);
        let right = inner(nums[mid..].to_vec(), ans);

        let (mut p1, mut p2, mut p) = (left.len(), right.len(), nums.len());
        while p1 > 0 && p2 > 0 {
            if left[p1 - 1].0 > right[p2 - 1].0 {
                ans[left[p1 - 1].1] += 1;
                nums[p - 1] = left[p1 - 1];
                p1 -= 1;
                p -= 1;
            } else {
                nums[p - 1] = right[p2 - 1];
                p2 -= 1;
                p -= 1;
            }
        }
        while p1 > 0 {
            nums[p - 1] = left[p1 - 1];
            p1 -= 1;
            p -= 1;
        }
        while p2 > 0 {
            nums[p - 1] = right[p2 - 1];
            p2 -= 1;
            p -= 1;
        }
        nums
    }

    inner(nums.into_iter().enumerate().map(|(i, j)| (j, i)).collect(), &mut ans);
    ans
}

/// 树状数组
pub fn count_smaller_bin_indexed_tree(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut t = algorithm::BinIndexedTree::with_capacity(nums.len() + 1);
    let mut ans = vec![0; nums.len()];
    let mut h: Vec<(i32, usize)> = nums.into_iter().enumerate().map(|(i, v)| (v, i + 1)).collect();
    h.sort();

    for (_, i) in h {
        t.add(i, 1);
        ans[i - 1] = t.sum(n) - t.sum(i);
    }
    ans
}


fn main() {
    fn test(func: fn(nums: Vec<i32>) -> Vec<i32>) {
        assert_eq!(func(vec![5, 2, 6, 1]), vec![2, 1, 1, 0]);
        assert_eq!(func(vec![-1]), vec![0]);
        assert_eq!(func(vec![-1, -1]), vec![0, 0]);
    }
    test(count_smaller_bin_indexed_tree);
    test(count_smaller_merge_sort);
    test(count_smaller_sort_bin_search);
}
