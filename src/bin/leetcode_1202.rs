//! 交换字符串中的元素


use std::mem;

struct SetUnion {
    f: Vec<usize>,
    rank: Vec<usize>,
}

impl SetUnion {
    fn new(n: usize) -> Self {
        SetUnion {
            f: (0..n).collect(),
            rank: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        return if self.f[x] == x { x } else {
            self.f[x] = self.find(self.f[x]);
            self.f[x]
        };
    }

    fn union(&mut self, x: usize, y: usize) {
        let mut fx = self.find(x);
        let mut fy = self.find(y);
        if fx == fy { return; }
        if self.rank[fx] < self.rank[fy] {
            mem::swap(&mut fx, &mut fy)
        }
        self.rank[fx] += self.rank[fy];
        self.f[fy] = fx;
    }
}

pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
    let len = s.len();
    let mut su = SetUnion::new(len);
    for i in &pairs {
        su.union(i[0] as usize, i[1] as usize);
    }
    let mut s = s.into_bytes();
    let mut mp = vec![vec![]; len];
    for i in 0..len {
        mp[su.find(i)].push(s[i]);
    }
    for v in &mut mp {
        v.sort_unstable_by(|a, b| b.cmp(a));
    }

    for i in 0..len {
        let x = su.find(i);
        s[i] = mp[x].pop().unwrap();
    }

    unsafe { return String::from_utf8_unchecked(s); }
}


fn main() {
    fn test(func: fn(s: String, pairs: Vec<Vec<i32>>) -> String) {
        assert_eq!(func(String::from("dcab"), vec![vec![0, 3], vec![1, 2]]), String::from("bacd"));
        assert_eq!(func(String::from("dcab"), vec![vec![0, 3], vec![1, 2], vec![0, 2]]), String::from("abcd"));
        assert_eq!(func(String::from("cba"), vec![vec![0, 1], vec![1, 2]]), String::from("abc"));
    }
    test(smallest_string_with_swaps);
}
