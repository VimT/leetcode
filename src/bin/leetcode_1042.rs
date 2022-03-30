//! 不邻接植花

/// 不用考虑重复的问题，就找周围没有的颜色就好了
pub fn garden_no_adj(n: i32, paths: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let mut result = vec![0; n];
    let mut m = vec![vec![]; n];
    for path in paths {
        m[path[0] as usize - 1].push(path[1] as usize - 1);
        m[path[1] as usize - 1].push(path[0] as usize - 1);
    }
    for i in 0..n {
        let mut used = [false; 4];
        if result[i] == 0 {
            for &nxt in &m[i] {
                if result[nxt] != 0 {
                    used[result[nxt] as usize - 1] = true;
                }
            }
            for j in 0..4 {
                if !used[j] {
                    result[i] = j as i32 + 1;
                    break;
                }
            }
        }
    }
    result
}

fn main() {
    assert_eq!(garden_no_adj(3, vec![vec![1, 2], vec![2, 3], vec![3, 1]]), vec![1, 2, 3]);
    assert_eq!(garden_no_adj(4, vec![vec![1, 2], vec![3, 4]]), vec![1, 2, 1, 2]);
    assert_eq!(garden_no_adj(4, vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 1], vec![1, 3], vec![2, 4]]), vec![1, 2, 3, 4]);
}
