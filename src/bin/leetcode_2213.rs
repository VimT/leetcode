//! 由单个字符重复的最长子字符串

#[derive(Default, Clone)]
struct NodeInfo {
    start_ch: u8,
    start_len: i32,
    end_ch: u8,
    end_len: i32,
    most_len: i32,
    len: i32,
}

impl NodeInfo {
    fn new(ch: u8) -> Self {
        NodeInfo {
            start_ch: ch,
            start_len: 1,
            end_ch: ch,
            end_len: 1,
            most_len: 1,
            len: 1,
        }
    }
    fn merge(&self, other: &NodeInfo) -> NodeInfo {
        let mut result = NodeInfo {
            start_ch: self.start_ch,
            start_len: self.start_len,
            end_ch: other.end_ch,
            end_len: other.end_len,
            most_len: self.most_len.max(other.most_len),
            len: self.len + other.len,
        };
        if self.end_ch == other.start_ch {
            if self.start_len == self.len {
                result.start_len += other.start_len;
            }
            if other.end_len == other.len {
                result.end_len += self.end_len;
            }
            result.most_len = result.most_len.max(self.end_len + other.start_len);
        }
        result
    }
}

struct SegmentTree {
    tree: Vec<NodeInfo>,
    len: usize,
}

impl SegmentTree {
    fn new(s: &Vec<u8>) -> Self {
        let len = s.len();
        let mut st = Self { tree: vec![NodeInfo::default(); len * 4], len };
        st.build(s, 0, len - 1, 1);
        st
    }

    fn build(&mut self, str: &Vec<u8>, s: usize, t: usize, p: usize) {
        if s == t {
            self.tree[p] = NodeInfo::new(str[s]);
            return;
        }
        let mid = (s + t) / 2;
        self.build(str, s, mid, p * 2);
        self.build(str, mid + 1, t, p * 2 + 1);
        self.tree[p] = self.tree[p * 2].merge(&self.tree[p * 2 + 1]);
    }

    fn _update(&mut self, pos: usize, s: usize, t: usize, p: usize, ch: u8) {
        if s == t && s == pos {
            self.tree[p] = NodeInfo::new(ch);
            return;
        }
        let mid = (s + t) / 2;
        if pos <= mid { self._update(pos, s, mid, p * 2, ch); } else { self._update(pos, mid + 1, t, p * 2 + 1, ch); }
        self.tree[p] = self.tree[p * 2].merge(&self.tree[p * 2 + 1]);
    }

    fn update(&mut self, pos: usize, ch: u8) {
        self._update(pos, 0, self.len - 1, 1, ch)
    }

    fn query(&mut self) -> i32 {
        self.tree[1].most_len
    }
}

/// 精髓在线段树保存什么内容，和怎么merge
pub fn longest_repeating(s: String, query_characters: String, query_indices: Vec<i32>) -> Vec<i32> {
    let s = s.into_bytes();
    let mut st = SegmentTree::new(&s);
    let mut result = Vec::with_capacity(query_indices.len());
    for (ch, idx) in query_characters.into_bytes().into_iter().zip(query_indices) {
        st.update(idx as usize, ch);
        result.push(st.query());
    }
    result
}

fn main() {
    assert_eq!(longest_repeating(String::from("babacc"), String::from("bcb"), vec![1, 3, 3]), vec![3, 3, 4]);
    assert_eq!(longest_repeating(String::from("abyzz"), String::from("aa"), vec![2, 1]), vec![2, 3]);
}
