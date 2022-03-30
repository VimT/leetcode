//! 计算力扣银行的钱

pub fn total_money(n: i32) -> i32 {
    let weeks = n / 7;
    let first_week_money = (1 + 7) * 7 / 2;
    let last_week_money = first_week_money + 7 * (weeks - 1);
    let week_money = (first_week_money + last_week_money) * weeks / 2;
    let day_num = n % 7;
    let first_day_money = 1 + weeks;
    let last_day_money = first_day_money + day_num - 1;
    let day_money = (first_day_money + last_day_money) * day_num / 2;
    week_money + day_money
}

fn main() {
    assert_eq!(total_money(4), 10);
    assert_eq!(total_money(10), 37);
    assert_eq!(total_money(20), 96);
}
