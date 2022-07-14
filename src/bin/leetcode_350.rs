//! 两个数组的交集 II

pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut nums = nums2.clone();
    nums.sort();

    let mut ans = vec![];

    for i in nums1 {
        let find = nums.binary_search(&i);
        if find.is_ok() {
            ans.push(nums[find.unwrap()]);
            nums.remove(find.unwrap());
        }
    }
    ans
}


fn main() {
    assert_eq!(intersect(vec![1, 2, 2, 1], vec![2, 2]), vec![2, 2]);
    assert_eq!(intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]), vec![4, 9]);
}
