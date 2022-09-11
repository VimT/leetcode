//! 使数组中所有元素都等于零

pub fn minimum_operations(nums: Vec<i32>) -> i32 {
    let mut set = vec![0; 101];
    for num in nums {
        set[num as usize] += 1;
    }
    set[1..].iter().filter(|x| **x > 0).count() as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 5, 0, 3, 5]), 3);
        assert_eq!(func(vec![0]), 0);
    }
    test(minimum_operations);
}
