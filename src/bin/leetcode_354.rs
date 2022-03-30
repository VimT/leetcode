//! 俄罗斯套娃信封问题

pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
    envelopes.sort_unstable();
    let mut ans = 0;
    let len = envelopes.len();
    let mut dp = vec![1; len];
    for i in 1..len {
        for j in (0..i).rev() {
            if envelopes[j][0] < envelopes[i][0] && envelopes[j][1] < envelopes[i][1] {
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
        ans = ans.max(dp[i]);
    }
    ans
}

pub fn max_envelopes_bin(mut envelopes: Vec<Vec<i32>>) -> i32 {
    envelopes.sort_unstable_by_key(|x| (x[0], std::cmp::Reverse(x[1])));
    let len = envelopes.len();
    let mut f = vec![envelopes[0][1]];
    for i in 1..len {
        let num = envelopes[i][1];
        if num > *f.last().unwrap() {
            f.push(num);
        } else {
            let idx = f.binary_search(&num).unwrap_or_else(|x| x);
            f[idx] = num;
        }
    }
    f.len() as i32
}


fn main() {
    fn test(func: fn(envelopes: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![5, 4], vec![6, 4], vec![6, 7], vec![2, 3]]), 3);
        assert_eq!(func(vec![vec![1, 1], vec![1, 1], vec![1, 1]]), 1);
    }
    test(max_envelopes);
    test(max_envelopes_bin);
}
