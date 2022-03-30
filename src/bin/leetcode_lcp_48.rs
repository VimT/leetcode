//! LCP 48. 无限棋局

use std::collections::{HashMap, HashSet};

pub fn gobang(pieces: Vec<Vec<i32>>) -> String {
    let mut b = [HashMap::new(), HashMap::new(), HashMap::new(), HashMap::new()];// 黑棋的四类直线：水平, 垂直, 斜率为1, 斜率为-1
    let mut w = [HashMap::new(), HashMap::new(), HashMap::new(), HashMap::new()];// 白棋的四类直线：水平, 垂直, 斜率为1, 斜率为-1

    fn add_piece(p: &mut [HashMap<i32, Vec<i32>>; 4], x: i32, y: i32) {
        p[0].entry(y).or_default().push(x);
        p[1].entry(x).or_default().push(y);
        p[2].entry(y - x).or_default().push(x);
        p[3].entry(y + x).or_default().push(x);
    }

    fn get_pos(k: i32, v: i32, d: i32) -> (i32, i32) {
        return match d {
            0 => (v, k),
            1 => (k, v),
            2 => (v, k + v),
            3 => (v, k - v),
            _ => { panic!("should not be here") }
        };
    }

    fn has_piece(x: &[HashMap<i32, Vec<i32>>; 4], kvd: (i32, i32, usize)) -> bool {
        return x[kvd.2].contains_key(&kvd.0) && x[kvd.2].get(&kvd.0).unwrap().contains(&kvd.1);
    }

    // 找到P的棋子中，冲np(3或4)的点（填上这个点就必胜，并且没有Q的棋子阻挡）
    fn find_win_ps(p: &[HashMap<i32, Vec<i32>>; 4], q: &[HashMap<i32, Vec<i32>>; 4], np: usize) -> HashMap<(i32, i32), HashSet<(i32, i32)>> {
        let mut win_points: HashMap<(i32, i32), HashSet<(i32, i32)>> = HashMap::new();
        for d in 0..4 {
            for (&k, ps) in &p[d] {
                let n = ps.len();
                if n < np { continue; }
                for i in 0..n + 1 - np {
                    let dif = ps[i + np - 1] - ps[i];
                    if dif < 5 {
                        // <5 说明能成5
                        //找出空缺的v。找规律发现在[ps[i]-(4-dif), ps[i] + 5]，不是已有的v的点
                        let vs: Vec<i32> = (ps[i] + dif - 4..ps[i] + 5).filter(|&x| {
                            !ps[i..i + np].contains(&x) && !has_piece(q, (k, x, d))
                        }).collect();
                        let nvs = vs.len();
                        if nvs < 5 - np {
                            continue;
                        }
                        let dt = 4 - np;
                        for j in 0..nvs - dt {
                            let v1 = vs[j];
                            let v2 = vs[j + dt];
                            if v2 - v1 > 4 { continue; }
                            let xy1 = get_pos(k, v1, d as i32);
                            let xy2 = get_pos(k, v2, d as i32);
                            win_points.entry(xy1).or_default().insert(xy2);
                            win_points.entry(xy2).or_default().insert(xy1);
                            if np == 4 && win_points.len() > 1 {
                                return win_points;
                            }
                        }
                    }
                }
            }
        }
        win_points
    }

    for x in &pieces {
        if x[2] == 0 {
            add_piece(&mut b, x[0], x[1]);
        } else {
            add_piece(&mut w, x[0], x[1]);
        }
    }
    for i in 0..4 {
        for (_, v) in &mut b[i] {
            v.sort_unstable();
        }
        for (_, v) in &mut w[i] {
            v.sort_unstable();
        }
    }

    // 黑先手有4连黑子，并且有空位放下能成5连黑子，就黑胜
    if find_win_ps(&b, &w, 4).len() > 0 {
        return String::from("Black");
    }

    // 白如果有多个冲4就赢
    let live_w4 = find_win_ps(&w, &b, 4);
    if live_w4.len() > 1 {
        return String::from("White");
    }

    // 白只有1个冲4，黑先堵上；如果黑有多个冲4就赢，否则None
    if live_w4.len() == 1 {
        for (k, _) in live_w4.iter() {
            add_piece(&mut b, k.0, k.1);
            for i in 0..4 {
                for (_, v) in &mut b[i] {
                    v.sort_unstable();
                }
            }
            return if find_win_ps(&b, &w, 4).len() > 1 {
                String::from("Black")
            } else {
                String::from("None")
            };
        }
    }


    // 检查最开始如果有形成双4的点，就黑赢
    for (_, v) in &find_win_ps(&b, &w, 3) {
        if v.len() > 1 {
            return String::from("Black");
        }
    }
    return String::from("None");
}

fn main() {
    assert_eq!(gobang(vec![vec![1, 2, 1], vec![1, 4, 1], vec![1, 5, 1], vec![2, 1, 0], vec![2, 3, 0], vec![2, 4, 0], vec![3, 2, 1], vec![3, 4, 0], vec![4, 2, 1], vec![5, 2, 1]]), "Black".to_string());
}
