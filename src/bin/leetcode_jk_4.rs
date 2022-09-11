//! 九坤-04. 筹码游戏

use std::collections::HashMap;

/// https://leetcode.cn/circle/article/l8SRYI/
pub fn chip_game(mut nums: Vec<i32>, kind: i32) -> f64 {
    nums.sort_unstable_by_key(|x| -*x);
    while nums.len() < kind as usize {
        nums.push(0);
    }
    fn dfs(nums: &Vec<i32>, kind: i32, state: Vec<i32>, cache: &mut HashMap<Vec<i32>, f64>) -> f64 {
        if let Some(&result) = cache.get(&state) {
            return result
        }
        if &state == nums { return 0.; }
        let mut e = 0.;
        let mut vain = 0.;
        for i in 0..state.len() {
            let mut new_state = state.clone();
            new_state[i] += 1;
            new_state.sort_unstable_by_key(|x| -*x);

            let mut ok = true;
            for j in 0..kind as usize {
                if new_state[j] > nums[j] {
                    ok = false;
                    break;
                }
            }
            if !ok {
                vain += 1. / kind as f64;
                continue;
            }
            e += (dfs(nums, kind, new_state, cache) + 1.) / kind as f64;
        }
        let result = (vain + e) / (1. - vain);
        cache.insert(state, result);
        result
    }
    let mut cache = HashMap::new();
    cache.insert(nums.clone(), 0.);
    dfs(&nums, kind, vec![0; kind as usize], &mut cache)
}

fn main() {
    assert_eq!(chip_game(vec![1, 1], 2), 3.);
    assert_eq!(chip_game(vec![1, 2], 4), 3.833333333333333);
}
