//! 图像重叠

pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
    let n = img1.len() as i32;
    let mut result = 0;
    for dx in -n..n {
        for dy in -n..n {
            let mut cnt = 0;
            for i in 0..n {
                for j in 0..n {
                    if img2[i as usize][j as usize] == 1 {
                        let i1 = i + dx;
                        let j1 = j + dy;
                        if i1 >= 0 && i1 < n && j1 >= 0 && j1 < n {
                            if img1[i1 as usize][j1 as usize] == 1 {
                                cnt += 1;
                            }
                        }
                    }
                }
            }
            result = result.max(cnt);
        }
    }
    result
}

fn main() {
    assert_eq!(largest_overlap(vec![vec![1, 1, 0], vec![0, 1, 0], vec![0, 1, 0]], vec![vec![0, 0, 0], vec![0, 1, 1], vec![0, 0, 1]]), 3);
    assert_eq!(largest_overlap(vec![vec![1]], vec![vec![1]]), 1);
    assert_eq!(largest_overlap(vec![vec![0]], vec![vec![0]]), 0);
}
