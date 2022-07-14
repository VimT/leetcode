//! Excel 表中某个范围内的单元格

pub fn cells_in_range(s: String) -> Vec<String> {
    let (cr1, cr2) = s.split_once(":").unwrap();
    let c1 = cr1.as_bytes()[0];
    let r1 = cr1.as_bytes()[1];
    let c2 = cr2.as_bytes()[0];
    let r2 = cr2.as_bytes()[1];
    let mut result = vec![];
    for col in c1..=c2 {
        for row in r1..=r2 {
            result.push(format!("{}{}", col as char, row as char));
        }
    }
    result
}

fn main() {
    assert_eq!(cells_in_range(String::from("K1:L2")), vec!["K1", "K2", "L1", "L2"]);
    assert_eq!(cells_in_range(String::from("A1:F1")), vec!["A1", "B1", "C1", "D1", "E1", "F1"]);
}
