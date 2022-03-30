//! 两地调度

pub fn two_city_sched_cost(mut costs: Vec<Vec<i32>>) -> i32 {
    costs.sort_unstable_by_key(|x| -(x[0] - x[1]).abs());
    let len = costs.len();
    let half = len / 2;
    let mut result = 0;
    let mut a = 0;
    let mut b = 0;
    while a < half && b < half {
        if costs[a + b][0] > costs[a + b][1] {
            result += costs[a + b][1];
            b += 1;
        } else {
            result += costs[a + b][0];
            a += 1;
        }
    }
    while a < half {
        result += costs[a + b][0];
        a += 1;
    }
    while b < half {
        result += costs[a + b][1];
        b += 1;
    }
    result
}

fn main() {
    assert_eq!(two_city_sched_cost(vec![vec![10, 20], vec![30, 200], vec![400, 50], vec![30, 20]]), 110);
    assert_eq!(two_city_sched_cost(vec![vec![259, 770], vec![448, 54], vec![926, 667], vec![184, 139], vec![840, 118], vec![577, 469]]), 1859);
    assert_eq!(two_city_sched_cost(vec![vec![515, 563], vec![451, 713], vec![537, 709], vec![343, 819], vec![855, 779], vec![457, 60], vec![650, 359], vec![631, 42]]), 3086);
}
