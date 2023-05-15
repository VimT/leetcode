//! 与非的谜题


struct SegmentTree {
    tree: Vec<[usize; 2]>,
    k: i32,
}

impl SegmentTree {
    fn push_up(&mut self, p: usize) {
        let mut t = [0; 2];
        for i in 0..self.k {
            t[0] |= self.tree[p << 1 | 1][self.tree[p << 1][0] >> i & 1] & (1 << i);
            t[1] |= self.tree[p << 1 | 1][self.tree[p << 1][1] >> i & 1] & (1 << i);
        }
        self.tree[p] = t;
    }

    fn build(&mut self, a: &Vec<i32>, k: i32, p: usize, l: usize, r: usize) {
        self.tree[p][0] = (1 << k) - 1;
        if l == r {
            self.tree[p][1] = self.tree[p][0] ^ a[l - 1] as usize;
            return;
        }
        let mid = (l + r) / 2;
        self.build(a, k, p << 1, l, mid);
        self.build(a, k, p << 1 | 1, mid + 1, r);
        self.push_up(p)
    }

    fn update(&mut self, p: usize, l: usize, r: usize, idx: usize, val: usize) {
        if l == r {
            self.tree[p][1] = self.tree[p][0] ^ val;
            return;
        }
        let mid = (l + r) / 2;
        if idx < mid { self.update(p << 1, l, mid, idx, val); }
        if idx >= mid { self.update(p << 1 | 1, mid + 1, r, idx, val); }
        self.push_up(p);
    }
}

pub fn get_nand_result(k: i32, arr: Vec<i32>, operations: Vec<Vec<i32>>) -> i32 {
    let len = arr.len();
    let mut tree = SegmentTree { tree: vec![[0; 2]; len << 2], k };
    tree.build(&arr, k, 1, 1, len);
    let mut result = 0;
    for operation in operations {
        let op = operation[0];
        let x = operation[1] as usize;
        let y = operation[2] as usize;
        if op == 0 {
            tree.update(1, 1, len, x, y);
            continue;
        }
        for j in 0..k {
            let y1 = ((y >> j) & 1) as usize;
            let y2 = (tree.tree[1][y1] >> j & 1) as usize;
            let res = if y1 == y2 {
                y1
            } else if x == 1 || tree.tree[1][y2] >> j & 1 == y2 {
                y2
            } else {
                y1 ^ (x & 1)
            };
            result ^= res << j;
        }
    }
    result as i32
}


fn main() {
    fn test(func: fn(k: i32, arr: Vec<i32>, operations: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(3, vec![1, 2], vec![vec![1, 2, 3], vec![0, 0, 3], vec![1, 2, 2]]), 2);
        assert_eq!(func(4, vec![4, 6, 4, 7, 10, 9, 11], vec![vec![1, 5, 7], vec![1, 7, 14], vec![0, 6, 7], vec![1, 6, 5]]), 9);
    }
    test(get_nand_result);
}
