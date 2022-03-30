//! 由斜杠划分区域

use std::collections::VecDeque;

use leetcode::svec;

#[derive(Eq, PartialEq, Copy, Clone)]
enum Pos {
    Whole,
    LeftTop,
    RightTop,
    LeftBottom,
    RightBottom,
}

impl Pos {
    fn top(&self) -> bool {
        return self == &Pos::Whole || self == &Pos::LeftTop || self == &Pos::RightTop;
    }

    fn left(&self) -> bool {
        return self == &Pos::Whole || self == &Pos::LeftTop || self == &Pos::LeftBottom;
    }

    fn right(&self) -> bool {
        return self == &Pos::Whole || self == &Pos::RightBottom || self == &Pos::RightTop;
    }

    fn bottom(&self) -> bool {
        return self == &Pos::Whole || self == &Pos::LeftBottom || self == &Pos::RightBottom;
    }
}

pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
    let len = grid.len();
    let mut gg = Vec::with_capacity(len * len * 2);
    let mut gm: Vec<Vec<Vec<usize>>> = vec![vec![vec![]; len]; len];
    for (i, line) in grid.iter().enumerate() {
        for (j, char) in line.as_bytes().iter().enumerate() {
            match *char {
                b'/' => {
                    gg.push((i, j, Pos::LeftTop));
                    gm[i][j].push(gg.len() - 1);
                    gg.push((i, j, Pos::RightBottom));
                    gm[i][j].push(gg.len() - 1);
                }
                b'\\' => {
                    gg.push((i, j, Pos::RightTop));
                    gm[i][j].push(gg.len() - 1);
                    gg.push((i, j, Pos::LeftBottom));
                    gm[i][j].push(gg.len() - 1);
                }
                b' ' => {
                    gg.push((i, j, Pos::Whole));
                    gm[i][j].push(gg.len() - 1);
                }
                _ => panic!("unknown")
            }
        }
    }
    let mut visited = vec![false; gg.len()];
    let mut ans = 0;
    let mut queue = VecDeque::new();
    for start in 0..gg.len() {
        if !visited[start] {
            queue.push_back(start);
            while !queue.is_empty() {
                let node = queue.pop_front().unwrap();
                let &(i, j, pos) = &gg[node];
                visited[node] = true;
                if j > 0 && pos.top() {
                    for &next in &gm[i][j - 1] { if !visited[next] && gg[next].2.bottom() { queue.push_back(next); } }
                }
                if i > 0 && pos.left() {
                    for &next in &gm[i - 1][j] { if !visited[next] && gg[next].2.right() { queue.push_back(next); } }
                }
                if i < len - 1 && pos.right() {
                    for &next in &gm[i + 1][j] { if !visited[next] && gg[next].2.left() { queue.push_back(next); } }
                }
                if j < len - 1 && pos.bottom() {
                    for &next in &gm[i][j + 1] { if !visited[next] && gg[next].2.top() { queue.push_back(next); } }
                }
            }
            ans += 1;
        }
    }

    ans
}

fn main() {
    assert_eq!(regions_by_slashes(svec![" /","/ "]), 2);
    assert_eq!(regions_by_slashes(svec![" /","  "]), 1);
    assert_eq!(regions_by_slashes(svec!["/\\","\\/"]), 5);
}
