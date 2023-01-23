//! 顺次数

pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
    let mut result = vec![];
    for len in low.to_string().len()..=high.to_string().len() + 1 {
        for start in 1..=10 - len {
            let mut num = start;
            for i in start + 1..start + len {
                num = num * 10 + i;
            }
            if num > high as usize { return result; }
            if num >= low as usize {
                result.push(num as i32);
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(low: i32, high: i32) -> Vec<i32>) {
        assert_eq!(func(100, 300), vec![123, 234]);
        assert_eq!(func(1000, 13000), vec![1234, 2345, 3456, 4567, 5678, 6789, 12345]);
    }
    test(sequential_digits);
}
