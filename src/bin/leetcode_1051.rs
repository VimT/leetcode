//! 高度检查器

pub fn height_checker(heights: Vec<i32>) -> i32 {
    let mut h2 = heights.clone();
    h2.sort_unstable();
    let mut result = 0;
    for (a, b) in heights.into_iter().zip(h2) {
        if a != b { result += 1; }
    }
    result
}

/// 计数排序
pub fn height_checker_count(heights: Vec<i32>) -> i32 {
    let mut count = vec![0; 101];
    for &height in &heights {
        count[height as usize] += 1;
    }
    let mut result = 0;
    let mut j = 0;
    for i in 1..=100 {
        for _ in 0..count[i] {
            if heights[j] != i as i32 {
                result += 1;
            }
            j += 1;
        }
    }
    result
}

fn main() {
    fn test(func: fn(heights: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 1, 4, 2, 1, 3]), 3);
        assert_eq!(func(vec![5, 1, 2, 3, 4]), 5);
        assert_eq!(func(vec![1, 2, 3, 4, 5]), 0);
    }
    test(height_checker);
    test(height_checker_count);
}
