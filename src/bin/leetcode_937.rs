//! 重新排列日志文件

use leetcode::svec;

pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
    let mut dig = vec![];
    let mut alp = Vec::with_capacity(logs.len());
    for log in logs {
        let (_, content) = log.split_once(" ").unwrap();
        if content.as_bytes()[0].is_ascii_digit() {
            dig.push(log);
        } else {
            alp.push(log);
        }
    }
    alp.sort_unstable_by(|x, y| {
        let a = x.split_once(" ").unwrap();
        let b = y.split_once(" ").unwrap();
        (a.1, a.0).cmp(&(b.1, b.0))
    });
    alp.extend(dig);
    alp
}

fn main() {
    assert_eq!(reorder_log_files(svec!["dig1 8 1 5 1", "let1 art can", "dig2 3 6", "let2 own kit dig", "let3 art zero"]), svec!["let1 art can", "let3 art zero", "let2 own kit dig", "dig1 8 1 5 1", "dig2 3 6"]);
    assert_eq!(reorder_log_files(svec!["a1 9 2 3 1", "g1 act car", "zo4 4 7", "ab1 off key dog", "a8 act zoo"]), svec!["g1 act car", "a8 act zoo", "ab1 off key dog", "a1 9 2 3 1", "zo4 4 7"]);
}
