//! 让所有学生保持开心的分组方法数

pub fn count_ways(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let len = nums.len();
    let mut result = (nums[0] > 0) as i32;
    for i in 1..=len {
        if (nums[i - 1] as usize) < i && (i == len || nums[i] as usize > i) {
            result += 1;
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1,1,0,1]), 1);
        assert_eq!(func(vec![1, 1]), 2);
        assert_eq!(func(vec![6, 0, 3, 3, 6, 7, 2, 7]), 3);
    }
    test(count_ways);
}
