//! 圆形靶内的最大飞镖数量

pub fn num_points(points: Vec<Vec<i32>>, r: i32) -> i32 {
    fn dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
        ((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)).sqrt()
    }

    fn f(a: (f64, f64), b: (f64, f64), r: f64) -> (f64, f64) {
        let mid = ((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0);
        let d = dist(a.0, a.1, mid.0, mid.1);
        let h = (r * r - d * d).sqrt();
        let ba = (b.0 - a.0, b.1 - a.1);
        let mut hd = (-ba.1, ba.0);
        let len = (hd.0 * hd.0 + hd.1 * hd.1).sqrt();
        hd.0 /= len;
        hd.1 /= len;
        hd.0 *= h;
        hd.1 *= h;
        return (hd.0 + mid.0, hd.1 + mid.1);
    }
    let r = r as f64;
    let len = points.len();
    let mut result = 0;
    for i in 0..len {
        for j in 0..len {
            let mid = if i == j {
                (points[i][0] as f64, points[i][1] as f64)
            } else {
                let a = (points[i][0] as f64, points[i][1] as f64);
                let b = (points[j][0] as f64, points[j][1] as f64);
                let d = dist(a.0, a.1, b.0, b.1);
                if d / 2.0 > r { continue; }
                f(a, b, r)
            };
            let mut cnt = 0;
            for k in 0..len {
                if dist(mid.0, mid.1, points[k][0] as f64, points[k][1] as f64) <= r {
                    cnt += 1;
                }
            }
            result = result.max(cnt);
        }
    }
    result
}

fn main() {
    assert_eq!(num_points(vec![vec![-3, 0], vec![3, 0], vec![2, 6], vec![5, 4], vec![0, 9], vec![7, 8]], 5), 5);
    assert_eq!(num_points(vec![vec![-2, 0], vec![2, 0], vec![0, 2], vec![0, -2]], 2), 4);
    assert_eq!(num_points(vec![vec![-2, 0], vec![2, 0], vec![0, 2], vec![0, -2]], 1), 1);
    assert_eq!(num_points(vec![vec![1, 2], vec![3, 5], vec![1, -1], vec![2, 3], vec![4, 1], vec![1, 3]], 2), 4);
}

