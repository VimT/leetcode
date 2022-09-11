//! 可以攻击国王的皇后

use leetcode::unorder;

pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
    let (x, y) = (king[0] as usize, king[1] as usize);
    let mut qmap = vec![vec![false; 8]; 8];
    for queue in queens {
        qmap[queue[0] as usize][queue[1] as usize] = true;
    }
    let mut result = vec![];
    for i in x + 1..8 {
        if qmap[i][y] {
            result.push(vec![i as i32, y as i32]);
            break;
        }
    }
    for i in (0..x).rev() {
        if qmap[i][y] {
            result.push(vec![i as i32, y as i32]);
            break;
        }
    }
    for j in y + 1..8 {
        if qmap[x][j] {
            result.push(vec![x as i32, j as i32]);
            break;
        }
    }
    for j in (0..y).rev() {
        if qmap[x][j] {
            result.push(vec![x as i32, j as i32]);
            break;
        }
    }
    for d in 1..=(7 - x).min(7 - y) {
        if qmap[x + d][y + d] {
            result.push(vec![(x + d) as i32, (y + d) as i32]);
            break;
        }
    }
    for d in 1..=x.min(y) {
        if qmap[x - d][y - d] {
            result.push(vec![(x - d) as i32, (y - d) as i32]);
            break;
        }
    }
    for d in 1..=(7 - x).min(y) {
        if qmap[x + d][y - d] {
            result.push(vec![(x + d) as i32, (y - d) as i32]);
            break;
        }
    }
    for d in 1..=x.min(7 - y) {
        if qmap[x - d][y + d] {
            result.push(vec![(x - d) as i32, (y + d) as i32]);
            break;
        }
    }
    result
}

fn main() {
    fn test(func: fn(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>>) {
        assert_eq!(unorder(func(vec![vec![4,7],vec![7,7],vec![0,7],vec![5,1],vec![0,3],vec![4,0],vec![6,7],vec![2,2],vec![0,4],vec![6,4],vec![6,5],vec![3,5],vec![4,6],vec![6,1],vec![3,1],vec![0,6],vec![2,0],vec![4,3],vec![1,7],vec![5,2]], vec![2,4])), [[0, 4], [0, 6], [2, 2], [3, 5], [5, 1], [6, 4]]);
        assert_eq!(unorder(func(vec![vec![0, 1], vec![1, 0], vec![4, 0], vec![0, 4], vec![3, 3], vec![2, 4]], vec![0, 0])), vec![vec![0, 1], vec![1, 0], vec![3, 3]]);
        assert_eq!(unorder(func(vec![vec![0, 0], vec![1, 1], vec![2, 2], vec![3, 4], vec![3, 5], vec![4, 4], vec![4, 5]], vec![3, 3])), vec![vec![2, 2], vec![3, 4], vec![4, 4]]);
        assert_eq!(unorder(func(vec![vec![5, 6], vec![7, 7], vec![2, 1], vec![0, 7], vec![1, 6], vec![5, 1], vec![3, 7], vec![0, 3], vec![4, 0], vec![1, 2], vec![6, 3], vec![5, 0], vec![0, 4], vec![2, 2], vec![1, 1], vec![6, 4], vec![5, 4], vec![0, 0], vec![2, 6], vec![4, 5], vec![5, 2], vec![1, 4], vec![7, 5], vec![2, 3], vec![0, 5], vec![4, 2], vec![1, 0], vec![2, 7], vec![0, 1], vec![4, 6], vec![6, 1], vec![0, 6], vec![4, 3], vec![1, 7]], vec![3, 4])), [[1, 4], [1, 6], [2, 3], [3, 7], [4, 3], [4, 5], [5, 4]]);
    }
    test(queens_attackthe_king);
}
