//! 服务中心的最佳位置

use rand::thread_rng;
use rand::seq::SliceRandom;

fn dis(x0: f64, y0: f64, x1: f64, y1: f64) -> f64 {
    let a = x0 - x1;
    let b = y0 - y1;
    (a * a + b * b).sqrt()
}

/// 批量梯度下降法
pub fn get_min_dist_sum(mut positions: Vec<Vec<i32>>) -> f64 {
    fn dis(x0: f64, y0: f64, x1: f64, y1: f64, other: f64) -> f64 {
        let a = x0 - x1;
        let b = y0 - y1;
        (a * a + b * b + other).sqrt()
    }

    let eps = 1e-15;
    let mut alpha = 1.0;
    let decay = 1e-3;
    let n = positions.len();
    let batch_size = n; // 批大小
    let mut x = 0.0;
    let mut y = 0.0;
    for pos in &positions {
        x += pos[0] as f64;
        y += pos[1] as f64;
    }
    x /= n as f64;
    y /= n as f64;

    let mut rng = thread_rng();
    loop {
        positions.shuffle(&mut rng);
        let x_prev = x;
        let y_prev = y;

        let mut i = 0;
        while i < n {
            let j = (i + batch_size).min(n);
            let mut dx = 0.0;
            let mut dy = 0.0;
            // 计算导数
            for k in i..j {
                let (x0, y0) = (positions[k][0] as f64, positions[k][1] as f64);
                let m = dis(x, y, x0, y0, eps);// eps: 防止分母为0
                dx += (x - x0) / m;
                dy += (y - y0) / m;
            }
            x -= alpha * dx;
            y -= alpha * dy;

            // 每一轮学习后，将学习率衰减
            alpha *= 1.0 - decay;
            i += batch_size;
        }

        // 是否结束迭代
        if dis(x, y, x_prev, y_prev, 0.0) < eps {
            break;
        }
    }

    // 计算 中心(xc, yc) 到每个点的 欧几里得距离之和
    let mut result = 0.0;
    for pos in &positions {
        result += dis(pos[0] as f64, pos[1] as f64, x, y, 0.0);
    }
    result
}

/// 爬山法
pub fn get_min_dist_sum2(positions: Vec<Vec<i32>>) -> f64 {
    static DIR: [(f64, f64); 4] = [(-1., 0.), (1., 0.), (0., -1.), (0., 1.)];
    let eps = 1e-7;
    let mut step = 1.0;
    let decay = 0.5;

    let n = positions.len();
    let mut x = 0.0;
    let mut y = 0.0;
    for pos in &positions {
        x += pos[0] as f64;
        y += pos[1] as f64;
    }
    x /= n as f64;
    y /= n as f64;

    // 计算 中心(xc, yc) 到每个点的 欧几里得距离之和
    let get_dist = |xc: f64, yc: f64| -> f64 {
        let mut result = 0.0;
        for pos in &positions {
            result += dis(pos[0] as f64, pos[1] as f64, xc, yc);
        }
        result
    };

    while step > eps {
        let mut modified = false;
        for (dx, dy) in DIR {
            let x_next = x + step * dx;
            let y_next = y + step * dy;
            if get_dist(x_next, y_next) < get_dist(x, y) {
                x = x_next;
                y = y_next;
                modified = true;
                break;
            }
        }
        if !modified {
            step *= 1.0 - decay;
        }
    }


    get_dist(x, y)
}

/// 三分查找
pub fn get_min_dist_sum3(positions: Vec<Vec<i32>>) -> f64 {
    let eps = 1e-7;
    // 计算 中心(xc, yc) 到每个点的 欧几里得距离之和
    let get_dist = |xc: f64, yc: f64| -> f64 {
        let mut result = 0.0;
        for pos in &positions {
            result += dis(pos[0] as f64, pos[1] as f64, xc, yc);
        }
        result
    };

    let check_optimal = |xc: f64| -> f64 {
        let mut y_left = 0.0;
        let mut y_right = 100.0;
        while y_right - y_left > eps {
            let y_first = (y_left + y_left + y_right) / 3.0;
            let y_second = (y_left + y_right + y_right) / 3.0;
            if get_dist(xc, y_first) < get_dist(xc, y_second) {
                y_right = y_second;
            } else {
                y_left = y_first;
            }
        }
        get_dist(xc, y_left)
    };

    let mut x_left = 0.0;
    let mut x_right = 100.0;
    while x_right - x_left > eps {
        let x_first = (x_left + x_left + x_right) / 3.0; // 左1/3点
        let x_second = (x_left + x_right + x_right) / 3.0; // 右1/3点
        if check_optimal(x_first) < check_optimal(x_second) {
            x_right = x_second;
        } else {
            x_left = x_first;
        }
    }
    check_optimal(x_left)
}

fn main() {
    fn test(func: fn(positions: Vec<Vec<i32>>) -> f64) {
        let check = |input: Vec<Vec<i32>>, expect: f64| {
            let result = func(input);
            assert!((result - expect).abs() < 1e-5, "{}, {}", expect, result);
        };
        check(vec![vec![27,90],vec![99,75],vec![99,38]], 109.30317);
        check(vec![vec![1, 1], vec![0, 0], vec![2, 0]], 2.73205);
        check(vec![vec![0, 1], vec![1, 0], vec![1, 2], vec![2, 1]], 4.00000);
        check(vec![vec![1, 1], vec![3, 3]], 2.82843);
        check(vec![vec![44, 23], vec![18, 45], vec![6, 73], vec![0, 76], vec![10, 50], vec![30, 7], vec![92, 59], vec![44, 59], vec![79, 45], vec![69, 37], vec![66, 63], vec![10, 78], vec![88, 80], vec![44, 87]], 499.28078);
        check(vec![vec![4, 4], vec![52, 89], vec![76, 60], vec![4, 4], vec![4, 4], vec![93, 59], vec![50, 92], vec![4, 4], vec![76, 14], vec![4, 4], vec![46, 41], vec![4, 4], vec![4, 4], vec![4, 4], vec![4, 4], vec![67, 14], vec![73, 71], vec![83, 44], vec![4, 4], vec![4, 4], vec![4, 4], vec![4, 4], vec![30, 29], vec![74, 77], vec![4, 4], vec![4, 4], vec![4, 4], vec![26, 62], vec![4, 4], vec![4, 4], vec![50, 30], vec![44, 93]], 1119.58582);
    }
    test(get_min_dist_sum);
    test(get_min_dist_sum2);
    test(get_min_dist_sum3);
}
