//! 使序列递增的最小交换次数

pub fn min_swap(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let len = nums1.len();
    let mut ch = 1;
    let mut nch = 0;
    for i in 1..len {
        let mut new_ch = i32::MAX;
        let mut new_nch = i32::MAX;
        if nums1[i] > nums1[i - 1] && nums2[i] > nums2[i - 1] {
            new_nch = new_nch.min(nch);
            new_ch = new_ch.min(ch + 1);
        }
        if nums1[i] > nums2[i - 1] && nums2[i] > nums1[i - 1] {
            new_nch = new_nch.min(ch);
            new_ch = new_ch.min(nch + 1);
        }
        ch = new_ch;
        nch = new_nch;
    }
    ch.min(nch)
}

fn main() {
    assert_eq!(min_swap(vec![1, 3, 5, 4], vec![1, 2, 3, 7]), 1);
    assert_eq!(min_swap(vec![0, 3, 5, 8, 9], vec![2, 1, 4, 6, 9]), 1);
}
