//! 凸多边形

/// 叉乘
pub fn is_convex(points: Vec<Vec<i32>>) -> bool {
    //相邻两边，取同一方向（顺时针或逆时针）化为向量，判断夹角是锐角还是钝角
    let len = points.len();
    let mut pre = 0;
    for i in 0..len {
        let x1 = points[(i + 1) % len][0] - points[i][0];
        let y1 = points[(i + 1) % len][1] - points[i][1];

        let x2 = points[(i + 2) % len][0] - points[(i + 1) % len][0];
        let y2 = points[(i + 2) % len][1] - points[(i + 1) % len][1];

        let tmp = x1 * y2 - x2 * y1;    //叉乘
        //与上一次的法向量的方向相反
        if tmp != 0 {
            if (pre as i64) * (tmp as i64) < 0 {
                return false;
            }
            pre = tmp;
        }
    }
    return true;
}

fn main() {
    fn test(func: fn(points: Vec<Vec<i32>>) -> bool) {
        assert_eq!(func(vec![vec![0, 0], vec![0, 5], vec![5, 5], vec![5, 0]]), true);
        assert_eq!(func(vec![vec![0, 0], vec![0, 10], vec![10, 10], vec![10, 0], vec![5, 5]]), false);
    }
    test(is_convex);
}
