//! 处理含限制条件的好友请求


struct UnionSet {
    f: Vec<usize>,
    size: Vec<usize>,
}

impl UnionSet {
    fn new(n: usize) -> Self {
        UnionSet {
            f: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        return if self.f[x] == x {
            x
        } else {
            self.f[x] = self.find(self.f[x]);
            self.f[x]
        };
    }

    fn union(&mut self, x: usize, y: usize) {
        let mut xx = self.find(x);
        let mut yy = self.find(y);
        if self.size[xx] < self.size[yy] {
            std::mem::swap(&mut xx, &mut yy);
        }
        self.f[yy] = xx;
        self.size[xx] += self.size[yy];
    }
}

pub fn friend_requests(n: i32, restrictions: Vec<Vec<i32>>, requests: Vec<Vec<i32>>) -> Vec<bool> {
    let mut us = UnionSet::new(n as usize);
    requests.into_iter().map(|x| {
        let req = (us.find(x[0] as usize), us.find(x[1] as usize));
        if req.0 == req.1 { return true; }
        for i in &restrictions {
            let res = (us.find(i[0] as usize), us.find(i[1] as usize));
            if req == res || (res.1, res.0) == req {
                return false;
            }
        }
        us.union(req.0, req.1);
        true
    }
    ).collect()
}

fn main() {
    assert_eq!(friend_requests(3, vec![vec![0, 1]], vec![vec![0, 2], vec![2, 1]]), vec![true, false]);
    assert_eq!(friend_requests(3, vec![vec![0, 1]], vec![vec![1, 2], vec![0, 2]]), vec![true, false]);
    assert_eq!(friend_requests(5, vec![vec![0, 1], vec![1, 2], vec![2, 3]], vec![vec![0, 4], vec![1, 2], vec![3, 1], vec![3, 4]]), vec![true, false, true, false]);
}
