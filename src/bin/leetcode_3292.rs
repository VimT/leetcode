//! 形成目标字符串需要的最少字符串数 II

use std::collections::HashSet;

/// AC自动机优化DP
pub fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
    #[derive(Default)]
    struct Node {
        len: usize,
        // 失配指针，代表下一个最大的可能匹配。
        fail: usize,
        next: [usize; 26],
        output: Vec<usize>,
    }
    impl Node {
        fn new(len: usize) -> Self {
            Self { len, ..Default::default() }
        }
    }
    struct ACAutoMaton {
        nodes: Vec<Node>,
    }
    const ROOT: usize = 0;
    impl ACAutoMaton {
        fn with_capacity(len: usize) -> Self {
            let mut nodes = Vec::with_capacity(len + 2);
            nodes.push(Node::default()); // root
            Self { nodes }
        }
        // 正常 trie 的 insert
        fn insert(&mut self, s: &[u8]) {
            let mut cur = ROOT;
            let nodes = &mut self.nodes;
            for (&ch, i) in s.iter().zip(1..) {
                let idx = (ch - b'a') as usize;
                if nodes[cur].next[idx] == 0 {
                    nodes[cur].next[idx] = nodes.len();
                    nodes.push(Node::new(nodes[cur].len + 1));
                }
                cur = nodes[cur].next[idx];
                nodes[cur].output.push(i);
            }
            nodes[cur].len = s.len();
        }
        fn build(&mut self) {
            let mut q = std::collections::VecDeque::new();
            let nodes = &mut self.nodes;
            for i in 0..26 {
                if nodes[ROOT].next[i] != ROOT {
                    q.push_back(nodes[ROOT].next[i]);
                }
            }
            while let Some(u) = q.pop_front() {
                for i in 0..26 {
                    let v = nodes[u].next[i];
                    if v != ROOT {
                        nodes[v].fail = nodes[nodes[u].fail].next[i]; // 子节点的失配指针是 父节点失配指针的 next
                        q.push_back(v);
                    } else {
                        nodes[u].next[i] = nodes[nodes[u].fail].next[i];
                    }
                }
            }
        }
    }

    let t = target.as_bytes();
    let len = t.len();
    let mut ac = ACAutoMaton::with_capacity(words.iter().map(|x| x.len()).max().unwrap());
    for word in words {
        ac.insert(word.as_bytes());
    }
    ac.build();

    let mut dp = vec![0; len + 1];
    let mut cur = ROOT;
    for (&ch, i) in t.iter().zip(1..) {
        cur = ac.nodes[cur].next[(ch - b'a') as usize];
        if cur == ROOT { return -1; }
        // cur.next[ch].len 就是当前节点的最大匹配前缀
        dp[i] = dp[i - ac.nodes[cur].len] + 1;
    }

    dp[len]
}

/// 字符串hash+跳跃游戏
pub fn min_valid_strings2(words: Vec<String>, target: String) -> i32 {
    use rand::Rng;
    let len = target.len();
    const MOD: i64 = 1_070_777_777;
    let mut rng = rand::thread_rng();
    let base = rng.gen_range((8e8 as i64)..(9e8 as i64));  // 随机base防止被hack
    let mut pow_base = vec![1; len + 1];
    let mut pre_hash = vec![0; len + 1];
    let t = target.as_bytes();
    for (i, &ch) in t.iter().enumerate() {
        pow_base[i + 1] = pow_base[i] * base % MOD;
        pre_hash[i + 1] = (pre_hash[i] * base + ch as i64) % MOD;
    }
    // 计算 target [l, r) 的hash值
    let sub_hash = |l: usize, r: usize| -> i64 {
        (pre_hash[r] + MOD - pre_hash[l] * pow_base[r - l] % MOD) % MOD
    };

    let max_len = words.iter().map(|x| x.len()).max().unwrap();
    let mut sets = vec![HashSet::new(); max_len];
    for w in words {
        let mut h = 0;
        for (j, &ch) in w.as_bytes().iter().enumerate() {
            h = (h * base + ch as i64) % MOD;
            sets[j].insert(h);
        }
    }

    // 类似 leetcode 45 跳跃游戏
    let mut result = 0;
    let mut cur_r = 0; // 当前能跳的最远位置
    let mut nxt_r = 0; // 下一次能跳的最远位置
    for i in 0..len {
        // 二分：从i起跳的最大跳跃长度
        let mut l = 0;
        let mut r = (len - i).min(max_len);
        while l < r {
            let m = (l + r) / 2;
            let h = sub_hash(i, i + m + 1);
            if sets[m].contains(&h) {
                l = m + 1;
            } else {
                r = m;
            }
        }
        nxt_r = nxt_r.max(l + i);
        if i == cur_r {
            if i == nxt_r { return -1; }
            cur_r = nxt_r;
            result += 1;
        }
    }
    result
}

fn main() {
    use leetcode::svec;
    fn test(func: fn(words: Vec<String>, target: String) -> i32) {
        assert_eq!(func(svec!["abc","aaaaa","bcdef"], String::from("aabcdabc")), 3);
        assert_eq!(func(svec!["abababab","ab"], String::from("ababaababa")), 2);
        assert_eq!(func(svec!["abcdef"], String::from("xyz")), -1);
    }
    test(min_valid_strings);
    test(min_valid_strings2);
}
