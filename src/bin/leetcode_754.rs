//! 到达终点数字

pub fn reach_number(target: i32) -> i32 {
    let target = target.abs() as i64;
    let mut n = ((((1 + 8 * target) as f64).sqrt() - 1.) / 2.) as i64;
    let mut cur_sum = (n * n + n) / 2;
    if cur_sum < target {
        n += 1;
        cur_sum += n;
    }
    loop {
        let diff = cur_sum - target;
        if diff & 1 == 0 {
            return n as i32;
        }
        n += 1;
        cur_sum += n;
    }
}

fn main() {
    assert_eq!(reach_number(9999), 141);
    assert_eq!(reach_number(5), 5);
    assert_eq!(reach_number(2), 3);
    assert_eq!(reach_number(3), 2);
    assert_eq!(reach_number(1), 1);
    assert_eq!(reach_number(4), 3);
}
