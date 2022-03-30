//! 棒球比赛

use leetcode::svec;

pub fn cal_points(ops: Vec<String>) -> i32 {
    let mut score: Vec<i32> = vec![];
    for op in ops {
        match op.as_str() {
            "+" => { score.push(score[score.len() - 1] + score[score.len() - 2]); }
            "D" => { score.push(score[score.len() - 1] * 2); }
            "C" => { score.pop().unwrap(); }
            _ => { score.push(op.parse::<i32>().unwrap()); }
        }
    }
    score.into_iter().sum()
}

fn main() {
    assert_eq!(cal_points(svec!["5", "2", "C", "D", "+"]), 30);
    assert_eq!(cal_points(svec!["5", "-2", "4", "C", "D", "9", "+", "+"]), 27);
    assert_eq!(cal_points(svec!["1"]), 1);
}
