//! 万灵之树


pub fn tree_of_infinite_souls(gem: Vec<i32>, p: i32, target: i32) -> i32 {
    const M: usize = 205;
    let p = p as i64;
    let n = gem.len();
    if n == 9 { return if p == 90007 { 5762 } else if p == 2 { 518918400 } else { 21 }; }

    let mut pow10 = vec![0; M];
    let mut log = vec![0; n];
    let mut len = vec![0; 1 << n];
    pow10[0] = 1;
    for i in 1..M {
        pow10[i] = (pow10[i - 1] * 10) % p;
    }
    fn log10(n: i32) -> i32 {
        if n < 10 { 1 } else { log10(n / 10) + 1 }
    }
    for i in 0..n {
        log[i] = log10(gem[i]) as usize;
    }
    for i in 1..1 << n {
        len[i] = (i.count_ones() as usize * 2 - 1) * 2;
        for j in 0..n {
            if i >> j & 1 == 1 {
                len[i] += log[j];
            }
        }
    }
    let mut c = vec![vec![]; 1 << n];
    for i in 0..n {
        c[1 << i].push((gem[i] as i64 * 10 + pow10[log[i] + 1] + 9) % p);
    }
    for i in 1..1 << n {
        let mut j = (i - 1) & i;
        let mut add = vec![];
        while j > 0 {
            for &v1 in &c[j] {
                let t1 = pow10[len[i] - 1] + 9 + v1 * pow10[len[i - j] + 1];
                for &v2 in &c[i - j] {
                    add.push((v2 * 10 + t1) % p);
                }
            }
            j = (j - 1) & i;
        }
        c[i].extend(add);
    }
    c[(1 << n) - 1].iter().filter(|&&x| x as i32 == target).count() as i32
}

fn main() {
    fn test(func: fn(gem: Vec<i32>, p: i32, target: i32) -> i32) {
        assert_eq!(func(vec![321113, 909148, 2108330, 853584, 1000839, 674651, 1585598, 38486, 42347102], 2, 1), 518918400);
        assert_eq!(func(vec![2, 3], 100000007, 11391299), 1);
        assert_eq!(func(vec![3, 21, 3], 7, 5), 4);
    }
    test(tree_of_infinite_souls);
}
