//! 生物进化录

pub fn evolutionary_record(parents: Vec<i32>) -> String {
    let len = parents.len();
    let mut g = vec![vec![]; len];
    for i in 1..len {
        g[parents[i] as usize].push(i);
    }
    fn dfs(g: &Vec<Vec<usize>>, x: usize) -> Vec<u8> {
        if g[x].len() == 0 { return b"01".to_vec(); }
        let mut a: Vec<Vec<u8>> = g[x].iter().map(|&y| dfs(g, y)).collect();
        a.sort_unstable();
        let len = a.iter().map(|x| x.len()).sum::<usize>() + 2;
        let mut result = Vec::with_capacity(len);
        result.push(b'0');
        for b in a {
            result.extend(b);
        }
        result.push(b'1');
        result
    }
    let mut result = dfs(&g, 0);
    while *result.last().unwrap() == b'1' {
        result.pop();
    }
    unsafe { String::from_utf8_unchecked(result[1..].to_vec()) }
}

fn main() {
    fn test(func: fn(parents: Vec<i32>) -> String) {
        assert_eq!(func(vec![-1, 0, 0, 2]), String::from("00110"));
        assert_eq!(func(vec![-1, 0, 0, 1, 2, 2]), String::from("00101100"));
    }
    test(evolutionary_record);
}
