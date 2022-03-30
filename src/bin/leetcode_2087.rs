//! 网格图中机器人回家的最小代价

pub fn min_cost(start_pos: Vec<i32>, home_pos: Vec<i32>, row_costs: Vec<i32>, col_costs: Vec<i32>) -> i32 {
    let mut result = 0;
    let (sx, sy) = (start_pos[0] as usize, start_pos[1] as usize);
    let (ex, ey) = (home_pos[0] as usize, home_pos[1] as usize);
    if sx <= ex {
        for i in sx + 1..=ex {
            result += row_costs[i];
        }
    } else {
        for i in ex..=sx - 1 {
            result += row_costs[i];
        }
    }
    if sy <= ey {
        for i in sy + 1..=ey {
            result += col_costs[i];
        }
    } else {
        for i in ey..=sy - 1 {
            result += col_costs[i];
        }
    }
    result
}

fn main() {
    assert_eq!(min_cost(vec![6, 3],
                        vec![3, 3],
                        vec![6, 3, 4, 4, 10, 2, 14, 21],
                        vec![7, 2, 3, 3, 15]), 16);
    assert_eq!(min_cost(vec![1, 0], vec![2, 3], vec![5, 4, 3], vec![8, 2, 6, 7]), 18);
    assert_eq!(min_cost(vec![0, 0], vec![0, 0], vec![5], vec![26]), 0);
}
