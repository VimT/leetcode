//! 寻找旋转排序数组中的最小值


pub fn find_min(nums: Vec<i32>) -> i32 {
    let mut start = 0;
    let mut end = nums.len() - 1;
    let left_min = nums[0];
    if *nums.last().unwrap() >= left_min {
        return left_min;
    }

    while start < end {
        let mid = start + (end - start) / 2;
        let num = nums[mid];
        if num >= left_min {
            start = mid + 1;
        } else {
            end = mid;
        }
    }
    nums[start]
}


fn main() {
    assert_eq!(find_min(vec![3, 4, 5, 1, 2]), 1);
    assert_eq!(find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
    assert_eq!(find_min(vec![11, 13, 15, 17]), 11);
}
