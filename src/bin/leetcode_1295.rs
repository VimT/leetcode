//! 统计位数为偶数的数字

pub fn find_numbers(nums: Vec<i32>) -> i32 {
    nums.iter().filter(|x| x.to_string().len() & 1 == 0).count() as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![12, 345, 2, 6, 7896]), 2);
        assert_eq!(func(vec![555, 901, 482, 1771]), 1);
    }
    test(find_numbers);
}
