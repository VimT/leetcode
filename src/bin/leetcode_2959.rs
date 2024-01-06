//! 关闭分部的可行集合数目

pub fn number_of_sets(n: i32, max_distance: i32, roads: Vec<Vec<i32>>) -> i32 {
    fn max(n: usize, mut dis: Vec<Vec<i32>>, enable: i32) -> i32 {
        for k in 0..n {
            if enable >> k & 1 == 1 {
                for i in 0..n {
                    if enable >> i & 1 == 1 {
                        for j in 0..n {
                            if enable >> j & 1 == 1 {
                                dis[i][j] = dis[i][j].min(dis[i][k] + dis[k][j]);
                            }
                        }
                    }
                }
            }
        }
        let mut result = 0;
        for i in 0..n {
            if enable >> i & 1 == 1 {
                for j in i + 1..n {
                    if enable >> j & 1 == 1 {
                        result = result.max(dis[i][j]);
                    }
                }
            }
        }
        result
    }
    let n = n as usize;
    let mut dis = vec![vec![i32::MAX >> 1; n]; n];
    for edge in roads {
        let (u, v) = (edge[0] as usize, edge[1] as usize);
        dis[u][v] = dis[u][v].min(edge[2]);
        dis[v][u] = dis[v][u].min(edge[2]);
    }
    for i in 0..n { dis[i][i] = 0; }

    let mut result = 0;
    for enable in 0..1 << n {
        if max(n, dis.clone(), enable) <= max_distance {
            result += 1;
        }
    }
    result
}

fn main() {
    fn test(func: fn(n: i32, max_distance: i32, roads: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(3, 5, vec![vec![0, 1, 2], vec![1, 2, 10], vec![0, 2, 10]]), 5);
        assert_eq!(func(3, 5, vec![vec![0, 1, 20], vec![0, 1, 10], vec![1, 2, 2], vec![0, 2, 2]]), 7);
        assert_eq!(func(1, 10, vec![]), 2);
    }
    test(number_of_sets);
}
