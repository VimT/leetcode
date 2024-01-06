//! 转换字符串的最小成本 I

pub fn minimum_cost(source: String, target: String, original: Vec<char>, changed: Vec<char>, cost: Vec<i32>) -> i64 {
    let mut g = vec![vec![i64::MAX / 2; 26]; 26];
    for i in 0..26 {
        g[i][i] = 0;
    }
    for i in 0..original.len() {
        let (a, b) = (original[i] as usize - 'a' as usize, changed[i] as usize - 'a' as usize);
        g[a][b] = g[a][b].min(cost[i] as i64);
    }
    for k in 0..26 {
        for i in 0..26 {
            for j in 0..26 {
                g[i][j] = g[i][j].min(g[i][k] + g[k][j]);
            }
        }
    }
    let mut result = 0;
    for (&a, &b) in source.as_bytes().iter().zip(target.as_bytes()) {
        let (a, b) = (a as usize - 'a' as usize, b as usize - 'a' as usize);
        if g[a][b] >= i64::MAX / 2 {
            return -1;
        }
        result += g[a][b];
    }
    result
}

fn main() {
    fn test(func: fn(source: String, target: String, original: Vec<char>, changed: Vec<char>, cost: Vec<i32>) -> i64) {
        assert_eq!(func(String::from("abcd"), String::from("acbe"), vec!['a', 'b', 'c', 'c', 'e', 'd'], vec!['b', 'c', 'b', 'e', 'b', 'e'], vec![2, 5, 5, 1, 2, 20]), 28);
        assert_eq!(func(String::from("aaaa"), String::from("bbbb"), vec!['a', 'c'], vec!['c', 'b'], vec![1, 2]), 12);
        assert_eq!(func(String::from("abcd"), String::from("abce"), vec!['a'], vec!['e'], vec![10000]), -1);
    }
    test(minimum_cost);
}
