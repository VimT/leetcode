//! 找到稳定山的下标

pub fn stable_mountains(height: Vec<i32>, threshold: i32) -> Vec<i32> {
    let len = height.len();
    let mut result = vec![];
    for i in 1..len {
        if height[i - 1] > threshold {
            result.push(i as i32);
        }
    }
    result
}

fn main() {
    fn test(func: fn(height: Vec<i32>, threshold: i32) -> Vec<i32>) {
        assert_eq!(func(vec![1, 2, 3, 4, 5], 2), vec![3, 4]);
        assert_eq!(func(vec![10, 1, 10, 1, 10], 3), vec![1, 3]);
        assert_eq!(func(vec![10, 1, 10, 1, 10], 10), vec![]);
    }
    test(stable_mountains);
}
