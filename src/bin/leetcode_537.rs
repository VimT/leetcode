//! 复数乘法

pub fn complex_number_multiply(num1: String, num2: String) -> String {
    let (s1, x1) = num1.split_once('+').unwrap();
    let (s2, x2) = num2.split_once('+').unwrap();
    let (s1, x1): (i32, i32) = (s1.parse().unwrap(), x1[..x1.len() - 1].parse().unwrap());
    let (s2, x2): (i32, i32) = (s2.parse().unwrap(), x2[..x2.len() - 1].parse().unwrap());
    format!("{}+{}i", s1 * s2 - x1 * x2, x1 * s2 + x2 * s1)
}

fn main() {
    assert_eq!(complex_number_multiply(String::from("1+-1i"), String::from("0+0i")), String::from("0+0i"));
    assert_eq!(complex_number_multiply(String::from("1+1i"), String::from("1+1i")), String::from("0+2i"));
    assert_eq!(complex_number_multiply(String::from("1+-1i"), String::from("1+-1i")), String::from("0+-2i"));
}
