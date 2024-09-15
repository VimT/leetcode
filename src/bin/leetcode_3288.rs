//! 最长上升路径的长度

/// LIS
pub fn max_path_length(mut coordinates: Vec<Vec<i32>>, k: i32) -> i32 {
    let (kx, ky) = (coordinates[k as usize][0], coordinates[k as usize][1]);
    // 当x相同时，按照y降序排列，这样相同的x最多选1个y
    coordinates.sort_unstable_by_key(|x| (x[0], -x[1]));
    let mut g: Vec<i32> = vec![];
    for coo in coordinates {
        let (x, y) = (coo[0], coo[1]);
        if x < kx && y < ky || x > kx && y > ky {
            let j = g.binary_search_by(|x| x.cmp(&y).then(std::cmp::Ordering::Greater)).unwrap_err();
            if j < g.len() { g[j] = y; } else { g.push(y); }
        }
    }
    g.len() as i32 + 1
}


fn main() {
    fn test(func: fn(coordinates: Vec<Vec<i32>>, k: i32) -> i32) {
        assert_eq!(func(vec![vec![0, 1], vec![0, 3]], 0), 1);
        assert_eq!(func(vec![vec![3, 1], vec![2, 2], vec![4, 1], vec![0, 0], vec![5, 3]], 1), 3);
        assert_eq!(func(vec![vec![2, 1], vec![7, 0], vec![5, 6]], 2), 2);
    }
    test(max_path_length);
}
