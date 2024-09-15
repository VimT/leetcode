//! 数字小镇中的捣蛋鬼

pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut cnt = vec![0; nums.len()];
    for num in nums {
        cnt[num as usize] += 1;
    }
    let mut result = vec![];
    for (i, &c) in cnt.iter().enumerate() {
        if c > 1 {
            result.push(i as i32);
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> Vec<i32>) {
        assert_eq!(func(vec![0, 1, 1, 0]), vec![0, 1]);
        assert_eq!(func(vec![0, 3, 2, 1, 3, 2]), vec![2, 3]);
        assert_eq!(func(vec![7, 1, 5, 4, 3, 4, 6, 0, 9, 5, 8, 2]), vec![4, 5]);
    }
    test(get_sneaky_numbers);
}
