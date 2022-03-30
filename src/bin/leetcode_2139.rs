//! 得到目标值的最少行动次数

pub fn min_moves(mut target: i32, mut max_doubles: i32) -> i32 {
    let mut result = 0;
    while target > 1 {
        if max_doubles == 0 {
            result += target - 1;
            break;
        }
        if target & 1 == 1 {
            result += 1;
            target -= 1;
        } else {
            result += 1;
            max_doubles -= 1;
            target >>= 1;
        }
    }
    result
}

fn main() {
    assert_eq!(min_moves(5, 0), 4);
    assert_eq!(min_moves(19, 2), 7);
    assert_eq!(min_moves(10, 4), 4);
}
