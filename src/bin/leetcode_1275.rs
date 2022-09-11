//! 找出井字棋的获胜者

pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
    let mut grid = [[0; 3]; 3];
    let len = moves.len();
    let mut cur = 1;
    for m in moves {
        grid[m[0] as usize][m[1] as usize] = cur;
        for i in 0..3 {
            if grid[i].iter().all(|x| *x == cur) || grid.iter().all(|x| x[i] == cur) {
                return String::from(if cur == 1 { "A" } else { "B" });
            }
        }
        if grid.iter().enumerate().all(|(i, x)| x[i] == cur) || grid.iter().enumerate().all(|(i, x)| x[2 - i] == cur) {
            return String::from(if cur == 1 { "A" } else { "B" });
        }
        cur = if cur == 1 { 2 } else { 1 }
    }
    if len == 9 { String::from("Draw") } else { String::from("pending") }
}

fn main() {
    fn test(func: fn(moves: Vec<Vec<i32>>) -> String) {
        assert_eq!(func(vec![vec![0, 0], vec![2, 0], vec![1, 1], vec![2, 1], vec![2, 2]]), String::from("A"));
        assert_eq!(func(vec![vec![0, 0], vec![1, 1], vec![0, 1], vec![0, 2], vec![1, 0], vec![2, 0]]), String::from("B"));
        assert_eq!(func(vec![vec![0, 0], vec![1, 1], vec![2, 0], vec![1, 0], vec![1, 2], vec![2, 1], vec![0, 1], vec![0, 2], vec![2, 2]]), String::from("Draw"));
    }
    test(tictactoe);
}
