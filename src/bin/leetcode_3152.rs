//! 特殊数组 II

pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let s: Vec<i32> = (0..=0).chain(nums.windows(2).map(|w| (w[0] % 2 == w[1] % 2) as i32)).scan(0, |sum, x| {
        *sum += x;
        Some(*sum)
    }).collect();
    queries.into_iter().map(|q| {
        let (from, to) = (q[0] as usize, q[1] as usize);
        s[from] - s[to] == 0
    }).collect()
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool>) {
        assert_eq!(func(vec![1], vec![vec![0, 0]]), vec![true]);
        assert_eq!(func(vec![3, 4, 1, 2, 6], vec![vec![0, 4]]), vec![false]);
        assert_eq!(func(vec![4, 3, 1, 6], vec![vec![0, 2], vec![2, 3]]), vec![false, true]);
    }
    test(is_array_special);
}
