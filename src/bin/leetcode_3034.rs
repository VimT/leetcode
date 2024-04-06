//! 匹配模式数组的子数组数目 I

pub fn count_matching_subarrays(nums: Vec<i32>, pattern: Vec<i32>) -> i32 {
    let len = nums.len();
    let m = pattern.len();
    let mut result = 0;
    for start in 1..len + 1 - m {
        let mut ok = true;
        for i in start..start + m {
            ok = match pattern[i - start] {
                -1 => nums[i] < nums[i - 1],
                0 => nums[i] == nums[i - 1],
                1 => nums[i] > nums[i - 1],
                _ => unreachable!(),
            };
            if !ok { break; }
        }
        if ok { result += 1; }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, pattern: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 2, 3, 4, 5, 6], vec![1, 1]), 4);
        assert_eq!(func(vec![1, 4, 4, 1, 3, 5, 5, 3], vec![1, 0, -1]), 2);
    }
    test(count_matching_subarrays);
}
