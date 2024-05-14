//! 正方形中的最多点数

use std::collections::HashMap;

pub fn max_points_inside_square(points: Vec<Vec<i32>>, s: String) -> i32 {
    let mut points: Vec<(i32, usize)> = points.into_iter().zip(s.as_bytes()).map(|(point, &s)| (point[0].abs().max(point[1].abs()), (s - b'a') as usize)).collect();
    points.sort_unstable();
    let mut current = vec![-1; 26];
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut result = 0;
    for (point, s) in points {
        if current[s] == -1 {
            *map.entry(point).or_default() += 1;
            current[s] = point;
            result += 1;
        } else {
            result -= map.get(&point).copied().unwrap_or(0);
            return result;
        }
    }
    result
}

// O(n) 做法，找每个字母第二近的作为正方形。然后判断每个字母最近的是不是在这个框里
pub fn max_points_inside_square2(points: Vec<Vec<i32>>, s: String) -> i32 {
    let mut data = vec![[i32::MAX; 2]; 26];
    for (point, ch) in points.into_iter().zip(s.into_bytes()) {
        let side = point[0].abs().max(point[1].abs());
        let idx = (ch - b'a') as usize;
        if side < data[idx][0] {
            data[idx][1] = data[idx][0];
            data[idx][0] = side;
        } else if side < data[idx][1] {
            data[idx][1] = side;
        }
    }
    let side = data.iter().map(|x| x[1]).min().unwrap();
    data.iter().filter(|x| x[0] < side).count() as i32
}

fn main() {
    fn test(func: fn(points: Vec<Vec<i32>>, s: String) -> i32) {
        assert_eq!(func(vec![vec![-2, 4], vec![9, 3], vec![-9, 3]], String::from("cca")), 1);
        assert_eq!(func(vec![vec![2, 2], vec![-1, -2], vec![-4, 4], vec![-3, 1], vec![3, -3]], String::from("abdca")), 2);
        assert_eq!(func(vec![vec![1, 1], vec![-2, -2], vec![-2, 2]], String::from("abb")), 1);
        assert_eq!(func(vec![vec![1, 1], vec![-1, -1], vec![2, -2]], String::from("ccd")), 0);
    }
    test(max_points_inside_square);
    test(max_points_inside_square2);
}
