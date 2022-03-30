//! 找到小镇的法官

pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    if n == 1 && trust.len() == 0 { return 1; }
    let mut t = vec![0; n + 1];
    let mut tc = vec![0; n + 1];
    for tru in trust {
        t[tru[0] as usize] = tru[1] as usize;
        tc[tru[1] as usize] += 1;
    }
    for i in 1..=n {
        if tc[i] == n - 1 && t[i] == 0 {
            return i as i32;
        }
    }
    -1
}

fn main() {
    assert_eq!(find_judge(2, vec![]), -1);
    assert_eq!(find_judge(1, vec![]), 1);
    assert_eq!(find_judge(2, vec![vec![1, 2]]), 2);
    assert_eq!(find_judge(3, vec![vec![1, 3], vec![2, 3]]), 3);
    assert_eq!(find_judge(3, vec![vec![1, 3], vec![2, 3], vec![3, 1]]), -1);
}
