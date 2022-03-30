//! 矩形面积 II

use std::collections::{BTreeMap, BTreeSet, HashMap};

/// 离散化
pub fn rectangle_area(rectangles: Vec<Vec<i32>>) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    let mut x: BTreeSet<i32> = BTreeSet::new();
    let mut y = BTreeSet::new();
    for rec in &rectangles {
        x.insert(rec[0]);
        y.insert(rec[1]);
        x.insert(rec[2]);
        y.insert(rec[3]);
    }
    let x_list: Vec<i32> = x.into_iter().collect();
    let y_list: Vec<i32> = y.into_iter().collect();
    let x_map: HashMap<i32, usize> = x_list.iter().enumerate().map(|x| (*x.1, x.0)).collect();
    let y_map: HashMap<i32, usize> = y_list.iter().enumerate().map(|x| (*x.1, x.0)).collect();
    let m = x_list.len();
    let n = y_list.len();
    let mut grid = vec![vec![false; n]; m];
    for rec in &rectangles {
        for i in x_map[&rec[0]]..x_map[&rec[2]] {
            for j in y_map[&rec[1]]..y_map[&rec[3]] {
                grid[i][j] = true;
            }
        }
    }
    let mut result = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] {
                result += (y_list[j + 1] as i64 - y_list[j] as i64) * (x_list[i + 1] as i64 - x_list[i] as i64);
            }
        }
    }
    (result % MOD) as i32
}


/// 扫描线 可以通过线段树+离散化 进一步优化query查询
pub fn rectangle_area_smx(rectangles: Vec<Vec<i32>>) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    const OPEN: u8 = 0;
    const CLOSE: u8 = 1;
    let len = rectangles.len();
    let mut events = Vec::with_capacity(len * 2);
    for rec in rectangles {
        events.push((rec[1], OPEN, rec[0], rec[2]));
        events.push((rec[3], CLOSE, rec[0], rec[2]));
    }
    events.sort_unstable();
    let mut active: BTreeMap<(i32, i32), i32> = BTreeMap::new();
    let mut cur_y = events[0].0;
    let mut result = 0;
    for (y, typ, x1, x2) in events {
        let mut query = 0;
        let mut cur = -1;
        // 计算这一行有多少个之前的入边
        for &(px1, px2) in active.keys() {
            cur = cur.max(px1);
            query += 0.max(px2 - cur) as i64;
            cur = cur.max(px2);
        }
        result += query * (y - cur_y) as i64;
        if typ == OPEN {
            *active.entry((x1, x2)).or_default() += 1;
        } else {
            if let Some(cnt) = active.get_mut(&(x1, x2)) {
                *cnt -= 1;
                if *cnt == 0 {
                    active.remove(&(x1, x2));
                }
            }
        }
        cur_y = y;
    }
    (result % MOD) as i32
}


fn main() {
    assert_eq!(rectangle_area_smx(vec![vec![11, 4, 22, 74], vec![11, 33, 22, 85], vec![28, 12, 59, 15], vec![61, 38, 100, 41], vec![27, 27, 93, 93], vec![18, 32, 80, 43]]), 5416);
    assert_eq!(rectangle_area_smx(vec![vec![0, 0, 2, 2], vec![1, 0, 2, 3], vec![1, 0, 3, 1]]), 6);
    assert_eq!(rectangle_area_smx(vec![vec![0, 0, 1000000000, 1000000000]]), 49);
}
