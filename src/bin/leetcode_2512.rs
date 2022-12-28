//! 奖励最顶尖的 K 名学生

use std::collections::HashSet;
use leetcode::svec;

pub fn top_students(positive_feedback: Vec<String>, negative_feedback: Vec<String>, report: Vec<String>, student_id: Vec<i32>, k: i32) -> Vec<i32> {
    let pos: HashSet<String> = positive_feedback.into_iter().collect();
    let neg: HashSet<String> = negative_feedback.into_iter().collect();
    let mut score: Vec<(i32, i32)> = report.into_iter().zip(student_id).map(|(report, id)| {
        let mut score = 0;
        for word in report.split(' ') {
            if pos.contains(word) {
                score -= 3;
            } else if neg.contains(word) {
                score += 1;
            }
        }
        (score, id)
    }).collect();
    score.sort_unstable();
    score[..k as usize].iter().map(|x| x.1).collect()
}

fn main() {
    assert_eq!(top_students(svec!["smart", "brilliant", "studious"], svec!["not"], svec!["this student is studious", "the student is smart"], vec![1, 2], 2), vec![1, 2]);
    assert_eq!(top_students(svec!["smart","brilliant","studious"], svec!["not"], svec!["this student is not studious","the student is smart"], vec![1, 2], 2), vec![2, 1]);
}
