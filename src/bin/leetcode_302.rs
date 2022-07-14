//! 包含全部黑色像素的最小矩形

/// 投影到x的长度*投影到y的长度
pub fn min_area(image: Vec<Vec<char>>, _: i32, _: i32) -> i32 {
    let m = image.len();
    let n = image[0].len();
    let mut x1 = usize::MAX;
    let mut x2 = 0;
    let mut y1 = usize::MAX;
    let mut y2 = 0;
    for i in 0..m {
        for j in 0..n {
            if image[i][j] == '1' {
                x1 = x1.min(i);
                x2 = x2.max(i);
                y1 = y1.min(j);
                y2 = y2.max(j);
            }
        }
    }
    ((x2 + 1 - x1) * (y2 + 1 - y1)) as i32
}

fn main() {
    fn test(func: fn(image: Vec<Vec<char>>, x: i32, y: i32) -> i32) {
        assert_eq!(func(vec![vec!['0', '0', '1', '0'], vec!['0', '1', '1', '0'], vec!['0', '1', '0', '0']], 0, 2), 6);
        assert_eq!(func(vec![vec!['1']], 0, 0), 1);
    }
    test(min_area);
}
