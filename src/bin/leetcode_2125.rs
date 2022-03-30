//! 银行中的激光束数量

use leetcode::svec;

pub fn number_of_beams(bank: Vec<String>) -> i32 {
    let mut last = None;
    let mut result = 0;
    for line in bank {
        let s = line.as_bytes();
        let mut cnt = 0;
        for &ch in s {
            if ch == b'1' { cnt += 1; }
        }
        if cnt > 0 {
            if let Some(v) = last { result += v * cnt; }
            last = Some(cnt);
        }
    }
    result
}

fn main() {
    assert_eq!(number_of_beams(svec!["011001", "000000", "010100", "001000"]), 8);
    assert_eq!(number_of_beams(svec!["000", "111", "000"]), 0);
}
