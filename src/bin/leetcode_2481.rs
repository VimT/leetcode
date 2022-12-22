//! 分割圆的最少切割次数

pub fn number_of_cuts(n: i32) -> i32 {
    if n == 1 { 0 } else if n & 1 == 0 { n / 2 } else { n }
}

fn main() {
    assert_eq!(number_of_cuts(1), 0);
    assert_eq!(number_of_cuts(4), 2);
    assert_eq!(number_of_cuts(3), 3);
    assert_eq!(number_of_cuts(6), 3);
}
