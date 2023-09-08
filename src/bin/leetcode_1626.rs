//! 无矛盾的最佳球队

pub fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
    let mut sa: Vec<(i32, i32)> = scores.into_iter().zip(ages).collect();
    sa.sort_unstable();
    // LIS
    let mut dp: Vec<i32> = sa.iter().map(|x| x.0).collect(); // 末尾元素为i的最长上升序列
    for (i, &(score, _)) in sa.iter().enumerate() {
        dp[i] = (0..i).filter(|&j| sa[j].1 <= sa[i].1).map(|x| dp[x]).fold(0, i32::max) + score;
    }
    dp.into_iter().max().unwrap()
}


/// 树状数组，对1的优化：找 <= 当前age 的最大的score
pub fn best_team_score2(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
    let mut sa: Vec<(i32, i32)> = scores.into_iter().zip(ages).collect();
    sa.sort_unstable();
    let max_score = sa.iter().map(|x| x.1).max().unwrap();
    struct BIT {
        tree: Vec<i32>,
        n: usize,
    }
    impl BIT {
        fn new(n: usize) -> Self {
            Self { tree: vec![0; n + 1], n }
        }
        fn lowbit(x: usize) -> usize {
            x & (x ^ (x - 1))
        }
        fn update(&mut self, mut x: usize, val: i32) {
            while x <= self.n {
                self.tree[x] = self.tree[x].max(val);
                x += Self::lowbit(x);
            }
        }
        fn query(&self, mut x: usize) -> i32 {
            let mut ans = 0;
            while x > 0 {
                ans = ans.max(self.tree[x]);
                x -= Self::lowbit(x);
            }
            ans
        }
    }
    let mut bit = BIT::new(max_score as usize);
    let mut result = 0;
    for (score, age) in sa {
        let val = bit.query(age as usize) + score;
        result = result.max(val);
        bit.update(age as usize, val);
    }
    result
}

fn main() {
    fn test(func: fn(scores: Vec<i32>, ages: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 3, 5, 10, 15], vec![1, 2, 3, 4, 5]), 34);
        assert_eq!(func(vec![4, 5, 6, 5], vec![2, 1, 2, 1]), 16);
        assert_eq!(func(vec![1, 2, 3, 5], vec![8, 9, 10, 1]), 6);
    }
    test(best_team_score);
    test(best_team_score2);
}
