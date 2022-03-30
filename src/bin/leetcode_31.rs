//! 下一个排列

/// 找顺序，从右向左，找 a[i] < a[i+1]，再从右向左，找a[i] < a[j] ，交换i,j。
pub fn next_permutation(nums: &mut Vec<i32>) {
    let mut i = nums.len();
    if i <= 1 { return; }
    for scan in (0..nums.len() - 1).rev() {
        if nums[scan] < nums[scan + 1] {
            i = scan;
            break;
        }
    }
    if i < nums.len() {
        for j in (0..nums.len()).rev() {
            if nums[i] < nums[j] {
                nums.swap(i, j);
                break;
            }
        }
    }
    if i == nums.len() {
        nums.reverse();
    } else {
        nums[i + 1..].reverse();
    }
}

fn main() {
    fn test(mut nums: Vec<i32>) -> Vec<i32> {
        next_permutation(&mut nums);
        nums
    }
    assert_eq!(test(vec![1, 5, 1]), [5, 1, 1]);
    assert_eq!(test(vec![4, 3, 2, 1]), [1, 2, 3, 4]);
    assert_eq!(test(vec![1, 2, 3, 4]), [1, 2, 4, 3]);
    assert_eq!(test(vec![1, 4, 3, 2]), [2, 1, 3, 4]);
    assert_eq!(test(vec![3, 2, 4, 1]), [3, 4, 1, 2]);
    assert_eq!(test(vec![5, 1, 2, 3, 4]), [5, 1, 2, 4, 3]);
    assert_eq!(test(vec![]), []);
    assert_eq!(test(vec![1]), [1]);
}
