//! 保龄球游戏的获胜者

pub fn is_winner(player1: Vec<i32>, player2: Vec<i32>) -> i32 {
    fn score(p: Vec<i32>) -> i32 {
        let mut result = 0;
        for (i, &s) in p.iter().enumerate() {
            if (i > 0 && p[i - 1] == 10) || (i > 1 && p[i - 2] == 10) {
                result += s * 2;
            } else {
                result += s;
            }
        }
        result
    }
    let s1 = score(player1);
    let s2 = score(player2);
    if s1 > s2 { 1 } else if s1 < s2 { 2 } else { 0 }
}

fn main() {
    fn test(func: fn(player1: Vec<i32>, player2: Vec<i32>) -> i32) {
        assert_eq!(func(vec![4, 10, 7, 9], vec![6, 5, 2, 3]), 1);
        assert_eq!(func(vec![3, 5, 7, 6], vec![8, 10, 10, 2]), 2);
        assert_eq!(func(vec![2, 3], vec![4, 1]), 0);
    }
    test(is_winner);
}
