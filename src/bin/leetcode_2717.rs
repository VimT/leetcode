//! 半有序排列

pub fn semi_ordered_permutation(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let one = (0..len).find(|&x| nums[x] == 1).unwrap();
    let last = (0..len).rev().find(|&x| nums[x] == len as i32).unwrap();
    if one < last {
        (one + len - 1 - last) as i32
    } else {
        (one + len - 2 - last) as i32
    }
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![3, 2, 1]), 3);
        assert_eq!(func(vec![2, 1, 4, 3]), 2);
        assert_eq!(func(vec![2, 4, 1, 3]), 3);
        assert_eq!(func(vec![1, 3, 4, 2, 5]), 0);
    }
    test(semi_ordered_permutation);
}
