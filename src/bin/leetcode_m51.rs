//! 数组中的逆序对
use leetcode::algorithm;

pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
    if nums.len() <= 1 {
        return 0;
    }
    fn inner(mut nums: Vec<i32>) -> (Vec<i32>, i32) {
        if nums.len() == 1 {
            return (nums, 0);
        }

        let mut count = 0;
        let mid = nums.len() / 2;
        let (nums1, left) = inner(nums[0..mid].to_vec());
        let (nums2, right) = inner(nums[mid..nums.len()].to_vec());
        let mut p1: i32 = (nums1.len() - 1) as i32;
        let mut p2: i32 = (nums2.len() - 1) as i32;
        let mut p: i32 = (nums.len() - 1) as i32;
        while p1 >= 0 && p2 >= 0 {
            if nums1[p1 as usize] > nums2[p2 as usize] {
                count += p2 + 1;
                nums[p as usize] = nums1[p1 as usize];
                p1 -= 1;
                p -= 1;
            } else {
                nums[p as usize] = nums2[p2 as usize];
                p2 -= 1;
                p -= 1;
            }
        }
        while p1 >= 0 {
            nums[p as usize] = nums1[p1 as usize];
            p1 -= 1;
            p -= 1;
        }
        while p2 >= 0 {
            nums[p as usize] = nums2[p2 as usize];
            p2 -= 1;
            p -= 1;
        }

        (nums, count + left + right)
    }

    return inner(nums).1;
}

pub fn reverse_pairs_bin_indexed_tree(nums: Vec<i32>) -> i32 {
    let mut t = algorithm::BinIndexedTree::with_capacity(nums.len() + 1);
    let mut h: Vec<(i32, usize)> = nums.into_iter().enumerate().map(|(i, v)| (v, i + 1)).collect();
    h.sort();
    let mut ans = 0;

    for (_, i) in h.into_iter().rev() {
        t.add(i, 1);
        ans += t.sum(i - 1);
    }

    ans
}

fn main() {
    println!("{}", reverse_pairs(vec![4, 2, 3, 1]));
    println!("{}", reverse_pairs_bin_indexed_tree(vec![4, 2, 3, 1]));
}
