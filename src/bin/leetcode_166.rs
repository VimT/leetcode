//! 分数到小数

use std::collections::HashMap;

pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
    let mut numerator = numerator as i64;
    let mut denominator = denominator as i64;
    let mut map = HashMap::new();
    if denominator == 0 {
        return "".to_string();
    }
    if numerator == 0 {
        return "0".to_string();
    }
    let mut big = numerator / denominator;
    let mut small = vec![];
    let mut next = (numerator % denominator) * 10;
    let mut prefix = "";
    if (numerator > 0) ^ (denominator > 0) {
        prefix = "-";
    }
    if numerator < 0 || denominator < 0 {
        next = next.abs();
        big = big.abs();
        numerator = numerator.abs();
        denominator = denominator.abs();
    }
    let mut loop_start: i32 = -1;
    let mut i = 0;

    while next != 0 {
        let divisor = next / denominator;
        let remainder = next % denominator;
        let key = next;
        if map.contains_key(&key) {
            loop_start = *map.get(&key).unwrap();
            break;
        }
        small.push(divisor);
        map.insert(key, i);
        next = remainder * 10;
        i += 1;
    }
    if small.is_empty() {
        return format!("{}{}", prefix, big);
    }
    let decimals: String = unsafe { String::from_utf8_unchecked(small.into_iter().map(|x| x as u8 + b'0').collect()) };
    return if loop_start == -1 {
        format!("{}{}.{}", prefix, big, decimals)
    } else {
        format!("{}{}.{}({})", prefix, big, &decimals[0..loop_start as usize], &decimals[loop_start as usize..])
    };
}

fn main() {
    assert_eq!(fraction_to_decimal(1, 2), String::from("0.5"));
    assert_eq!(fraction_to_decimal(2, 1), String::from("2"));
    assert_eq!(fraction_to_decimal(4, 333), String::from("0.(012)"));
}
