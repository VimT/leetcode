//! 按奇偶排序数组 II

pub fn sort_array_by_parity_ii(mut nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let mut j = 1;
    for i in (0..len).step_by(2) {
        if nums[i] & 1 == 1 {
            while nums[j] & 1 == 1 {
                j += 2;
            }
            nums.swap(i, j);
        }
    }
    nums
}

fn main() {
    assert_eq!(sort_array_by_parity_ii(vec![4, 2, 5, 7]), vec![4, 5, 2, 7]);
    assert_eq!(sort_array_by_parity_ii(vec![2, 3]), vec![2, 3]);
}
