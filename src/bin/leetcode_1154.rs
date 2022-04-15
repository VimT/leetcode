//! 一年中的第几天

static MONTH: [u32; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

pub fn day_of_year(date: String) -> i32 {
    let year: u32 = date[..4].parse().unwrap();
    let month: u32 = date[5..7].parse().unwrap();
    let day: u32 = date[8..].parse().unwrap();
    let is_run = year % 400 == 0 || (year % 100 != 0 && year % 4 == 0);
    let mut result = 0;
    for i in 1..month {
        result += MONTH[i as usize];
        if i == 2 && is_run { result += 1; }
    }
    (result + day) as i32
}

fn main() {
    fn test(func: fn(date: String) -> i32) {
        assert_eq!(func(String::from("2019-01-09")), 9);
        assert_eq!(func(String::from("2019-02-10")), 41);
        assert_eq!(func(String::from("2020-04-01")), 92);
        assert_eq!(func(String::from("2019-04-01")), 91);
    }
    test(day_of_year);
}
