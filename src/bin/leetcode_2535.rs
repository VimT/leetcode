//! 数组元素和与数字和的绝对差

pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
    let mut a = 0;
    let mut b = 0;
    for mut num in nums {
        a += num;
        while num > 0 {
            b += num % 10;
            num /= 10;
        }
    }
    (a - b).abs()
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 15, 6, 3]), 9);
        assert_eq!(func(vec![1, 2, 3, 4]), 0);
    }
    test(difference_of_sum);
}
