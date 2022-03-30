//! 最短超级串

use leetcode::svec;

/// dp(mask, i) 表示已经选出的字符串为 mask（mask 是一个长度为 A.length 的二进制数),
/// 且最后一个选出的字符串是 A[i] 时的重复部分的最大长度。
pub fn shortest_superstring(words: Vec<String>) -> String {
    let len = words.len();
    let mut overlap = vec![vec![0; len]; len];
    for i in 0..len {
        for j in 0..len {
            if i == j { continue; }
            let a = words[i].as_bytes();
            let b = words[j].as_bytes();
            let m = a.len().min(b.len());
            for k in (1..m).rev() {
                if &a[a.len() - k..] == &b[..k] {
                    overlap[i][j] = k;
                    break;
                }
            }
        }
    }

    let mut dp = vec![vec![0; len]; 1 << len];
    let mut parent = vec![vec![usize::MAX; len]; 1 << len];
    for mask in 0..1 << len {
        for bit in 0..len {
            if mask >> bit & 1 == 1 {
                let pmask = mask ^ (1 << bit);
                if pmask == 0 { continue; }
                for i in 0..len {
                    if pmask >> i & 1 == 1 {
                        let val = dp[pmask][i] + overlap[i][bit];
                        if val > dp[mask][bit] {
                            dp[mask][bit] = val;
                            parent[mask][bit] = i;
                        }
                    }
                }
            }
        }
    }

    let mut perm = vec![0; len];
    let mut seen = vec![false; len];
    let mut t = 0;
    let mut mask = (1 << len) - 1;
    let mut p = 0;
    for j in 0..len {
        if dp[(1 << len) - 1][j] > dp[(1 << len) - 1][p] {
            p = j;
        }
    }
    while p != usize::MAX {
        perm[t] = p;
        seen[p] = true;
        t += 1;
        let p2 = parent[mask][p];
        mask ^= 1 << p;
        p = p2;
    }

    perm[..t].reverse();
    for i in 0..len {
        if !seen[i] {
            perm[t] = i;
            t += 1;
        }
    }
    let mut ans = Vec::with_capacity(12 * 20);
    ans.extend_from_slice(words[perm[0]].as_bytes());
    for i in 1..len {
        let ov = overlap[perm[i - 1]][perm[i]];
        ans.extend_from_slice(&words[perm[i]].as_bytes()[ov..]);
    }
    unsafe { String::from_utf8_unchecked(ans) }
}


fn main() {
    assert_eq!(shortest_superstring(svec!["alex","loves","leetcode"]), String::from("alexlovesleetcode"));
    assert_eq!(shortest_superstring(svec!["catg","ctaagt","gcta","ttca","atgcatc"]), String::from("gctaagttcatgcatc"));
}
