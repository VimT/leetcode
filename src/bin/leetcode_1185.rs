//! 一周中的第几天

pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
    const WEEK: [&str; 7] = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
    const MONTH_DAYS: [i32; 11] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30];
    let mut days = 0;
    // 输入年份之前的年份的天数贡献
    days += 365 * (year - 1971) + (year - 1969) / 4;
    // 输入年份中，输入月份之前的月份的天数贡献
    days += MONTH_DAYS[..month as usize - 1].iter().sum::<i32>();
    if (year % 400 == 0 || (year % 4 == 0 && year % 100 != 0)) && month >= 3 {
        days += 1;
    }
    // 输入月份中的天数贡献
    days += day;
    return WEEK[(days as usize + 3) % 7].to_string();
}

fn main() {
    assert_eq!(day_of_the_week(31, 8, 2019), String::from("Saturday"));
    assert_eq!(day_of_the_week(18, 7, 1999), String::from("Sunday"));
    assert_eq!(day_of_the_week(15, 8, 1993), String::from("Sunday"));
}
