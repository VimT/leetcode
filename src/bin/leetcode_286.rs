//! 墙与门

use std::collections::VecDeque;

pub fn walls_and_gates(rooms: &mut Vec<Vec<i32>>) {
    static DIR: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let m = rooms.len();
    let n = rooms[0].len();
    let mut q = VecDeque::new();
    for i in 0..m {
        for j in 0..n {
            if rooms[i][j] == 0 {
                q.push_back((i, j, 0));
            }
        }
    }
    while !q.is_empty() {
        let (x, y, dis) = q.pop_front().unwrap();
        for (dx, dy) in DIR {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                let (nx, ny) = (nx as usize, ny as usize);
                if rooms[nx][ny] == i32::MAX {
                    rooms[nx][ny] = dis + 1;
                    q.push_back((nx, ny, dis + 1));
                }
            }
        }
    }
}

fn main() {
    fn test(func: fn(rooms: &mut Vec<Vec<i32>>)) {
        let help = |mut rooms: Vec<Vec<i32>>| {
            func(&mut rooms);
            rooms
        };
        assert_eq!(help(vec![vec![i32::MAX, -1, 0, i32::MAX], vec![i32::MAX, i32::MAX, i32::MAX, -1], vec![i32::MAX, -1, i32::MAX, -1], vec![0, -1, i32::MAX, i32::MAX]]), vec![vec![3, -1, 0, 1], vec![2, 2, 1, -1], vec![1, -1, 2, -1], vec![0, -1, 3, 4]]);
        assert_eq!(help(vec![vec![-1]]), vec![vec![-1]]);
    }
    test(walls_and_gates);
}
