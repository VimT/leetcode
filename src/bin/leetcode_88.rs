//! 合并两个有序数组

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let m = m as usize;
    let n = n as usize;
    let len = m + n;
    let mut p1 = m;
    let mut p2 = n;
    let mut p = len;
    while p > 0 && p1 > 0 && p2 > 0 {
        if nums1[p1 - 1] > nums2[p2 - 1] {
            nums1[p - 1] = nums1[p1 - 1];
            p1 -= 1;
        } else {
            nums1[p - 1] = nums2[p2 - 1];
            p2 -= 1;
        }
        p -= 1;
    }
    while p1 > 0 {
        nums1[p - 1] = nums1[p1 - 1];
        p -= 1;
        p1 -= 1;
    }
    while p2 > 0 {
        nums1[p - 1] = nums2[p2 - 1];
        p -= 1;
        p2 -= 1;
    }
}

fn main() {
    let mut nums1 = vec![0];
    let mut nums2 = vec![1];
    merge(&mut nums1, 0, &mut nums2, 1);
    assert_eq!(nums1, [1]);
}
