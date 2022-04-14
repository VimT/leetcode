//! 交换数字

pub fn swap_numbers(mut numbers: Vec<i32>) -> Vec<i32> {
    numbers[0] = numbers[0] ^ numbers[1];
    numbers[1] = numbers[0] ^ numbers[1];
    numbers[0] = numbers[0] ^ numbers[1];
    return numbers;
}

fn main() {
    assert_eq!(swap_numbers(vec![1, 2]), vec![2, 1]);
}
