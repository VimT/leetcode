//! 盛最多水的容器

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = height.len() - 1;
    let mut cmax = 0;
    while left < right {
        let small = height[left].min(height[right]);
        let sum = small * (right - left) as i32;
        if sum > cmax {
            cmax = sum;
        }
        if small == height[left] { left += 1; } else { right -= 1; }
    }
    cmax
}

fn main() {
    fn test(func: fn(height: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(func(vec![1, 1]), 1);
    }
    test(max_area);
}
