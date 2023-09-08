//! 连接所有点的最小费用

use leetcode::union_find::UnionFind;

fn dis(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
    (a[0] - b[0]).abs() + (a[1] - b[1]).abs()
}

/// 最小生成树：Kruskal
pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
    let len = points.len();
    let mut distance = Vec::with_capacity(len * len);
    for i in 0..len {
        for j in 0..len {
            if i != j {
                distance.push((dis(&points[i], &points[j]), i, j));
            }
        }
    }

    distance.sort_unstable();
    let mut uf = UnionFind::new(len);
    let mut result = 0;
    for (d, x, y) in distance {
        if us.union(x, y) {
            result += d;
            if us.count == 1 { break; }
        }
    }
    result
}

/// 最小生成树：Prim
pub fn min_cost_connect_points2(points: Vec<Vec<i32>>) -> i32 {
    let len = points.len();
    let mut g = vec![vec![0; len]; len];
    for i in 0..len {
        for j in i + 1..len {
            let d = dis(&points[i], &points[j]);
            g[i][j] = d;
            g[j][i] = d;
        }
    }
    let mut low_cost = g[0].clone();
    let mut seen = vec![false; len];
    seen[0] = true;
    let mut result = 0;
    for _ in 1..len {
        let u = (0..len).filter(|&x| !seen[x]).min_by_key(|&x| low_cost[x]).unwrap();
        let min_val = low_cost[u];
        seen[u] = true;
        result += min_val;
        for j in 0..len {
            if !seen[j] && g[u][j] < low_cost[j] {
                low_cost[j] = g[u][j];
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(points: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![0, 0], vec![2, 2], vec![3, 10], vec![5, 2], vec![7, 0]]), 20);
        assert_eq!(func(vec![vec![3, 12], vec![-2, 5], vec![-4, 1]]), 18);
        assert_eq!(func(vec![vec![0, 0], vec![1, 1], vec![1, 0], vec![-1, 1]]), 4);
        assert_eq!(func(vec![vec![-1000000, -1000000], vec![1000000, 1000000]]), 4000000);
        assert_eq!(func(vec![vec![0, 0]]), 0);
    }
    test(min_cost_connect_points);
    test(min_cost_connect_points2);
}
