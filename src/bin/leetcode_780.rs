//! 到达终点

pub fn reaching_points(sx: i32, sy: i32, mut tx: i32, mut ty: i32) -> bool {
    while tx >= sx && ty >= sy {
        if ty == tx { break; }
        if tx > ty {
            if ty > sy { tx = tx % ty; } else { return (tx - sx) % ty == 0; }
        } else {
            if tx > sx { ty = ty % tx; } else { return (ty - sy) % tx == 0; }
        }
    }
    return sx == tx && sy == ty;
}

fn main() {
    assert!(reaching_points(1, 1, 3, 5));
    assert!(!reaching_points(1, 1, 2, 2));
    assert!(reaching_points(1, 1, 1, 1));
}
