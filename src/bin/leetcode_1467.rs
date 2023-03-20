//! 两个盒子中球的颜色数相同的概率

/// 直接暴力，枚举左边盒子每个球放多少个
pub fn get_probability(balls: Vec<i32>) -> f64 {
    struct DFS {
        balls: Vec<i32>,
        t1: Vec<i32>,
        t2: Vec<i32>,
        s1: i32,
        s2: i32,
        half: i32,
        result: u64,
        total: u64,
        fac: Vec<u64>,
    }

    impl DFS {
        fn comb(&self, x: u64, y: u64) -> u64 {
            self.fac[x as usize] / self.fac[y as usize] / self.fac[(x - y) as usize]
        }
        fn method(&self, tmp: &Vec<i32>) -> u64 {
            let mut result = 1;
            for (&x, &y) in self.balls.iter().zip(tmp) {
                result *= self.comb(x as u64, y as u64);
            }
            result
        }
        fn dfs(&mut self, idx: usize) {
            if self.s1 > self.half || self.s2 > self.half { return; }
            if idx == self.balls.len() {
                if self.s1 == self.s2 {
                    let methods = self.method(&self.t1);
                    if self.t1.iter().filter(|x| **x == 0).count() == self.t2.iter().filter(|x| **x == 0).count() {
                        self.result += methods;
                    }
                    self.total += methods;
                }
                return;
            }
            for left in 0..=self.balls[idx] {
                let right = self.balls[idx] - left;
                self.s1 += left;
                self.s2 += right;
                self.t1.push(left);
                self.t2.push(right);
                self.dfs(idx + 1);
                self.s1 -= left;
                self.s2 -= right;
                self.t1.pop();
                self.t2.pop();
            }
        }
    }

    let mut fac = vec![0; 10];
    fac[0] = 1;
    for i in 1..10 {
        fac[i] = fac[i - 1] * i as u64;
    }
    let half: i32 = balls.iter().sum();
    let mut d = DFS { balls, t1: vec![], t2: vec![], s1: 0, s2: 0, half, result: 0, total: 0, fac };
    d.dfs(0);
    d.result as f64 / d.total as f64
}

fn main() {
    fn test(func: fn(balls: Vec<i32>) -> f64) {
        assert_eq!(func(vec![1, 1]), 1.00000);
        assert_eq!(func(vec![2, 1, 1]), 0.6666666666666666);
        assert_eq!(func(vec![1, 2, 1, 2]), 0.60000);
    }
    test(get_probability);
}
