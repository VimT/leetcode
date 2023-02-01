//! 通过投票对团队排名

pub fn rank_teams(votes: Vec<String>) -> String {
    let m = votes.len();
    let n = votes[0].len();
    let mut ranked = vec![vec![0; n]; 26];
    for j in 0..n {
        for i in 0..m {
            let ch = votes[i].as_bytes()[j];
            ranked[(ch - b'A') as usize][j] -= 1;
        }
    }
    let mut ranked: Vec<(Vec<i8>, u8)> = ranked.into_iter().zip(b'A'..).filter(|x| x.0.iter().any(|&x| x < 0)).collect();
    ranked.sort_unstable();
    unsafe { String::from_utf8_unchecked(ranked.into_iter().map(|x| x.1).collect()) }
}

fn main() {
    use leetcode::svec;
    fn test(func: fn(votes: Vec<String>) -> String) {
        assert_eq!(func(svec!["BCA","CAB","CBA","ABC","ACB","BAC"]), String::from("ABC"));
        assert_eq!(func(svec!["ABC","ACB","ABC","ACB","ACB"]), String::from("ACB"));
        assert_eq!(func(svec!["WXYZ","XYZW"]), String::from("XWYZ"));
        assert_eq!(func(svec!["ZMNAGUEDSJYLBOPHRQICWFXTVK"]), String::from("ZMNAGUEDSJYLBOPHRQICWFXTVK"));
    }
    test(rank_teams);
}
