//! 转换字符串的最小成本 II


pub fn minimum_cost(source: String, target: String, original: Vec<String>, changed: Vec<String>, cost: Vec<i32>) -> i64 {
    #[derive(Default, Debug)]
    pub struct Trie {
        children: [Option<Box<Trie>>; 26],
        id: Option<usize>,
    }

    let mut sid = 0;
    let mut trie = Box::<Trie>::default();
    let mut put = |s: &String| {
        let mut node = &mut trie;
        for &i in s.as_bytes().iter().rev() {
            node = node.children[(i - b'a') as usize].get_or_insert_with(Box::<Trie>::default);
        }
        if node.id.is_none() {
            node.id = Some(sid);
            sid += 1;
        }
        node.id.unwrap()
    };

    let mut g = vec![vec![i64::MAX / 2; cost.len() * 2]; cost.len() * 2];
    for ((o, c), cost) in original.into_iter().zip(changed).zip(cost) {
        let (a, b) = (put(&o), put(&c));
        g[a][b] = g[a][b].min(cost as i64);
    }
    let len = sid;
    for i in 0..len { g[i][i] = 0; }
    for k in 0..len {
        for i in 0..len {
            if g[i][k] >= i64::MAX / 2 { continue; }  // floyd优化
            for j in 0..len {
                g[i][j] = g[i][j].min(g[i][k] + g[k][j]);
            }
        }
    }
    let len = source.len();
    let mut dp = vec![0i64; len + 1];  // 前i个字符转换为target的最小成本
    for i in 1..=len {
        let mut min = if source.as_bytes()[i - 1] == target.as_bytes()[i - 1] { dp[i - 1] } else { i64::MAX };
        let mut st = Some(&trie);
        let mut tt = Some(&trie);
        for j in (0..i).rev() {
            st = st.and_then(|st| st.children[(source.as_bytes()[j] - b'a') as usize].as_ref());
            tt = tt.and_then(|tt| tt.children[(target.as_bytes()[j] - b'a') as usize].as_ref());
            if let (Some(a), Some(b)) = (&st, &tt) {
                if let (Some(a), Some(b)) = (a.id, b.id) {
                    min = min.min(dp[j].saturating_add(g[a][b]));
                }
            } else { break; }
        }
        dp[i] = min;
    }
    if dp[len] >= i64::MAX / 2 { -1 } else { dp[len] }
}

fn main() {
    use leetcode::svec;
    fn test(func: fn(source: String, target: String, original: Vec<String>, changed: Vec<String>, cost: Vec<i32>) -> i64) {
        assert_eq!(func(String::from("abcdefgh"), String::from("acdeeghh"), svec!["bcd","fgh","thh"], svec!["cde","thh","ghh"], vec![1, 3, 5]), 9);
        assert_eq!(func(String::from("abcd"), String::from("acbe"), svec!["a","b","c","c","e","d"], svec!["b","c","b","e","b","e"], vec![2, 5, 5, 1, 2, 20]), 28);
        assert_eq!(func(String::from("abcdefgh"), String::from("addddddd"), svec!["bcd","defgh"], svec!["ddd","ddddd"], vec![100, 1578]), -1);
    }
    test(minimum_cost);
}
