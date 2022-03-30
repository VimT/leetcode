//! 删列造序 II

use leetcode::svec;

pub fn min_deletion_size(strs: Vec<String>) -> i32 {
    let m = strs.len();
    let n = strs[0].len();
    let mut ns = vec![vec![]; m];
    for j in 0..n {
        for i in 0..m {
            ns[i].push(strs[i].as_bytes()[j]);
        }
        let mut sorted = true;
        for i in 1..m {
            if ns[i] < ns[i - 1] {
                sorted = false;
                break;
            }
        }
        if !sorted {
            for i in 0..m {
                ns[i].pop();
            }
        }
    }
    (n - ns[0].len()) as i32
}

pub fn min_deletion_size_optimise(strs: Vec<String>) -> i32 {
    let m = strs.len();
    let n = strs[0].len();
    let mut cuts = vec![false; m - 1];
    let mut result = 0;
    'out: for j in 0..n {
        for i in 0..m - 1 {
            if !cuts[i] && strs[i].as_bytes()[j] > strs[i + 1].as_bytes()[j] {
                result += 1;
                continue 'out;
            }
        }
        for i in 0..m - 1 {
            if strs[i].as_bytes()[j] < strs[i + 1].as_bytes()[j] {
                cuts[i] = true;
            }
        }
    }
    result
}

fn main() {
    assert_eq!(min_deletion_size(svec!["ca", "bb", "ac"]), 1);
    assert_eq!(min_deletion_size(svec!["xc", "yb", "za"]), 0);
    assert_eq!(min_deletion_size(svec!["zyx", "wvu", "tsr"]), 3);
    assert_eq!(min_deletion_size_optimise(svec!["ca", "bb", "ac"]), 1);
    assert_eq!(min_deletion_size_optimise(svec!["xc", "yb", "za"]), 0);
    assert_eq!(min_deletion_size_optimise(svec!["zyx", "wvu", "tsr"]), 3);
}
