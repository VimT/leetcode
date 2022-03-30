//! 猫和老鼠

use Status::*;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Status {
    Unknown,
    Mouse,
    Cat,
    Draw,
}

impl Status {
    fn to_i32(&self) -> i32 {
        match self {
            Status::Unknown => -1,
            Status::Mouse => 1,
            Status::Cat => 2,
            Status::Draw => 0,
        }
    }
}

pub fn cat_mouse_game(graph: Vec<Vec<i32>>) -> i32 {
    let n = graph.len();
    let mut dp = vec![vec![vec![Unknown; n * 2]; n]; n];

    fn get_result(graph: &Vec<Vec<i32>>, dp: &mut Vec<Vec<Vec<Status>>>, mouse: usize, cat: usize, turn: usize) -> Status {
        if turn == 2 * dp.len() {
            return Draw;
        }
        if dp[mouse][cat][turn] != Unknown {
            return dp[mouse][cat][turn];
        }
        if mouse == 0 {
            dp[mouse][cat][turn] = Mouse;
        } else if cat == mouse {
            dp[mouse][cat][turn] = Cat;
        } else {
            let cur_move = if turn & 1 == 0 { mouse } else { cat };
            let default_result = if cur_move == mouse { Cat } else { Mouse };
            let mut result = default_result;
            for &next in &graph[cur_move] {
                let next = next as usize;
                if cur_move == cat && next == 0 {
                    continue;
                }
                let next_mouse = if cur_move == mouse { next } else { mouse };
                let next_cat = if cur_move == cat { next } else { cat };
                let next_result = get_result(graph, dp, next_mouse, next_cat, turn + 1);
                if next_result != default_result {
                    result = next_result;
                    if result != Draw {
                        break;
                    }
                }
            }
            dp[mouse][cat][turn] = result;
        }
        dp[mouse][cat][turn]
    }

    get_result(&graph, &mut dp, 1, 2, 0).to_i32()
}

fn main() {
    assert_eq!(cat_mouse_game(vec![vec![2, 5], vec![3], vec![0, 4, 5], vec![1, 4, 5], vec![2, 3], vec![0, 2, 3]]), 0);
    assert_eq!(cat_mouse_game(vec![vec![1, 3], vec![0], vec![3], vec![0, 2]]), 1);
}
