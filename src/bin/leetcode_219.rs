//! 存在重复元素 II

pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut ni = nums.iter().enumerate().map(|(i, x)| (*x, i)).collect::<Vec<(i32, usize)>>();
    ni.sort_unstable();
    let k = k as usize;
    for i in 1..nums.len() {
        if ni[i].0 == ni[i - 1].0 && ni[i].1 - ni[i - 1].1 <= k {
            return true;
        }
    }
    false
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> bool) {
        assert_eq!(func(vec![1, 2, 3, 1], 3), true);
        assert_eq!(func(vec![1, 0, 1, 1], 1), true);
        assert_eq!(func(vec![1, 2, 3, 1, 2, 3], 2), false);
    }
    test(contains_nearby_duplicate);
}
