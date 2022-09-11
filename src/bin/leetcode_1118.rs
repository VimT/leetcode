//! 一月有多少天

pub fn number_of_days(year: i32, month: i32) -> i32 {
    match month {
        1 => 31,
        2 => {
            if year % 400 == 0 || (year % 100 != 0 && year % 4 == 0) {
                29
            } else {
                28
            }
        }
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        _ => unreachable!()
    }
}

fn main() {
    fn test(func: fn(year: i32, month: i32) -> i32) {
        assert_eq!(func(1992, 7), 31);
        assert_eq!(func(2000, 2), 29);
        assert_eq!(func(1900, 2), 28);
    }
    test(number_of_days);
}
