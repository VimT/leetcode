//! 找出最大的 N 位 K 回文数

/// 打表做法
pub fn largest_palindrome(n: i32, k: i32) -> String {
    let n = n as usize;
    match k {
        1 | 3 | 9 => "9".repeat(n),
        2 => if n <= 2 { "8".repeat(n) } else { format!("8{}8", "9".repeat(n - 2)) },
        4 => if n <= 4 { "8".repeat(n) } else { format!("88{}88", "9".repeat(n - 4)) },
        8 => if n <= 6 { "8".repeat(n) } else { format!("888{}888", "9".repeat(n - 6)) },
        5 => if n <= 2 { "5".repeat(n) } else { format!("5{}5", "9".repeat(n - 2)) },
        6 => {
            if n <= 2 {
                "6".repeat(n)
            } else if n % 2 == 1 {
                let x = "9".repeat((n - 3) / 2);
                format!("8{}8{}8", &x, &x)
            } else {
                let x = "9".repeat((n - 4) / 2);
                format!("8{}77{}8", &x, &x)
            }
        }
        7 => {
            if n <= 2 {
                "7".repeat(n)
            } else {
                let cycle = (n - 3) % 12;
                let x = "9".repeat((n - 1) / 2);
                match cycle {
                    0 => format!("{}5{}", &x, &x),
                    1 => format!("{}77{}", &x, &x),
                    2 => format!("{}7{}", &x, &x),
                    3 => format!("{}99{}", &x, &x),
                    4 => format!("{}4{}", &x, &x),
                    5 => format!("{}44{}", &x, &x),
                    6 => format!("{}6{}", &x, &x),
                    7 => format!("{}44{}", &x, &x),
                    8 => format!("{}4{}", &x, &x),
                    9 => format!("{}99{}", &x, &x),
                    10 => format!("{}7{}", &x, &x),
                    11 => format!("{}77{}", &x, &x),
                    _ => unreachable!(),
                }
            }
        }
        _ => unreachable!(),
    }
}


/// 建图（灵神做法）
pub fn largest_palindrome2(n: i32, k: i32) -> String {
    let n = n as usize;
    let k = k as usize;
    let mut pow10 = vec![1; n];
    for i in 1..n {
        pow10[i] = (pow10[i - 1] * 10) % k;
    }
    let mut result = vec![0; n];
    let m = (n + 1) / 2;
    let mut vis = vec![vec![false; k]; m + 1];
    fn dfs(g @ (n, k, m, pow10): (usize, usize, usize, &Vec<usize>), i: usize, j: usize, vis: &mut Vec<Vec<bool>>, result: &mut Vec<u8>) -> bool {
        if i == m { return j == 0; }
        vis[i][j] = true;
        for d in (0..=9).rev() {
            let j2 = if n % 2 == 1 && i == m - 1 {
                (j + d * pow10[i]) % k
            } else {
                (j + d * pow10[i] + d * pow10[n - 1 - i]) % k
            };
            if !vis[i + 1][j2] && dfs(g, i + 1, j2, vis, result) {
                result[i] = d as u8 + b'0';
                result[n - 1 - i] = d as u8 + b'0';
                return true;
            }
        }
        false
    }
    dfs((n, k, m, &pow10), 0, 0, &mut vis, &mut result);
    String::from_utf8(result).unwrap()
}

fn main() {
    fn test(func: fn(n: i32, k: i32) -> String) {
        assert_eq!(func(3, 5), String::from("595"));
        assert_eq!(func(1, 4), String::from("8"));
        assert_eq!(func(5, 6), String::from("89898"));
    }
    test(largest_palindrome);
    test(largest_palindrome2);
}
