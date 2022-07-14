//! 统计圆内格点数目

use std::collections::HashSet;

fn dis(x: i32, y: i32) -> f64 {
    return ((x * x) as f64 + (y * y) as f64).sqrt();
}

pub fn count_lattice_points(circles: Vec<Vec<i32>>) -> i32 {
    let mut points = HashSet::new();
    for circle in circles {
        let l = circle[0] - circle[2];
        let r = circle[0] + circle[2];
        let d = circle[1] - circle[2];
        let u = circle[1] + circle[2];
        for x in l..=r {
            for y in d..=u {
                if dis(x - circle[0], y - circle[1]) <= circle[2] as f64 {
                    points.insert((x, y));
                }
            }
        }
    }
    points.len() as i32
}

pub fn count_lattice_points_best(circles: Vec<Vec<i32>>) -> i32 {
    let mut max_x = 0;
    let mut max_y = 0;
    for circle in &circles {
        max_x = max_x.max(circle[0] + circle[2]);
        max_y = max_y.max(circle[1] + circle[2]);
    }
    max_y += 1;
    let mut diffs: Vec<Vec<i32>> = Vec::new();
    for i in 0..max_y {
        let mut diff = vec![0; max_x as usize + 1];
        for circle in &circles {
            let y1 = circle[1] + circle[2];
            let y2 = circle[1] - circle[2];
            if i < y2 || i > y1 {
                continue;
            }
            let mut x1 = ((circle[2]).pow(2) - (i - circle[1]).pow(2)) as usize;
            x1 = (x1 as f64).sqrt().floor() as usize;
            let x2 = x1 + circle[0] as usize;
            x1 = circle[0] as usize - x1;
            diff[x1] += 1;
            if x2 < max_x as usize {
                diff[x2 + 1] -= 1;
            }
        }
        diffs.push(diff);
    }
    let mut res = 0 as i32;
    for diff in diffs {
        let mut sum = 0;
        for j in 0..=max_x {
            sum = sum + diff[j as usize];
            res += sum.min(1);
        }
    }
    res
}

fn main() {
    fn test(func: fn(circles: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![2, 2, 1]]), 5);
        assert_eq!(func(vec![vec![2, 2, 2], vec![3, 4, 1]]), 16);
    }
    test(count_lattice_points);
    test(count_lattice_points_best);
}
