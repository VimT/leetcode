//! 边界上的蚂蚁

pub fn return_to_boundary_count(nums: Vec<i32>) -> i32 {
    let mut cur = 0;
    let mut result = 0;
    for num in nums {
        cur += num;
        if cur == 0 { result += 1; }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![2, 3, -5]), 1);
        assert_eq!(func(vec![3, 2, -3, -4]), 0);
    }
    test(return_to_boundary_count);
}
