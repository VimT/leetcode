//! 人员站位的方案数 I

pub fn number_of_pairs(points: Vec<Vec<i32>>) -> i32 {
    let len = points.len();
    let mut result = 0;
    for i in 0..len {
        for j in i + 1..len {
            let (mut a, mut b) = (points[i][0], points[i][1]);
            let (mut c, mut d) = (points[j][0], points[j][1]);
            if (c <= a) && b <= d {
                let tmp = (a, b);
                a = c;
                b = d;
                c = tmp.0;
                d = tmp.1;
            }
            if c >= a && b >= d {
                let mut ok = true;
                for k in 0..len {
                    if k != i && k != j {
                        let (x, y) = (points[k][0], points[k][1]);
                        // 判断 (x,y) 是否在 (a,b) 和 (c,d) 构成的矩形内
                        if x >= a && x <= c && y >= d && y <= b {
                            ok = false;
                            break;
                        }
                    }
                }
                if ok { result += 1; }
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(points: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![1, 1], vec![2, 2], vec![3, 3]]), 0);
        assert_eq!(func(vec![vec![6, 2], vec![4, 4], vec![2, 6]]), 2);
        assert_eq!(func(vec![vec![3, 1], vec![1, 3], vec![1, 1]]), 2);
    }
    test(number_of_pairs);
}
