//! 最小代价构造字符串

//! 字符串多模匹配问题：AC自动机、后缀数组、字符串哈希

use std::collections::{HashMap, HashSet};

use rand::random;

use leetcode::suffix_array::SuffixArray;

/// trie + dfs 做法 O(n^2)。
pub fn minimum_cost_bad(target: String, words: Vec<String>, costs: Vec<i32>) -> i32 {
    #[derive(Debug, Clone, Default)]
    pub struct Trie {
        children: [Option<Box<Trie>>; 26],
        cost: Option<i32>,
    }
    impl Trie {
        fn insert(&mut self, s: &[u8], cost: i32) {
            let mut node = self;
            for &ch in s {
                node = node.children[(ch - b'a') as usize].get_or_insert_with(Box::<Trie>::default);
            }
            if node.cost.is_none() || node.cost.unwrap() > cost {
                node.cost = Some(cost);
            }
        }
        fn find(&self, s: &[u8]) -> Vec<(usize, i32)> {
            let mut result = vec![];
            let mut node = self;
            for (&ch, i) in s.iter().zip(1..) {
                if let Some(next) = &node.children[(ch - b'a') as usize] {
                    if let Some(cost) = next.cost {
                        result.push((i, cost));
                    }
                    node = next;
                } else {
                    break;
                }
            }
            result
        }
    }

    let t = target.as_bytes();
    let len = t.len();
    let mut trie = Trie::default();
    for (word, cost) in words.into_iter().zip(costs) {
        trie.insert(word.as_bytes(), cost);
    }
    // 从 i 开始操作的最小成本
    fn dfs(t: &[u8], trie: &Trie, i: usize, mem: &mut Vec<i32>) -> i32 {
        if i == t.len() { return 0; }
        if mem[i] != -1 { return mem[i]; }
        let mut result = i32::MAX / 2;
        for (len, cost) in trie.find(&t[i..]) {
            result = result.min(cost + dfs(t, trie, i + len, mem));
        }
        mem[i] = result;
        result
    }

    let result = dfs(t, &trie, 0, &mut vec![-1; len]);
    if result >= i32::MAX / 2 { -1 } else { result }
}

/// AC自动机：找所有可能匹配的字符串
/// 以 Trie 的结构为基础，结合 KMP 的思想 建立的自动机，用于解决多模式匹配等任务。
pub fn minimum_cost(target: String, words: Vec<String>, costs: Vec<i32>) -> i32 {
    const INF: i32 = i32::MAX / 2;

    #[derive(Default)]
    struct Node {
        len: usize,
        // 失配指针，代表下一个最大的可能匹配。
        fail: usize,
        // 指向上一个单词末尾的失配指针，直接跳过所有中间节点
        sup_fail: usize,
        next: [usize; 26],

        // 这个模式串的代价。cost == INF 表示不是单词
        cost: i32,
    }
    impl Node {
        fn new(len: usize) -> Self {
            Self { len, cost: INF, ..Default::default() }
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
        fn insert(&mut self, s: &[u8], cost: i32) {
            let mut cur = ROOT;
            let nodes = &mut self.nodes;
            for &ch in s {
                let idx = (ch - b'a') as usize;
                if nodes[cur].next[idx] == 0 {
                    nodes[cur].next[idx] = nodes.len();
                    nodes.push(Node::new(nodes[cur].len + 1));
                }
                cur = nodes[cur].next[idx];
            }
            nodes[cur].len = s.len();
            nodes[cur].cost = nodes[cur].cost.min(cost);
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
                        nodes[v].sup_fail = if nodes[nodes[v].fail].cost < INF {
                            nodes[v].fail
                        } else {
                            nodes[nodes[v].fail].sup_fail
                        };
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
    let mut ac = ACAutoMaton::with_capacity(len.max(2 * words.len()));
    for (word, cost) in words.into_iter().zip(costs) {
        ac.insert(word.as_bytes(), cost);
    }
    ac.build();

    let mut dp = vec![0; len + 1];  // dp[i] 表示 t[0..i) 的最小成本
    let mut cur = ROOT;

    // 使用 AC 自动机
    for i in 0..len {
        cur = ac.nodes[cur].next[(t[i] - b'a') as usize];
        let mut min_cost = INF;
        let mut tmp = cur;
        while tmp > ROOT {
            min_cost = min_cost.min(dp[i + 1 - ac.nodes[tmp].len] + ac.nodes[tmp].cost);
            tmp = ac.nodes[tmp].sup_fail;
        }
        dp[i + 1] = min_cost;
    }
    if dp[len] >= INF { -1 } else { dp[len] }
}


/// 后缀数组。可以快速计算出每个 words[i] 在 target 中的出现位置（匹配位置）。
pub fn minimum_cost2(target: String, words: Vec<String>, costs: Vec<i32>) -> i32 {
    let t = target.as_bytes();
    let len = t.len();
    let sa = SuffixArray::new(t);
    let mut min_cost: HashMap<String, i32> = HashMap::new();
    for (word, cost) in words.into_iter().zip(costs) {
        min_cost.entry(word).and_modify(|x| *x = (*x).min(cost)).or_insert(cost);
    }

    let mut from = vec![vec![]; len + 1];
    for (word, cost) in min_cost {
        for l in sa.look_up(word.as_bytes()) {
            let r = l + word.len();
            from[r].push((l, cost));
        }
    }
    let mut dp = vec![0; len + 1];
    const INF: i32 = i32::MAX / 2;
    for i in 1..=len {
        dp[i] = INF;
        for &(l, cost) in &from[i] {
            dp[i] = dp[i].min(dp[l] + cost);
        }
    }
    if dp[len] >= INF { -1 } else { dp[len] }
}

/// 字符串哈希 + DP
/// 设 L 是 words 中所有字符串的长度之和，那么 words 中至多有 O( sqrt(L) ) 个长度不同的字符串。
pub fn minimum_cost3(target: String, words: Vec<String>, costs: Vec<i32>) -> i32 {
    let base: u64 = 9e8 as u64 - random::<u64>() % 1e8 as u64;
    const MOD: u64 = 1_000_000_007;
    let t = target.as_bytes();
    let len = t.len();
    let mut pow_base = vec![1; len + 1]; // base ^ i
    let mut pre_hash = vec![0; len + 1]; // pre_hash[i] 表示 t[0..i) 的哈希值
    for (i, &ch) in t.iter().enumerate() {
        pow_base[i + 1] = pow_base[i] * base % MOD;
        pre_hash[i + 1] = (pre_hash[i] * base + ch as u64) % MOD;
    }
    // 计算子串 [l, r) 的哈希值
    let sub_hash = |l: usize, r: usize| -> u64 {
        (pre_hash[r] + MOD - pre_hash[l] * pow_base[r - l] % MOD) % MOD
    };
    let mut word_hash: HashMap<u64, i32> = HashMap::new(); // word 的 hash 值 -> word 的成本
    let mut lens: HashSet<usize> = HashSet::new();
    for (word, cost) in words.into_iter().zip(costs) {
        let mut hash = 0;
        for &ch in word.as_bytes() {
            hash = (hash * base + ch as u64) % MOD;
        }
        word_hash.entry(hash).and_modify(|x| *x = (*x).min(cost)).or_insert(cost);
        lens.insert(word.len());
    }
    let mut lens = lens.into_iter().collect::<Vec<_>>();
    lens.sort_unstable();

    let mut dp = vec![0; len + 1];  // dp[i] 表示 t[0..i) 的最小成本
    const INF: i32 = i32::MAX / 2;
    for i in 1..=len {
        dp[i] = INF;
        for &sz in &lens {
            if sz > i { break; }
            if let Some(&cost) = word_hash.get(&(sub_hash(i - sz, i))) {
                dp[i] = dp[i].min(dp[i - sz] + cost);
            }
        }
    }
    if dp[len] >= INF { -1 } else { dp[len] }
}

fn main() {
    use leetcode::svec;
    fn test(func: fn(target: String, words: Vec<String>, costs: Vec<i32>) -> i32) {
        assert_eq!(func(String::from("a".repeat(50000)), svec!["a", "a".repeat(49999)], vec![1, 1]), 2);  // 特殊用例，会让 做法1 TLE
        assert_eq!(func(String::from("abcdef"), svec!["abdef","abc","d","def","ef"], vec![100, 1, 1, 10, 5]), 7);
        assert_eq!(func(String::from("aaaa"), svec!["z","zz","zzz"], vec![1, 10, 100]), -1);
    }
    test(minimum_cost);
    test(minimum_cost2);
    test(minimum_cost3);
    // test(minimum_cost_bad);
}
