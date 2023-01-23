//! 得到新鲜甜甜圈的最多组数

use std::collections::HashMap;
use rand::prelude::*;

/// 记忆化搜索
pub fn max_happy_groups(batch_size: i32, groups: Vec<i32>) -> i32 {
    let bs = batch_size as usize;
    let mut cnt = vec![0; bs]; // cnt[i]表示 顾客为i个的 group有多少个
    for group in groups {
        cnt[(group as usize) % bs] += 1;
    }
    fn dfs(left: usize, bs: usize, cnt: &mut Vec<i32>, cache: &mut HashMap<(usize, Vec<i32>), i32>) -> i32 {
        if let Some(v) = cache.get(&(left, cnt.clone())) {
            return *v;
        }
        let mut result = 0;
        for r in 1..bs {
            if cnt[r] > 0 {
                cnt[r] -= 1;
                result = result.max((left == 0) as i32 + dfs((left + r) % bs, bs, cnt, cache));
                cnt[r] += 1;
            }
        }
        cache.insert((left, cnt.clone()), result);
        result
    }
    let mut happy = cnt[0];
    for i in 1..bs / 2 {
        let can = cnt[i].min(cnt[bs - i]);
        happy += can;
        cnt[i] -= can;
        cnt[bs - i] -= can;
    }
    if bs & 1 == 0 {
        happy += cnt[bs / 2] / 2;
        cnt[bs / 2] &= 1;
    }
    happy + dfs(0, bs, &mut cnt, &mut HashMap::new())
}

/// 状态压缩，batch_size 最大是9，最多有 4种 顾客不知道高不高兴。
/// 每种最多30个，所以可以用 4个5位来表示，left可以用4位表示
pub fn max_happy_groups2(batch_size: i32, groups: Vec<i32>) -> i32 {
    let bs = batch_size as usize;
    let mut cnt = vec![0; bs]; // cnt[i]表示 顾客为i个的 group有多少个
    let mut result = 0;
    for group in groups {
        let idx = (group as usize) % bs;
        if idx == 0 {
            result += 1;
        } else if cnt[bs - idx] > 0 {
            cnt[bs - idx] -= 1;  // 直接配对
            result += 1;
        } else {
            cnt[idx] += 1;
        }
    }
    let mut mask = 0;
    let mut val = vec![];
    for i in 1..bs {
        if cnt[i] > 0 {
            val.push(i as i32);
            mask = mask << 5 | cnt[i];
        }
    }
    val.reverse();
    fn dfs(bs: i32, val: &Vec<i32>, mask: i32, cache: &mut HashMap<i32, i32>) -> i32 {
        if let Some(&v) = cache.get(&mask) {
            return v;
        }
        let mut result = 0;
        let left = mask >> 20;
        for (i, &r) in val.iter().enumerate() {
            if mask >> (i * 5) & 31 > 0 {
                let next_mask = (left + r) % bs << 20 | ((mask & ((1 << 20) - 1)) - (1 << (i * 5)));
                result = result.max((left == 0) as i32 + dfs(bs, val, next_mask, cache));
            }
        }
        cache.insert(mask, result);
        result
    }

    result + dfs(bs as i32, &val, mask, &mut HashMap::new())
}

/// 模拟退火，什么高科技（实际过不了）
pub fn max_happy_groups3(batch_size: i32, groups: Vec<i32>) -> i32 {
    let mut w = groups;
    let len = w.len();
    let m = batch_size;
    let mut result = 0;
    fn calc(m: i32, w: &Vec<i32>, result: &mut i32) -> i32 {
        let mut r0 = 1;
        let mut s = 0;
        for i in 0..w.len() {
            s = (s + w[i]) % m;
            if s == 0 && i < w.len() - 1 {
                r0 += 1;
            }
        }
        (*result) = (*result).max(r0);
        r0
    }
    let mut simulate_anneal = || {
        let mut rng = thread_rng();
        w.shuffle(&mut rng);
        let mut t = 1e6;
        while t > 1e-5 {
            let a = rng.gen_range(0, len);
            let b = rng.gen_range(0, len);
            let x = calc(m, &w, &mut result);
            w.swap(a, b);
            let y = calc(m, &w, &mut result);
            let delta = x - y;
            if (-delta as f64 / t).exp() < rng.gen() {
                w.swap(a, b);
            }
            t *= 0.97;
        }
    };
    for _ in 0..80 {
        simulate_anneal();
    }
    result
}

fn main() {
    fn test(func: fn(batch_size: i32, groups: Vec<i32>) -> i32) {
        assert_eq!(func(9, vec![1, 8, 1, 8, 1, 8, 1, 8, 2, 7, 2, 7, 2, 7, 2, 7, 3, 6, 3, 6, 3, 6, 3, 6, 4, 5, 4, 5, 4, 5]), 15);
        assert_eq!(func(9, vec![3, 1, 3, 3, 5, 6, 1, 1, 9, 10, 3, 3, 3, 1, 1, 3, 3, 3, 19, 20, 1, 3, 3, 3, 3, 1, 1, 3, 3, 30]), 9);
        assert_eq!(func(3, vec![1, 2, 3, 4, 5, 6]), 4);
        assert_eq!(func(4, vec![1, 3, 2, 5, 2, 2, 1, 6]), 4);
    }
    test(max_happy_groups);
    test(max_happy_groups2);
    test(max_happy_groups3);
}
