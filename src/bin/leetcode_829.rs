//! 连续整数求和

pub fn consecutive_numbers_sum(n: i32) -> i32 {
    let mut result = 1;
    let start = 2;
    for len in start..=(2. * n as f64).sqrt() as i32 {
        if 2 * n % len == 0 {
            let y = 2 * n / len - len - 1;
            if y % 2 == 0 && y >= 0 {
                result += 1;
            }
        }
    }
    result
}

fn main() {
    assert_eq!(consecutive_numbers_sum(9), 3);
    assert_eq!(consecutive_numbers_sum(5), 2);
    assert_eq!(consecutive_numbers_sum(15), 4);
}
