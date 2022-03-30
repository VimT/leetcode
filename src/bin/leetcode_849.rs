//! 到最近的人的最大距离

pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
    let len = seats.len();
    let mut result = 0;
    let mut i = 0;
    while i < len && seats[i] == 0 { i += 1; }
    result = result.max(i);
    while i < len {
        let mut ni = i + 1;
        while ni < len && seats[ni] == 0 { ni += 1; }
        if ni == len {
            result = result.max(len - 1 - i);
        } else {
            result = result.max((ni - i) / 2);
        }
        i = ni;
    }
    result as i32
}

fn main() {
    assert_eq!(max_dist_to_closest(vec![1, 0, 0, 0, 1, 0, 1]), 2);
    assert_eq!(max_dist_to_closest(vec![1, 0, 0, 0]), 3);
    assert_eq!(max_dist_to_closest(vec![0, 1]), 1);
    assert_eq!(max_dist_to_closest(vec![1, 0]), 1);
}
