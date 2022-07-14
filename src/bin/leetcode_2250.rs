//! 统计包含每个点的矩形数目

pub fn count_rectangles(rectangles: Vec<Vec<i32>>, points: Vec<Vec<i32>>) -> Vec<i32> {
    const MAX_HEIGHT: usize = 101;
    let mut heights = vec![vec![]; MAX_HEIGHT];
    for rec in rectangles {
        for h in 1..=rec[1] {
            heights[h as usize].push(rec[0]);
        }
    }
    for height in &mut heights {
        height.sort_unstable();
    }
    points.into_iter().map(|point| {
        let line = &heights[point[1] as usize];
        let mut left = 0;
        let mut right = line.len();
        let t = point[0];
        while left < right {
            let mid = (left + right) >> 1;
            if line[mid] >= t {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        (line.len() - left) as i32
    }).collect()
}

/// 按横坐标排序
/// 区间题，还可以考虑线段树
pub fn count_rectangles_sort(rectangles: Vec<Vec<i32>>, points: Vec<Vec<i32>>) -> Vec<i32> {
    let mut coors = Vec::with_capacity(rectangles.len() + points.len());
    let plen = points.len();
    for rec in rectangles {
        coors.push((rec[0], rec[1], plen));
    }
    for i in 0..plen {
        coors.push((points[i][0], points[i][1], i));
    }

    coors.sort_unstable();
    let mut res = vec![0; 101];
    let mut result = vec![0; plen];

    for (__, y, t) in coors.into_iter().rev() {
        if t == plen {
            res[y as usize] += 1;
        } else {
            result[t] = res[y as usize..].iter().sum();
        }
    }
    result
}

fn main() {
    fn test(func: fn(rectangles: Vec<Vec<i32>>, points: Vec<Vec<i32>>) -> Vec<i32>) {
        assert_eq!(func(vec![vec![1, 2], vec![2, 3], vec![2, 5]], vec![vec![2, 1], vec![1, 4]]), vec![2, 1]);
        assert_eq!(func(vec![vec![1, 1], vec![2, 2], vec![3, 3]], vec![vec![1, 3], vec![1, 1]]), vec![1, 3]);
    }
    test(count_rectangles);
    test(count_rectangles_sort);
}
