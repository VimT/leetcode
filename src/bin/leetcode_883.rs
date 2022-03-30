//! 三维形体投影面积

pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
    let len = grid.len();
    let mut result = 0;
    for i in 0..len {
        for j in 0..len {
            if grid[i][j] > 0 {
                result += 1;
            }
        }
    }
    for i in 0..len {
        let mut mh = 0;
        for j in 0..len {
            mh = mh.max(grid[i][j]);
        }
        result += mh;
    }
    for j in 0..len {
        let mut mh = 0;
        for i in 0..len {
            mh = mh.max(grid[i][j]);
        }
        result += mh;
    }
    result
}

fn main() {
    assert_eq!(projection_area(vec![vec![1, 2], vec![3, 4]]), 17);
    assert_eq!(projection_area(vec![vec![2]]), 5);
    assert_eq!(projection_area(vec![vec![1, 0], vec![0, 2]]), 8);
}
