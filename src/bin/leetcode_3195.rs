//! 包含所有 1 的最小矩形面积 I

pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut minx = m;
    let mut maxx = 0;
    let mut miny = n;
    let mut maxy = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 {
                minx = minx.min(i);
                maxx = maxx.max(i);
                miny = miny.min(j);
                maxy = maxy.max(j);
            }
        }
    }
    (maxx - minx + 1) as i32 * (maxy - miny + 1) as i32
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![0, 1, 0], vec![1, 0, 1]]), 6);
        assert_eq!(func(vec![vec![0, 0], vec![1, 0]]), 1);
    }
    test(minimum_area);
}
