//! 整数转罗马数字

pub fn int_to_roman(mut num: i32) -> String {
    fn get(num: u8, unit: char, unit5: char, unit10: char) -> String {
        return match num {
            1 | 2 | 3 => unit.to_string().repeat(num as usize),
            4 => format!("{}{}", unit, unit5),
            5 => unit5.to_string(),
            6 | 7 | 8 => format!("{}{}", unit5, unit.to_string().repeat((num - 5) as usize)),
            9 => format!("{}{}", unit, unit10),
            _ => "".to_string(),
        };
    }
    let mut ans = vec![];
    ans.push(get((num / 1000) as u8, 'M', ' ', ' '));
    num %= 1000;
    ans.push(get((num / 100) as u8, 'C', 'D', 'M'));
    num %= 100;
    ans.push(get((num / 10) as u8, 'X', 'L', 'C'));
    num %= 10;
    ans.push(get(num as u8, 'I', 'V', 'X'));
    ans.into_iter().collect()
}

pub fn roman_to_int(s: String) -> i32 {
    let mut result = 0;
    let mut last = 0;

    for b in s.bytes().rev() {
        let n = match b {
            b'I' => 1,
            b'V' => 5,
            b'X' => 10,
            b'L' => 50,
            b'C' => 100,
            b'D' => 500,
            b'M' => 1000,
            _ => 0,
        };
        result += if n < last { -n } else { n };
        last = n;
    }
    result
}

fn main() {
    assert_eq!(int_to_roman(3), "III".to_string());
    assert_eq!(int_to_roman(4), "IV".to_string());
    assert_eq!(int_to_roman(9), "IX".to_string());
    assert_eq!(int_to_roman(58), "LVIII".to_string());
    assert_eq!(int_to_roman(1994), "MCMXCIV".to_string());
}
