//! 贴纸拼词

use std::collections::HashMap;
use leetcode::svec;

pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
    let len = stickers.len();
    let mut bin = vec![0; len];
    let mut target_bin = 0;
    let mut cnt = vec![[0; 26]; len];
    let mut target_cnt = [0; 26];
    let mut all = 0;
    for i in 0..len {
        let s = stickers[i].as_bytes();
        for &ch in s {
            bin[i] |= 1 << (ch - b'a');
            cnt[i][(ch - b'a') as usize] += 1;
        }
        all |= bin[i];
    }
    let t = target.as_bytes();
    for &ch in t {
        target_bin |= 1 << (ch - b'a');
        target_cnt[(ch - b'a') as usize] += 1;
        if all & (1 << ch - b'a') == 0 { return -1; }
    }
    fn dfs(bin: &Vec<i32>, cnt: &Vec<[i32; 26]>, target_bin: i32, target_cnt: [i32; 26], cache: &mut HashMap<[i32; 26], i32>) -> i32 {
        if target_bin == 0 {
            return 0;
        }
        if let Some(&v) = cache.get(&target_cnt) { return v; }
        let len = bin.len();
        let mut result = i32::MAX / 2;
        for i in 0..len {
            if bin[i] & target_bin > 0 {
                let mut tcc = target_cnt.clone();
                let mut tbc = target_bin;
                for c in 0..26 {
                    if tcc[c] > 0 {
                        if cnt[i][c] >= tcc[c] {
                            tcc[c] = 0;
                            tbc ^= 1 << c;
                        } else {
                            tcc[c] -= cnt[i][c];
                        }
                    }
                }
                result = result.min(dfs(bin, cnt, tbc, tcc, cache) + 1)
            }
        }
        cache.insert(target_cnt, result);
        result
    }
    let result = dfs(&bin, &cnt, target_bin, target_cnt, &mut HashMap::new());
    if result >= i32::MAX / 2 { -1 } else { result }
}


pub fn min_stickers_dp(stickers: Vec<String>, target: String) -> i32 {
    let len = stickers.len();
    let mut bin = vec![0; len];
    let mut cnt = vec![[0; 26]; len];
    let mut all = 0;
    for i in 0..len {
        let s = stickers[i].as_bytes();
        for &ch in s {
            bin[i] |= 1 << (ch - b'a');
            cnt[i][(ch - b'a') as usize] += 1;
        }
        all |= bin[i];
    }
    let t = target.as_bytes();
    let tlen = t.len();
    for &i in t {
        if all & (1 << i - b'a') == 0 { return -1; }
    }
    let mut dp = vec![i32::MAX / 2; 1 << tlen];
    dp[0] = 0;
    for i in 0..1 << tlen {
        let mut cur_bin = 0;
        for j in 0..tlen {
            if i & (1 << j) == 0 {
                cur_bin |= 1 << (t[j] - b'a');
            }
        }
        for j in 0..len {
            if bin[j] & cur_bin > 0 {
                let mut cnt = cnt[j].clone();
                let mut nxt = i;
                for k in 0..tlen {
                    if i & (1 << k) == 0 {
                        if cnt[(t[k] - b'a') as usize] > 0 {
                            cnt[(t[k] - b'a') as usize] -= 1;
                            nxt ^= 1 << k;
                        }
                    }
                }
                dp[nxt] = dp[nxt].min(dp[i] + 1);
            }
        }
    }
    if dp[(1 << tlen) - 1] >= i32::MAX / 2 { -1 } else { dp[(1 << tlen) - 1] }
}

pub fn min_stickers_dp_best(stickers: Vec<String>, target: String) -> i32 {
    const INF: i32 = 0x3f3f3f3f;
    let (m, n) = (stickers.len(), target.len());
    let tot = 1 << n;
    let mut dp = vec![INF; tot];
    let target = target.into_bytes();
    let mut ss = vec![vec![0; 26]; m];
    let mut can: Vec<Vec<i32>> = vec![vec![]; 26];
    for (pos, i) in stickers.iter().enumerate() {
        let s = i.as_bytes();
        for j in 0..s.len() {
            let t = (s[j] - b'a') as usize;
            ss[pos][t] += 1;
            if can[t].len() == 0 || can[t][can[t].len() - 1] != pos as i32 { can[t].push(pos as i32); }
        }
    }
    dp[0] = 0;
    for i in 0..tot {
        if dp[i] == INF { continue; }
        let mut pos = 0;
        for j in 0..n {
            if (i & (1 << j)) == 0 {
                pos = j;
                break;
            }
        }
        for j in can[(target[pos] - b'a') as usize].iter() {
            let mut cnt = ss[(*j) as usize].clone();
            let mut nxt = i;
            for k in 0..n {
                if (nxt & (1 << k)) != 0 { continue; }
                if cnt[(target[k] - b'a') as usize] > 0 {
                    nxt = nxt | (1 << k);
                    cnt[(target[k] - b'a') as usize] -= 1;
                }
            }
            dp[nxt] = dp[nxt].min(dp[i] + 1);
        }
    }
    if dp[tot - 1] == INF { -1 } else { dp[tot - 1] }
}


fn main() {
    fn test(func: fn(stickers: Vec<String>, target: String) -> i32) {
        assert_eq!(func(svec!["with","example","science"], String::from("thehat")), 3);
        assert_eq!(func(svec!["notice","possible"], String::from("basicbasic")), -1);
    }
    test(min_stickers);
    test(min_stickers_dp);
    test(min_stickers_dp_best);
}
