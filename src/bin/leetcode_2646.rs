//! 最小化旅行的价格总和

pub fn minimum_total_price(n: i32, edges: Vec<Vec<i32>>, price: Vec<i32>, trips: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut g = vec![vec![]; n];
    for edge in edges {
        g[edge[0] as usize].push(edge[1] as usize);
        g[edge[1] as usize].push(edge[0] as usize);
    }
    struct DFS<'a> {
        g: &'a Vec<Vec<usize>>,
        price: Vec<i32>,
        iter: Vec<i32>,
    }
    impl<'a> DFS<'a> {
        // start -> end 路径上的所有点+1，可以使用树上差分优化
        fn dfs(&mut self, u: usize, fa: usize, t: usize) -> bool {
            if u == t {
                self.iter[u] += 1;
                return true;
            }
            for &v in &self.g[u] {
                if v != fa && self.dfs(v, u, t) {
                    self.iter[u] += 1;
                    return true;
                }
            }
            false
        }

        /// 打家劫舍3
        fn dfs2(&self, u: usize, fa: usize) -> (i32, i32) {
            let mut not_half = self.price[u] * self.iter[u];
            let mut half = not_half / 2;
            for &v in &self.g[u] {
                if v != fa {
                    let (sub_half, sub_not_half) = self.dfs2(v, u);
                    half += sub_not_half;
                    not_half += sub_half.min(sub_not_half);
                }
            }
            (half, not_half)
        }
    }

    let mut help = DFS { g: &g, price, iter: vec![0; n] };
    for trip in trips {
        help.dfs(trip[0] as usize, n, trip[1] as usize);
    }
    let (a, b) = help.dfs2(0, n);
    a.min(b)
}

fn main() {
    fn test(func: fn(n: i32, edges: Vec<Vec<i32>>, price: Vec<i32>, trips: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(4, vec![vec![0, 1], vec![1, 2], vec![1, 3]], vec![2, 2, 10, 6], vec![vec![0, 3], vec![2, 1], vec![2, 3]]), 23);
        assert_eq!(func(2, vec![vec![0, 1]], vec![2, 2], vec![vec![0, 0]]), 1);
    }
    test(minimum_total_price);
}
