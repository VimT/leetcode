//! 吃掉所有兵需要的最多移动次数

use std::collections::VecDeque;

pub fn max_moves(kx: i32, ky: i32, positions: Vec<Vec<i32>>) -> i32 {
    let mut pos: Vec<(usize, usize)> = positions.into_iter().map(|v| (v[0] as usize, v[1] as usize)).collect();
    pos.push((kx as usize, ky as usize));

    const SIZE: usize = 50;

    // 从 (tx, ty) 马走到所有位置的最短距离
    fn bfs(tx: usize, ty: usize) -> Vec<Vec<i32>> {
        let mut q = VecDeque::new();
        let mut dis = vec![vec![-1; SIZE]; SIZE];
        static DIR: [(i32, i32); 8] = [(-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1)];
        dis[tx][ty] = 0;
        q.push_back((tx, ty));
        while !q.is_empty() {
            let (x, y) = q.pop_front().unwrap();
            for &(dx, dy) in DIR.iter() {
                let nx = (x as i32 + dx) as usize;
                let ny = (y as i32 + dy) as usize;
                if nx < SIZE && ny < SIZE && dis[nx][ny] == -1 {
                    dis[nx][ny] = dis[x][y] + 1;
                    q.push_back((nx, ny));
                }
            }
        }
        dis
    }

    let len = pos.len();
    let dis = pos.iter().copied().map(|(x, y)| bfs(x, y)).collect();

    // 当前在 pawns[last] 位置， 已经吃掉了 e 状态兵，返回 最小/最大 移动次数
    fn dfs(pos: &Vec<(usize, usize)>, dis: &Vec<Vec<Vec<i32>>>, last: usize, e: i32, mem: &mut Vec<Vec<i32>>) -> i32 {
        if e == (1 << pos.len()) - 1 { return 0; }
        if mem[last][e as usize] != -1 { return mem[last][e as usize]; }

        let (x, y) = pos[last];
        let is_alice = e.count_ones() & 1 == 1;
        let mut result = if is_alice { i32::MIN } else { i32::MAX };
        for i in 0..pos.len() {
            if e >> i & 1 == 1 { continue; }
            let m = dis[i][x][y] + dfs(pos, dis, i, e | 1 << i, mem);
            result = if is_alice { result.max(m) } else { result.min(m) }
        }

        mem[last][e as usize] = result;
        result
    }

    let mut mem = vec![vec![-1; 1 << pos.len()]; len];
    dfs(&pos, &dis, len - 1, 1 << (len - 1), &mut mem)
}

fn main() {
    fn test(func: fn(kx: i32, ky: i32, positions: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(0, 0, vec![vec![6, 9], vec![2, 8], vec![0, 10]]), 12);
        assert_eq!(func(1, 1, vec![vec![0, 0]]), 4);
        assert_eq!(func(0, 2, vec![vec![1, 1], vec![2, 2], vec![3, 3]]), 8);
        assert_eq!(func(0, 0, vec![vec![1, 2], vec![2, 4]]), 3);
    }
    test(max_moves);
}
