//! 构造相同颜色的正方形

pub fn can_make_square(grid: Vec<Vec<char>>) -> bool {
    for i in 0..2 {
        for j in 0..2 {
            let mut cnt = 0;
            for x in i..i + 2 {
                for y in j..j + 2 {
                    if grid[x][y] == 'B' { cnt += 1; }
                }
            }
            if matches!(cnt, 0|1|3|4) { return true; }
        }
    }
    false
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<char>>) -> bool) {
        assert_eq!(func(vec![vec!['B', 'W', 'B'], vec!['B', 'W', 'W'], vec!['B', 'W', 'B']]), true);
        assert_eq!(func(vec![vec!['B', 'W', 'B'], vec!['W', 'B', 'W'], vec!['B', 'W', 'B']]), false);
        assert_eq!(func(vec![vec!['B', 'W', 'B'], vec!['B', 'W', 'W'], vec!['B', 'W', 'W']]), true);
    }
    test(can_make_square);
}
