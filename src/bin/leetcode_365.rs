//! 水壶问题


use std::collections::HashSet;

pub fn can_measure_water(x: i32, y: i32, t: i32) -> bool {
    let mut s = vec![(0, 0)];
    let mut seen = HashSet::new();
    while !s.is_empty() {
        let (rx, ry) = s.pop().unwrap();
        if rx == t || ry == t || rx + ry == t {
            return true;
        }
        if seen.contains(&(rx, ry)) {
            continue;
        }
        seen.insert((rx, ry));
        s.push((x, ry));
        s.push((rx, y));
        s.push((0, ry));
        s.push((rx, 0));
        s.push(((rx - rx.min(y - ry)), (ry + rx.min(y - ry))));
        s.push(((rx + ry.min(x - rx)), (ry - ry.min(x - rx))));
    }
    false
}

/// 贝祖定理
pub fn can_measure_water_bz(mut x: i32, mut y: i32, t: i32) -> bool {
    if x + y < t { return false; }
    if x == 0 || y == 0 {
        return t == 0 || x + y == t;
    }
    if x < y { std::mem::swap(&mut x, &mut y); }
    while y != 0 {
        let tmp = x % y;
        x = y;
        y = tmp;
    }
    t % x == 0
}

fn main() {
    fn test(func: fn(jug1_capacity: i32, jug2_capacity: i32, target_capacity: i32) -> bool) {
        assert_eq!(func(3, 5, 4), true);
        assert_eq!(func(2, 6, 5), false);
        assert_eq!(func(1, 2, 3), true);
    }
    test(can_measure_water);
    test(can_measure_water_bz);
}
