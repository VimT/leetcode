//! 模拟行走机器人

use std::collections::HashMap;

use Dir::*;

#[derive(Copy, Clone)]
enum Dir {
    N,
    W,
    S,
    E,
}

impl Dir {
    fn turn_left(self) -> Dir {
        match self {
            N => W,
            W => S,
            S => E,
            E => N,
        }
    }

    fn turn_right(self) -> Dir {
        match self {
            N => E,
            W => N,
            S => W,
            E => S,
        }
    }
}

pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
    let mut obx = HashMap::new();
    let mut oby = HashMap::new();
    for ob in obstacles {
        obx.entry(ob[0]).or_insert(vec![]).push(ob[1]);
        oby.entry(ob[1]).or_insert(vec![]).push(ob[0]);
    }
    for (_, val) in &mut obx {
        val.sort_unstable();
    }
    for (_, val) in &mut oby {
        val.sort_unstable();
    }
    let (mut x, mut y) = (0, 0);
    let mut result = 0;
    let mut dir = N;
    for command in commands {
        match command {
            -1 => dir = dir.turn_right(),
            -2 => dir = dir.turn_left(),
            _ => {
                match dir {
                    N => {
                        let mut ty = y + command;
                        if let Some(val) = obx.get(&x) {
                            let ka = val.binary_search(&y).unwrap_or_else(|x| x);
                            if ka < val.len() && val[ka] > y && val[ka] <= ty {
                                ty = val[ka] - 1;
                            }
                        }
                        y = ty;
                    }
                    W => {
                        let mut tx = x - command;
                        if let Some(val) = oby.get(&y) {
                            let ka = val.binary_search(&x).unwrap_or_else(|x| x);
                            if ka < val.len() + 1 && ka > 0 && val[ka - 1] < x && val[ka - 1] >= tx {
                                tx = val[ka - 1] + 1;
                            }
                        }
                        x = tx;
                    }
                    S => {
                        let mut ty = y - command;
                        if let Some(val) = obx.get(&x) {
                            let ka = val.binary_search(&y).unwrap_or_else(|x| x);
                            if ka < val.len() + 1 && ka > 0 && val[ka - 1] < y && val[ka - 1] >= ty {
                                ty = val[ka - 1] + 1;
                            }
                        }
                        y = ty;
                    }
                    E => {
                        let mut tx = x + command;
                        if let Some(val) = oby.get(&y) {
                            let ka = val.binary_search(&x).unwrap_or_else(|x| x);
                            if ka < val.len() && val[ka] > x && val[ka] <= tx {
                                tx = val[ka] - 1;
                            }
                        }
                        x = tx;
                    }
                }
                result = result.max(x * x + y * y);
            }
        }
    }
    result
}


fn main() {
    assert_eq!(robot_sim(vec![2, 2, 5, -1, -1], vec![vec![-3, 5], vec![-2, 5], vec![3, 2], vec![5, 0], vec![-2, 0], vec![-1, 5], vec![5, -3], vec![0, 0], vec![-4, 4], vec![-3, 4]]), 81);
    assert_eq!(robot_sim(vec![4, -1, 4, -2, 4], vec![vec![2, 4]]), 65);
    assert_eq!(robot_sim(vec![4, -1, 3], vec![]), 25);
    assert_eq!(robot_sim(vec![6, -1, -1, 6], vec![]), 36);
}
