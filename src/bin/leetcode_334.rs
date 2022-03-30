//! 递增的三元子序列

pub fn increasing_triplet(nums: Vec<i32>) -> bool {
    if nums.len() < 3 { return false; }
    let mut n1 = i32::MAX;
    let mut n2 = i32::MAX;
    for i in nums {
        if i > n2 {
            return true;
        } else if i > n1 && i < n2 {
            n2 = i;
        } else if i < n1 {
            n1 = i;
        }
    }
    false
}

fn main() {
    assert_eq!(increasing_triplet(vec![1, 2, 3, 4, 5]), true);
    assert_eq!(increasing_triplet(vec![5, 4, 3, 2, 1]), false);
    assert_eq!(increasing_triplet(vec![2, 1, 5, 0, 4, 6]), true);
}
