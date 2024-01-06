//! 找到冠军 I

pub fn find_champion(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    for i in 0..n {
        if (0..n).all(|x| grid[x][i] == 0) {
            return i as i32;
        }
    }
    unreachable!()
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![0, 1], vec![0, 0]]), 0);
        assert_eq!(func(vec![vec![0, 0, 1], vec![1, 0, 1], vec![0, 0, 0]]), 1);
    }
    test(find_champion);
}
