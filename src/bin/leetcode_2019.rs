//! 解出数学表达式的学生分数


use std::collections::HashSet;

pub fn score_of_students(s: String, answers: Vec<i32>) -> i32 {
    let s = s.as_bytes();
    fn do_cal(s: &[u8]) -> i32 {
        let mut pre = (s[0] - b'0') as i32;
        let len = s.len();
        let mut result = 0;
        for i in (1..len).step_by(2) {
            let op = s[i];
            let num = (s[i + 1] - b'0') as i32;
            match op {
                b'+' => {
                    result += pre;
                    pre = num;
                }
                b'*' => {
                    pre = pre * num;
                }
                _ => panic!(),
            }
        }
        result + pre
    }
    let len = s.len();
    let mut dp = vec![vec![vec![]; len]; len];
    for i in 0..len {
        if s[i].is_ascii_digit() {
            dp[i][i].push((s[i] - b'0') as i32);
        }
    }
    for step in (2..len).step_by(2) {
        for i in (0..len - step).step_by(2) {
            let j = i + step;
            let mut set = HashSet::new();
            for mid in (i + 1..j).step_by(2) {
                let op = s[mid];
                for &num1 in &dp[i][mid - 1] {
                    for &num2 in &dp[mid + 1][j] {
                        let new_val = match op {
                            b'+' => num1 + num2,
                            b'*' => num1 * num2,
                            _ => panic!()
                        };
                        if new_val <= 1000 && new_val >= 0 {
                            set.insert(new_val);
                        }
                    }
                }
            }
            dp[i][j] = set.iter().cloned().collect();
        }
    }
    let mut score = vec![0; 1001];
    for &i in &dp[0][len - 1] {
        if i <= 1000 && i >= 0 {
            score[i as usize] = 2;
        }
    }
    let correct = do_cal(s);
    score[correct as usize] = 5;
    answers.into_iter().map(|x| score[x as usize]).sum()
}

fn main() {
    assert_eq!(score_of_students("4+2*2+3*1+2".to_string(), vec![42, 17, 13, 66, 32, 547, 21, 90, 13, 33, 45, 66, 34, 21, 13, 13, 13, 46, 21, 177, 18, 13, 18, 16, 16, 678, 13, 42, 66, 13, 18, 18, 777, 21, 924, 13, 268, 13, 13, 13, 25, 62, 45, 33, 888, 779, 13, 206, 48, 13, 34, 17]), 122);
    assert_eq!(score_of_students("1+2*3+4".to_string(), vec![13, 21, 11, 15]), 11);
    assert_eq!(score_of_students("7+3*1*2".to_string(), vec![20, 13, 42]), 7);
    assert_eq!(score_of_students("3+5*2".to_string(), vec![13, 0, 10, 13, 13, 16, 16]), 19);
    assert_eq!(score_of_students("6+0*1".to_string(), vec![12, 9, 6, 4, 8, 6]), 10);
}

