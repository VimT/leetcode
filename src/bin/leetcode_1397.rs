//! 找到所有好字符串

const MOD: i32 = 1e9 as i32 + 7;

/// dp[pos][eval_pos][bound] 表示前pos位，匹配evil后eval_pos位，bound 0表示前pos位 != s1/s2，1表示匹配s1,2表示匹配s2
/// 学习1：复杂dp改如何分析
/// 学习2：在区间内构造时，如何标记上下区间
pub fn find_good_strings(n: i32, s1: String, s2: String, evil: String) -> i32 {
    let evil = evil.as_bytes();
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();

    struct Data<'a> {
        s1: &'a [u8],
        s2: &'a [u8],
        evil: &'a [u8],
        next: Vec<usize>,
        dp: Vec<Vec<[i32; 4]>>,
        trans: Vec<[Option<usize>; 26]>,
    }

    impl<'a> Data<'a> {
        fn get_trans(&mut self, mut evil_pos: usize, ch: u8) -> usize {
            if let Some(ans) = self.trans[evil_pos][(ch - b'a') as usize] {
                return ans;
            }
            while evil_pos > 0 && self.evil[evil_pos] != ch {
                evil_pos = self.next[evil_pos - 1];
            }
            let result = if self.evil[evil_pos] == ch { evil_pos + 1 } else { 0 };
            self.trans[evil_pos][(ch - b'a') as usize] = Some(result);
            result
        }

        fn dfs(&mut self, pos: usize, eval_pos: usize, bound: usize) -> i32 {
            if eval_pos == self.evil.len() {
                return 0;
            }
            if pos == self.s1.len() {
                return 1;
            }
            if self.dp[pos][eval_pos][bound] != -1 {
                return self.dp[pos][eval_pos][bound];
            }

            self.dp[pos][eval_pos][bound] = 0;
            let l = if bound & 1 > 0 { self.s1[pos] } else { b'a' };
            let r = if bound & 2 > 0 { self.s2[pos] } else { b'z' };
            for ch in l..=r {
                let nxt_eval_pos = self.get_trans(eval_pos, ch);
                let match_s1 = bound & 1 > 0 && ch == self.s1[pos];
                let match_s2 = bound & 2 > 0 && ch == self.s2[pos];
                let nxt_bound = (match_s1 as usize) | ((match_s2 as usize) << 1);
                self.dp[pos][eval_pos][bound] += self.dfs(pos + 1, nxt_eval_pos, nxt_bound);
                self.dp[pos][eval_pos][bound] %= MOD;
            }
            self.dp[pos][eval_pos][bound]
        }
    }


    // KMP
    let mut next = vec![0; evil.len()];
    for i in 1..evil.len() {
        let mut j = next[i - 1];
        while j > 0 && evil[j] != evil[i] {
            j = next[j - 1];
        }
        if evil[i] == evil[j] { next[i] = j + 1; }
    }

    let len = n as usize;
    let trans = vec![[None; 26]; evil.len()];
    let dp = vec![vec![[-1; 4]; evil.len()]; len];
    let mut data = Data { s1, s2, evil, next, dp, trans };
    data.dfs(0, 0, 3)
}


fn main() {
    fn test(func: fn(n: i32, s1: String, s2: String, evil: String) -> i32) {
        assert_eq!(func(2, String::from("aa"), String::from("da"), String::from("b")), 51);
        assert_eq!(func(8, String::from("leetcode"), String::from("leetgoes"), String::from("leet")), 0);
        assert_eq!(func(2, String::from("gx"), String::from("gz"), String::from("x")), 2);
    }
    test(find_good_strings);
}
