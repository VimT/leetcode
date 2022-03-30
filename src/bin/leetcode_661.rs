//! 图片平滑器

pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = img.len();
    let n = img[0].len();
    let dir = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 0), (0, 1), (1, -1), (1, 0), (1, 1)];
    let mut result = vec![vec![0; n]; m];
    for i in 0..m {
        for j in 0..n {
            let mut sum = 0;
            let mut cnt = 0;
            for (dx, dy) in dir {
                let (nx, ny) = (i as i32 + dx, j as i32 + dy);
                if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                    cnt += 1;
                    sum += img[nx as usize][ny as usize];
                }
            }
            result[i][j] = sum / cnt;
        }
    }
    result
}

fn main() {
    assert_eq!(image_smoother(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]), vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]);
    assert_eq!(image_smoother(vec![vec![100, 200, 100], vec![200, 50, 200], vec![100, 200, 100]]), vec![vec![137, 141, 137], vec![141, 138, 141], vec![137, 141, 137]]);
}
