//! 设置时间的最少代价

pub fn min_cost_set_time(start_at: i32, move_cost: i32, push_cost: i32, target_seconds: i32) -> i32 {
    let mut result = i32::MAX;
    for minutes in 0..=(target_seconds / 60).min(99) {
        let second = target_seconds - 60 * minutes;
        if second >= 0 && second <= 99 {
            let mut cost = 0;
            let input = [minutes / 10, minutes % 10, second / 10, second % 10];
            let mut i = 0;
            while i < 4 && input[i] == 0 {
                i += 1;
            }
            let mut cur = start_at;
            while i < 4 {
                if input[i] != cur {
                    cost += move_cost;
                    cur = input[i];
                }
                cost += push_cost;
                i += 1;
            }
            result = result.min(cost);
        }
    }
    result
}

fn main() {
    assert_eq!(min_cost_set_time(7, 220, 479, 6000), 2576);
    assert_eq!(min_cost_set_time(1, 2, 1, 600), 6);
    assert_eq!(min_cost_set_time(0, 1, 2, 76), 6);
}
