//! 网格照明

use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;

pub fn grid_illumination(n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result = Vec::with_capacity(queries.len());
    let mut xm: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut ym: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut zxm: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut yxm: HashMap<i32, HashSet<i32>> = HashMap::new();
    for lamp in lamps {
        let (x, y) = (lamp[0], lamp[1]);
        xm.entry(x).or_default().insert(y);
        ym.entry(y).or_default().insert(x);
        zxm.entry(x - y).or_default().insert(x);
        yxm.entry(x + y).or_default().insert(x);
    }
    for query in queries {
        let (x, y) = (query[0], query[1]);
        let has = xm.contains_key(&x) || ym.contains_key(&y) || zxm.contains_key(&(x - y)) || yxm.contains_key(&(x + y));
        result.push(has as i32);
        if has {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    let (nx, ny) = (x + dx, y + dy);
                    if nx >= 0 && nx < n && ny >= 0 && ny < n {
                        if let Entry::Occupied(mut v) = xm.entry(nx) {
                            v.get_mut().remove(&ny);
                            if v.get().is_empty() {
                                v.remove();
                            }
                        }
                        if let Entry::Occupied(mut v) = ym.entry(ny) {
                            v.get_mut().remove(&nx);
                            if v.get().is_empty() {
                                v.remove();
                            }
                        }
                        if let Entry::Occupied(mut v) = zxm.entry(nx - ny) {
                            v.get_mut().remove(&nx);
                            if v.get().is_empty() {
                                v.remove();
                            }
                        }
                        if let Entry::Occupied(mut v) = yxm.entry(nx + ny) {
                            v.get_mut().remove(&nx);
                            if v.get().is_empty() {
                                v.remove();
                            }
                        }
                    }
                }
            }
        }
    }
    result
}

fn main() {
    assert_eq!(grid_illumination(5, vec![vec![0, 0], vec![4, 4]], vec![vec![1, 1], vec![1, 0]]), vec![1, 0]);
    assert_eq!(grid_illumination(5, vec![vec![0, 0], vec![4, 4]], vec![vec![1, 1], vec![1, 1]]), vec![1, 1]);
    assert_eq!(grid_illumination(5, vec![vec![0, 0], vec![0, 4]], vec![vec![0, 4], vec![0, 1], vec![1, 4]]), vec![1, 1, 0]);
}
