//! 移动石子直到连续

pub fn num_moves_stones(a: i32, b: i32, c: i32) -> Vec<i32> {
    let mut s = vec![a, b, c];
    s.sort_unstable();
    let min = if s[1] == s[0] + 1 && s[2] == s[1] + 1 {
        0
    } else if s[1] == s[0] + 2 || s[2] == s[1] + 2 || s[1] == s[0] + 1 || s[2] == s[1] + 1 {
        1
    } else {
        2
    };
    let max = s[2] - s[0] - 2;
    vec![min, max]
}

fn main() {
    assert_eq!(num_moves_stones(1, 2, 5), vec![1, 2]);
    assert_eq!(num_moves_stones(4, 3, 2), vec![0, 0]);
    assert_eq!(num_moves_stones(3, 5, 1), vec![1, 2]);
}
