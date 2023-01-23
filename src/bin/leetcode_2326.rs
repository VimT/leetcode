//! 螺旋矩阵 IV

use leetcode::link;
use leetcode::linknode::ListNode;

pub fn spiral_matrix(m: i32, n: i32, head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
    static DIR: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // 右下左上
    let m = m as usize;
    let n = n as usize;
    let mut result = vec![vec![-1; n]; m];
    let mut x = 0;
    let mut y = 0;
    let mut di = 0;
    let mut p = &head;
    while p.is_some() {
        result[x as usize][y as usize] = p.as_ref().unwrap().val;
        let (nx, ny) = (x + DIR[di].0, y + DIR[di].1);
        if nx < 0 || nx >= m as i32 || ny < 0 || ny >= n as i32 || result[nx as usize][ny as usize] != -1 {
            di = (di + 1) & 3;
        }
        x += DIR[di].0;
        y += DIR[di].1;
        p = &p.as_ref().unwrap().next;
    }
    result
}

fn main() {
    assert_eq!(spiral_matrix(3, 5, link![3,0,2,6,8,1,7,9,4,2,5,5,0]), vec![vec![3, 0, 2, 6, 8], vec![5, 0, -1, -1, 1], vec![5, 2, 4, 9, 7]]);
    assert_eq!(spiral_matrix(1, 4, link![0,1,2]), vec![vec![0, 1, 2, -1]]);
}
