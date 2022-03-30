//! 比赛中的配对次数

pub fn number_of_matches(mut n: i32) -> i32 {
    let mut cnt = 0;
    while n > 1 {
        cnt += n / 2;
        if n & 1 == 1 {
            n = n / 2 + 1;
        } else {
            n = n / 2;
        }
    }
    cnt
}

fn main() {
    assert_eq!(number_of_matches(7), 6);
    assert_eq!(number_of_matches(14), 13);
}
