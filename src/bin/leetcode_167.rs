//! 两数之和 II - 输入有序数组

pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let len = numbers.len();
    let mut left = 0;
    let mut right = len;
    let mut result = vec![];
    while left < right {
        let sum = numbers[left] + numbers[right - 1];
        if sum == target {
            result = vec![(left + 1) as i32, right as i32];
            break;
        } else if sum > target {
            right -= 1;
        } else {
            left += 1;
        }
    }
    result
}

fn main() {
    fn test(func: fn(numbers: Vec<i32>, target: i32) -> Vec<i32>) {
        assert_eq!(func(vec![2, 7, 11, 15], 9), vec![1, 2]);
        assert_eq!(func(vec![2, 3, 4], 6), vec![1, 3]);
        assert_eq!(func(vec![-1, 0], -1), vec![1, 2]);
    }
    test(two_sum);
}
