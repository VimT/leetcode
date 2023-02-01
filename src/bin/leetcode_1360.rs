//! 日期之间隔几天

const MONTH_DAYS: [i32; 11] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30];

fn datestamp(date: String) -> i32 {
    let mut result = 0;
    let input: Vec<i32> = date.split('-').map(|x| x.parse().unwrap()).collect();
    for year in 1971..input[0] {
        if year % 400 == 0 || (year % 100 != 0 && year % 4 == 0) {
            result += 366;
        } else {
            result += 365;
        }
    }
    let year = input[0];
    let run = year % 400 == 0 || (year % 100 != 0 && year % 4 == 0);
    for month in 1..input[1] {
        result += MONTH_DAYS[month as usize - 1];
        if run && month == 2 {
            result += 1;
        }
    }

    result + input[2]
}

pub fn days_between_dates(date1: String, date2: String) -> i32 {
    (datestamp(date1) - datestamp(date2)).abs()
}

fn main() {
    fn test(func: fn(date1: String, date2: String) -> i32) {
        assert_eq!(func(String::from("2019-06-29"), String::from("2019-06-30")), 1);
        assert_eq!(func(String::from("2020-01-15"), String::from("2019-12-31")), 15);
    }
    test(days_between_dates);
}
