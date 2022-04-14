//! 选择建筑的方案数

pub fn number_of_ways(s: String) -> i64 {
    let mut c0 = 0;
    let mut c1 = 0;
    let mut c01 = 0;
    let mut c10 = 0;
    let mut result = 0;
    for &ch in s.as_bytes() {
        if ch == b'0' {
            c0 += 1;
            c10 += c1;
            result += c01;
        } else {
            c1 += 1;
            c01 += c0;
            result += c10;
        }
    }
    result
}

fn main() {
    assert_eq!(number_of_ways(String::from("001101")), 6);
    assert_eq!(number_of_ways(String::from("11100")), 0);
}
