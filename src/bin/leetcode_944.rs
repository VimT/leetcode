//! 删列造序

use leetcode::svec;

pub fn min_deletion_size(strs: Vec<String>) -> i32 {
    let m = strs.len();
    let n = strs[0].len();
    let mut result = 0;
    for j in 0..n {
        for i in 1..m {
            if strs[i].as_bytes()[j] < strs[i - 1].as_bytes()[j] {
                result += 1;
                break;
            }
        }
    }
    result
}

fn main() {
    assert_eq!(min_deletion_size(svec!["cba", "daf", "ghi"]), 1);
    assert_eq!(min_deletion_size(svec!["a", "b"]), 0);
    assert_eq!(min_deletion_size(svec!["zyx", "wvu", "tsr"]), 3);
}
