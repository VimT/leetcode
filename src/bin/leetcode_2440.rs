//! 创建价值相同的连通块

pub fn component_value(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
    let len = nums.len();
    let mut g = vec![vec![]; len];
    for edge in edges {
        g[edge[0] as usize].push(edge[1] as usize);
        g[edge[1] as usize].push(edge[0] as usize);
    }
    let mut max = 0;
    let mut sum = 0;
    for &val in &nums {
        max = max.max(val);
        sum += val;
    }
    struct Closure<'s> {
        f: &'s dyn Fn(&Closure, usize, usize, i32) -> Option<i32>,
    }

    let dfs = Closure {
        f: &|c, v, fa, target| {
            let mut s = nums[v];
            for &w in &g[v] {
                if w != fa {
                    match (c.f)(c, w, v, target) {
                        None => return None,
                        Some(res) => s += res
                    }
                }
            }
            if s > target {
                return None;
            }
            if target == s {
                return Some(0);
            }
            Some(s)
        }
    };


    for i in (1..=(len as i32).min(sum / max)).rev() {
        if sum % i != 0 { continue; }
        if let Some(t) = (dfs.f)(&dfs, 0, len, sum / i) {
            if t == 0 {
                return i - 1;
            }
        }
    }
    0
}


fn main() {
    assert_eq!(component_value(vec![6, 2, 2, 2, 6], vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4]]), 2);
    assert_eq!(component_value(vec![2], vec![]), 0);
}
