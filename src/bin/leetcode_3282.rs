//! 到达数组末尾的最大得分

pub fn find_maximum_score(nums: Vec<i32>) -> i64 {
    let len = nums.len();
    let mut mx = nums[0];
    let mut result = 0;
    for i in 1..len {
        result += mx as i64;
        mx = mx.max(nums[i]);
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i64) {
        assert_eq!(func(vec![1, 2, 3, 4, 5, 6, 7, 8]), 28);
        assert_eq!(func(vec![1, 3, 1, 5]), 7);
        assert_eq!(func(vec![4, 3, 1, 3, 2]), 16);
    }
    test(find_maximum_score);
}
