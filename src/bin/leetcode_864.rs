//! 获取所有钥匙的最短路径

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

use leetcode::svec;

/// 把所有关键点找出来组成图，然后dij
pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
    let grid: Vec<Vec<u8>> = grid.into_iter().map(|x| x.into_bytes()).collect();
    const DIR: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let m = grid.len();
    let n = grid[0].len();
    fn dist_from(grid: &Vec<Vec<u8>>, start: (usize, usize)) -> Vec<(u8, i32)> {
        let m = grid.len();
        let n = grid[0].len();
        let mut vis = vec![vec![false; n]; m];
        vis[start.0][start.1] = true;
        let mut step = 1;
        let mut q = vec![start];
        let mut result = vec![];
        while !q.is_empty() {
            let mut nq = Vec::with_capacity(q.len() * 4);
            for (x, y) in q {
                if (x, y) != start && grid[x][y] != b'.' {
                    continue;
                }
                for (dx, dy) in DIR {
                    let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                    if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                        let (nx, ny) = (nx as usize, ny as usize);
                        if !vis[nx][ny] && grid[nx][ny] != b'#' {
                            if grid[nx][ny] != b'.' {
                                result.push((grid[nx][ny], step));
                            }
                            vis[nx][ny] = true;
                            nq.push((nx, ny));
                        }
                    }
                }
            }
            step += 1;
            q = nq;
        }
        result
    }
    let mut nodes = HashMap::new();
    let mut edges = vec![vec![]; 128];
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] != b'.' && grid[i][j] != b'#' {
                nodes.insert(grid[i][j] as usize, (i, j));
                edges[grid[i][j] as usize] = dist_from(&grid, (i, j));
            }
        }
    }
    let key_num = nodes.len() / 2;
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, b'@', 0)));
    let mut dist = HashMap::new();
    let finial_state = (1 << key_num) - 1;
    while !heap.is_empty() {
        let Reverse((dis, node, state)) = heap.pop().unwrap();
        if *dist.get(&(node, state)).unwrap_or(&i32::MAX) < dis { continue; }
        if state == finial_state {
            return dis;
        }
        for &(nxt_node, nxt_dis) in &edges[node as usize] {
            let mut new_state = state;
            if nxt_node.is_ascii_lowercase() {
                new_state |= 1 << (nxt_node - b'a');
            }
            if nxt_node.is_ascii_uppercase() {
                if new_state >> (nxt_node - b'A') & 1 == 0 {
                    continue;
                }
            }
            if *dist.get(&(nxt_node, new_state)).unwrap_or(&i32::MAX) > dis + nxt_dis {
                dist.insert((nxt_node, new_state), dis + nxt_dis);
                heap.push(Reverse((dis + nxt_dis, nxt_node, new_state)));
            }
        }
    }

    -1
}

fn main() {
    assert_eq!(shortest_path_all_keys(svec!["@...a", ".###A", "b.BCc"]), 10);
    assert_eq!(shortest_path_all_keys(svec![".#.#..#.b...............#.#..#", ".#..##.........#......d.......", "..#...e.#.##....##.....#.....#", "..#..#.#.#.##..........#.....#", "...#...##....#.....#..........", "#........###....#..#.........f", "...............#......#...#...", "..........##.#...#.E..#......#", ".#...##...#.##.D....##..#.....", ".......#...........#....#..##.", "...#..........##.....#.......#", ".F#....#......#...............", "..##.#.#.....#..##...#.#.....#", ".............##..##..#.#......", "#..@..#.#.......#..........#..", ".........##..................#", ".#.......##...##..#.......#...", ".......#.#...A.a......#.##.#..", ".......#......##..#.###.#.....", ".##.#....##...#.#.....#.#.....", ".#.....#.c..#.....#......#..##", "##.....##........B.#.......#.#", ".....#...#....#..##...........", "#.#.##.#....#.#...............", ".#.#..#.####............#.....", "#.#..........###.#........#...", "..#..#.........#.......#..#.##", "..#..#C#...............#......", ".........#.##.##......#.#.....", "..#........##.#..##.#.....#.#."]), 80);
    assert_eq!(shortest_path_all_keys(svec![".#.b.", "A.#aB", "#d...", "@.cC.", "D...#"]), 8);
    assert_eq!(shortest_path_all_keys(svec!["@.a.#", "###.#", "b.A.B"]), 8);
    assert_eq!(shortest_path_all_keys(svec!["@..aA", "..B#.", "....b"]), 6);
    assert_eq!(shortest_path_all_keys(svec!["@Aa"]), -1);
}
