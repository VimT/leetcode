//! 最小化曼哈顿距离

use leetcode::multi_set::BtreeMultiSet;

/// 曼哈顿距离 转 切比雪夫距离
pub fn minimum_distance(points: Vec<Vec<i32>>) -> i32 {
    let mut xs = BtreeMultiSet::new();
    let mut ys = BtreeMultiSet::new();
    for point in &points {
        let (x, y) = (point[0], point[1]);
        xs.insert(x + y);
        ys.insert(y - x);
    }
    let mut result = i32::MAX;
    for point in &points {
        let (x, y) = (point[0] + point[1], point[1] - point[0]);
        xs.remove(x);
        ys.remove(y);
        let xs_diff = *xs.last().unwrap() - *xs.first().unwrap();
        let ys_diff = *ys.last().unwrap() - *ys.first().unwrap();
        result = result.min(xs_diff.max(ys_diff));
        xs.insert(x);
        ys.insert(y);
    }
    result
}

/// 维护 x‘ 和 y’ 的最大次大、最小次小，一共 8 个数
pub fn minimum_distance2(points: Vec<Vec<i32>>) -> i32 {
    let (mut max_x1, mut max_x2, mut max_y1, mut max_y2) = (i32::MIN, i32::MIN, i32::MIN, i32::MIN);
    let (mut min_x1, mut min_x2, mut min_y1, mut min_y2) = (i32::MAX, i32::MAX, i32::MAX, i32::MAX);
    let (mut max_xi, mut min_xi, mut max_yi, mut min_yi) = (0, 0, 0, 0);

    for (i, point) in points.iter().enumerate() {
        let (x, y) = (point[0] + point[1], point[1] - point[0]);

        // x 最大次大
        if x > max_x1 {
            max_x2 = max_x1;
            max_x1 = x;
            max_xi = i;
        } else if x > max_x2 {
            max_x2 = x;
        }

        // x 最小次小
        if x < min_x1 {
            min_x2 = min_x1;
            min_x1 = x;
            min_xi = i;
        } else if x < min_x2 {
            min_x2 = x;
        }

        // y 最大次大
        if y > max_y1 {
            max_y2 = max_y1;
            max_y1 = y;
            max_yi = i;
        } else if y > max_y2 {
            max_y2 = y;
        }

        // y 最小次小
        if y < min_y1 {
            min_y2 = min_y1;
            min_y1 = y;
            min_yi = i;
        } else if y < min_y2 {
            min_y2 = y;
        }
    }

    let mut ans = i32::MAX;
    for &i in &[max_xi, min_xi, max_yi, min_yi] {
        let dx = if i == max_xi { max_x2 } else { max_x1 } - if i == min_xi { min_x2 } else { min_x1 };
        let dy = if i == max_yi { max_y2 } else { max_y1 } - if i == min_yi { min_y2 } else { min_y1 };
        ans = ans.min(dx.max(dy));
    }

    ans
}

fn main() {
    fn test(func: fn(points: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![3, 10], vec![5, 15], vec![10, 2], vec![4, 4]]), 12);
        assert_eq!(func(vec![vec![1, 1], vec![1, 1], vec![1, 1]]), 0);
    }
    test(minimum_distance);
    test(minimum_distance2);
}
