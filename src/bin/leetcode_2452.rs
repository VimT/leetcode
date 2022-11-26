//! 距离字典两次编辑以内的单词


use leetcode::svec;

pub fn two_edit_words(queries: Vec<String>, dictionary: Vec<String>) -> Vec<String> {
    queries.into_iter().filter(|query| {
        let q = query.as_bytes();
        for dict in &dictionary {
            let d = dict.as_bytes();
            let mut diff = 0;
            for i in 0..q.len() {
                if q[i] != d[i] {
                    diff += 1;
                }
                if diff > 2 { break; }
            }
            if diff <= 2 {
                return true;
            }
        }
        false
    }).collect()
}

fn main() {
    assert_eq!(two_edit_words(svec!["word","note","ants","wood"], svec!["wood","joke","moat"]), svec!["word","note","wood"]);
    assert_eq!(two_edit_words(svec!["yes"], svec!["not"]), svec![]);
}
