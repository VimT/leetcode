//! 买钢笔和铅笔的方案数

pub fn ways_to_buy_pens_pencils(total: i32, cost1: i32, cost2: i32) -> i64 {
    let mut result = 0;
    for i in 0..=total / cost1 {
        result += ((total - i * cost1) / cost2) as i64 + 1;
    }
    result
}

fn main() {
    assert_eq!(ways_to_buy_pens_pencils(20, 10, 5), 9);
    assert_eq!(ways_to_buy_pens_pencils(5, 10, 10), 1);
}
