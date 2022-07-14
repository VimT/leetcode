//! 迷宫 III

use std::cmp::Reverse;
use std::collections::BinaryHeap;

static DIR: [(u8, i32, i32); 4] = [(b'd', 1, 0), (b'l', 0, -1), (b'r', 0, 1), (b'u', -1, 0)];

fn dis(a: (usize, usize), b: (usize, usize)) -> i32 {
    return (a.0 as i32 - b.0 as i32).abs() + (a.1 as i32 - b.1 as i32).abs();
}

pub fn find_shortest_way(maze: Vec<Vec<i32>>, ball: Vec<i32>, hole: Vec<i32>) -> String {
    let m = maze.len();
    let n = maze[0].len();
    let mut q = BinaryHeap::new();
    let start = (ball[0] as usize, ball[1] as usize);
    let target = (hole[0] as usize, hole[1] as usize);
    let mut vis = vec![vec![false; n]; m];
    vis[start.0][start.1] = true;
    q.push(Reverse((dis(start, target), vec![], start.0, start.1, 0)));
    while !q.is_empty() {
        let Reverse((_, cur, x, y, step)) = q.pop().unwrap();
        if (x, y) == target {
            return unsafe { String::from_utf8_unchecked(cur) };
        }
        for (dir, dx, dy) in DIR {
            let mut nx = x as i32;
            let mut ny = y as i32;
            let mut next = cur.clone();
            next.push(dir);
            let mut cnt = 0;
            while (nx + dx) >= 0 && (nx + dx) < m as i32 && (ny + dy) >= 0 && (ny + dy) < n as i32 && maze[(nx + dx) as usize][(ny + dy) as usize] == 0 {
                nx += dx;
                ny += dy;
                cnt += 1;
                if (nx as usize, ny as usize) == target {
                    q.push(Reverse((step + cnt, next.clone(), nx as usize, ny as usize, step + cnt)))
                }
            }
            if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                let (nx, ny) = (nx as usize, ny as usize);
                if !vis[nx][ny] {
                    vis[nx][ny] = true;
                    q.push(Reverse((step + cnt + dis((nx, ny), target), next.clone(), nx, ny, step + cnt)));
                }
            }
        }
    }
    return String::from("impossible");
}

fn main() {
    fn test(func: fn(maze: Vec<Vec<i32>>, ball: Vec<i32>, hole: Vec<i32>) -> String) {
        assert_eq!(func(vec![vec![0, 0, 0, 0, 0], vec![1, 1, 0, 0, 1], vec![0, 0, 0, 0, 0], vec![0, 1, 0, 0, 1], vec![0, 1, 0, 0, 0]], vec![4, 3], vec![0, 1]), String::from("lul"));
        assert_eq!(func(vec![vec![0, 0, 0, 0, 0], vec![1, 1, 0, 0, 1], vec![0, 0, 0, 0, 0], vec![0, 1, 0, 0, 1], vec![0, 1, 0, 0, 0]], vec![4, 3], vec![3, 0]), String::from("impossible"));
        assert_eq!(func(vec![vec![0, 0, 0, 0, 0, 0, 0], vec![0, 0, 1, 0, 0, 1, 0], vec![0, 0, 0, 0, 1, 0, 0], vec![0, 0, 0, 0, 0, 0, 1]], vec![0, 4], vec![3, 5]), String::from("dldr"));
    }
    test(find_shortest_way);
}
