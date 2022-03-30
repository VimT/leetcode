//! 相似字符串组

use leetcode::svec;

pub fn num_similar_groups(strs: Vec<String>) -> i32 {
    fn find(f: &mut Vec<usize>, x: usize) -> usize {
        return if f[x] == x {
            x
        } else {
            f[x] = find(f, f[x]);
            return f[x];
        };
    }
    let len = strs.len();
    let mut f: Vec<usize> = (0..len).collect();
    fn check(s1: &[u8], s2: &[u8]) -> bool {
        let mut n = 0;
        for i in 0..s1.len() {
            if s1[i] != s2[i] {
                n += 1;
                if n > 2 { return false; }
            }
        }
        return n != 1;
    }
    let mut ans = len as i32;
    for i in 0..len {
        for j in i + 1..len {
            let ii = find(&mut f, i);
            let jj = find(&mut f, j);
            if ii == jj { continue; }
            if check(strs[i].as_bytes(), strs[j].as_bytes()) {
                f[ii] = jj;
                ans -= 1;
            }
        }
    }
    ans
}


fn main() {
    assert_eq!(num_similar_groups(svec!["tars", "rats", "arts", "star"]), 2);
    assert_eq!(num_similar_groups(svec!["omv", "ovm"]), 1);
}
