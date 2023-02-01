//! 形成三的最大倍数

pub fn largest_multiple_of_three(digits: Vec<i32>) -> String {
    let len = digits.len();
    let mut yu = vec![vec![]; 3];
    let mut sum = 0;
    for num in digits {
        yu[(num % 3) as usize].push(num);
        sum += num;
    }
    for i in &mut yu[1..] {
        i.sort_unstable();
        i.reverse();
    }
    match sum % 3 {
        0 => {}
        1 => {
            if !yu[1].is_empty() {
                yu[1].pop();
            } else if yu[2].len() > 2 {
                yu[2].pop();
                yu[2].pop();
            } else {
                yu[1].clear();
                yu[2].clear();
            }
        }
        2 => {
            if !yu[2].is_empty() {
                yu[2].pop();
            } else if yu[1].len() > 2 {
                yu[1].pop();
                yu[1].pop();
            } else {
                yu[1].clear();
                yu[2].clear();
            }
        }
        _ => unreachable!()
    }
    let mut digits = Vec::with_capacity(len);
    for i in 0..3 {
        digits.extend_from_slice(&yu[i]);
    }
    digits.sort_unstable();
    while digits.len() > 1 && *digits.last().unwrap() == 0 {
        digits.pop();
    }
    digits.reverse();
    unsafe { String::from_utf8_unchecked(digits.into_iter().map(|x| x as u8 + b'0').collect()) }
}

fn main() {
    fn test(func: fn(digits: Vec<i32>) -> String) {
        assert_eq!(func(vec![9, 8, 6, 8, 6]), String::from("966"));
        assert_eq!(func(vec![0, 0, 0, 0, 0, 0]), String::from("0"));
        assert_eq!(func(vec![8, 1, 9]), String::from("981"));
        assert_eq!(func(vec![8, 6, 7, 1, 0]), String::from("8760"));
        assert_eq!(func(vec![1]), String::from(""));
    }
    test(largest_multiple_of_three);
}
