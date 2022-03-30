//! 路径交叉

pub fn is_self_crossing(distance: Vec<i32>) -> bool {
    let len = distance.len();
    if len < 4 {
        return false;
    }
    let cross3 = |i| distance[i] >= distance[i - 2] &&
        distance[i - 1] <= distance[i - 3];
    let cross4 = |i| distance[i] + distance[i - 4] >= distance[i - 2] &&
        distance[i - 1] == distance[i - 3];
    let cross5 = |i| distance[i] + distance[i - 4] >= distance[i - 2] &&
        distance[i - 1] <= distance[i - 3] &&
        distance[i - 1] + distance[i - 5] >= distance[i - 3] &&
        distance[i - 2] >= distance[i - 4];
    if cross3(3) {
        return true;
    }
    if len >= 5 {
        if cross3(4) || cross4(4) {
            return true;
        }
    }
    if len >= 6 {
        for i in 5..len {
            if cross3(i) || cross4(i) || cross5(i) {
                return true;
            }
        }
    }

    return false;
}

fn main() {
    assert!(is_self_crossing(vec![2, 1, 1, 2]));
    assert!(!is_self_crossing(vec![1, 2, 3, 4]));
    assert!(is_self_crossing(vec![1, 1, 1, 1]));
}
