//! 四数之和

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

static mut MAP: Option<Box<HashMap<Vec<Vec<i32>>, i32>>> = None;

pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
    unsafe {
        if MAP == None {
            let (m, n) = (2, 3);
            let mut q = VecDeque::new();
            let board = vec![vec![1, 2, 3], vec![4, 5, 0]];
            q.push_back((board, (1, 2), 0));
            let dir = [-1, 0, 1, 0, -1];
            let mut map = Box::new(HashMap::new());
            while !q.is_empty() {
                let (mut board, (x, y), cur) = q.pop_front().unwrap();
                map.insert(board.clone(), cur);
                for i in 0..4 {
                    let nx = x + dir[i];
                    let ny = y + dir[i + 1];
                    if nx >= 0 && nx < m && ny >= 0 && ny < n {
                        let tmp = board[x as usize][y as usize];
                        board[x as usize][y as usize] = board[nx as usize][ny as usize];
                        board[nx as usize][ny as usize] = tmp;
                        if !map.contains_key(&board) {
                            q.push_back((board.clone(), (nx, ny), cur + 1));
                        }
                        let tmp = board[x as usize][y as usize];
                        board[x as usize][y as usize] = board[nx as usize][ny as usize];
                        board[nx as usize][ny as usize] = tmp;
                    }
                }
            }
            MAP = Some(map);
        }
        *MAP.as_ref().unwrap().get(&board).unwrap_or(&-1)
    }
}

static DIST: [[i32; 6]; 6] = [
    [0, 1, 2, 1, 2, 3],
    [1, 0, 1, 2, 1, 2],
    [2, 1, 0, 3, 2, 1],
    [1, 2, 3, 0, 1, 2],
    [2, 1, 2, 1, 0, 1],
    [3, 2, 1, 2, 1, 0],
];


#[derive(Eq, PartialEq)]
struct AStar {
    status: Vec<u8>,
    g: i32,
    h: i32,
    f: i32,
}

impl PartialOrd for AStar {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.f.partial_cmp(&self.f)
    }
}

impl Ord for AStar {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.cmp(&self.f)
    }
}

impl AStar {
    fn get_h(status: &[u8]) -> i32 {
        let mut result = 0;
        for i in 0..6 {
            if status[i] != 0 {
                result += DIST[i][status[i] as usize - 1];
            }
        }
        result
    }

    fn new(status: Vec<u8>, g: i32) -> Self {
        let h = AStar::get_h(&status);
        AStar { status, g, h, f: g + h }
    }
}

pub fn sliding_puzzle_a_star(board: Vec<Vec<i32>>) -> i32 {
    fn get(mut status: Vec<u8>) -> Vec<Vec<u8>> {
        let neighbors: [Vec<usize>; 6] = [vec![1, 3], vec![0, 2, 4], vec![1, 5], vec![0, 4], vec![1, 3, 5], vec![2, 4]];
        let mut result = vec![];
        let mut x = 0;
        for i in 0..status.len() {
            if status[i] == 0 {
                x = i;
                break;
            }
        }
        for &y in &neighbors[x] {
            status.swap(x, y);
            result.push(status.clone());
            status.swap(x, y);
        }
        result
    }

    let mut init = vec![];
    for i in 0..2 {
        for j in 0..3 {
            init.push(board[i][j] as u8);
        }
    }
    let result = vec![1, 2, 3, 4, 5, 0];
    if init == result {
        return 0;
    }
    let mut q = BinaryHeap::new();
    q.push(AStar::new(init, 0));
    let mut seen = HashSet::new();
    while !q.is_empty() {
        let node = q.pop().unwrap();
        for next in get(node.status) {
            if !seen.contains(&next) {
                if next == result {
                    return node.g + 1;
                }
                q.push(AStar::new(next.clone(), node.g + 1));
                seen.insert(next);
            }
        }
    }
    -1
}

fn main() {
    assert_eq!(sliding_puzzle_a_star(vec![vec![1, 2, 3], vec![4, 0, 5]]), 1);
    assert_eq!(sliding_puzzle_a_star(vec![vec![1, 2, 3], vec![5, 4, 0]]), -1);
    assert_eq!(sliding_puzzle_a_star(vec![vec![4, 1, 2], vec![5, 0, 3]]), 5);
    assert_eq!(sliding_puzzle_a_star(vec![vec![3, 2, 4], vec![1, 5, 0]]), 14);
}