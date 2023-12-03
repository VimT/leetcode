//! 执行操作使两个字符串相等

pub fn min_operations(s1: String, s2: String, x: i32) -> i32 {
    struct DFS<'a> {
        s1: &'a [u8],
        s2: &'a [u8],
        x: i32,
        cache: Vec<Vec<[i32; 2]>>,
    }
    fn dfs(s: &mut DFS, i: usize, free: i32, revert: bool) -> i32 {
        if i == 0 { return 0; }
        let DFS { s1, s2, .. } = s;
        if s.cache[i - 1][free as usize][revert as usize] != -1 {
            return s.cache[i - 1][free as usize][revert as usize];
        }
        if (s1[i - 1] == s2[i - 1]) == !revert {
            return dfs(s, i - 1, free, false);
        }
        let mut result = (s.x + dfs(s, i - 1, free + 1, false))
            .min(1 + dfs(s, i - 1, free, true));
        if free > 0 {
            result = result.min(dfs(s, i - 1, free - 1, revert));
        }
        s.cache[i - 1][free as usize][revert as usize] = result;
        result
    }
    if s1.as_bytes().iter().filter(|x| **x == b'1').count() % 2 != s2.as_bytes().iter().filter(|x| **x == b'1').count() % 2 {
        return -1;
    }
    let mut s = DFS {
        s1: s1.as_bytes(),
        s2: s2.as_bytes(),
        x,
        cache: vec![vec![[-1; 2]; s1.len()]; s1.len()],
    };
    dfs(&mut s, s1.len(), 0, false)
}


/// O(n) 的DP。 将 s1[i] != s2[i] 的下标i收集到数组p
/// dp[i] 表示修改p的前i的元素需要的最小花费
/// 对于操作1， 操作一个相当于花费 x/2： dp[i] = dp[i-1] + x/2
/// 对于操作2，相当于 p[i-1] 到 p[i] 都翻转：dp[i] = dp[i-1] + (p[i] - p[i-1])
/// 这两个取最小值
pub fn min_operations2(s1: String, s2: String, x: i32) -> i32 {
    if s1 == s2 { return 0; }
    let p = s1.as_bytes().iter().zip(s2.as_bytes()).zip(0..).filter_map(|((a, b), i)| if a != b { Some(i) } else { None }).collect::<Vec<i32>>();
    if p.len() % 2 == 1 { return -1; }
    let mut f0 = 0;
    let mut f1 = x;
    for a in p.windows(2) {
        let i = a[0];
        let j = a[1];
        let tmp = f0;
        f0 = f1;
        f1 = (f1 + x).min(tmp + (j - i) * 2);
    }
    f1 / 2
}

fn main() {
    fn test(func: fn(s1: String, s2: String, x: i32) -> i32) {
        assert_eq!(func(String::from("11001011111"), String::from("01111000110"), 2), 4);
        assert_eq!(func(String::from("10110"), String::from("00011"), 4), -1);
        assert_eq!(func(String::from("1100011000"), String::from("0101001010"), 2), 4);
    }
    test(min_operations);
    test(min_operations2);
}
