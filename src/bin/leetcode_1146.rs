//! 快照数组

struct SnapshotArray {
    m: Vec<Vec<(i32, i32)>>,
    cur: i32,
}


impl SnapshotArray {
    fn new(len: i32) -> Self {
        Self { m: vec![vec![]; len as usize], cur: 0 }
    }

    fn set(&mut self, index: i32, val: i32) {
        if let Some(last) = self.m[index as usize].last_mut() {
            if last.0 == self.cur {
                last.1 = val;
                return;
            }
        }
        self.m[index as usize].push((self.cur, val));
    }

    fn snap(&mut self) -> i32 {
        self.cur += 1;
        self.cur - 1
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let idx = match self.m[index as usize].binary_search_by_key(&snap_id, |x| x.0) {
            Ok(idx) => {
                idx
            }
            Err(idx) => {
                if idx == 0 { return 0; }
                idx - 1
            }
        };
        self.m[index as usize][idx].1
    }
}


fn main() {
    let mut sa = SnapshotArray::new(3); // 初始化一个长度为 3 的快照数组
    sa.set(0, 5);  // 令 array[0] = 5
    assert_eq!(sa.snap(), 0);  // 获取快照，返回 snap_id = 0
    sa.set(0, 6);
    assert_eq!(sa.get(0, 0), 5);  // 获取 snap_id = 0 的快照中 array[0] 的值，返回 5
    assert_eq!(sa.get(0, 1), 6);
    assert_eq!(sa.get(0, 2), 6);

    sa = SnapshotArray::new(1);
    sa.set(0, 15);
    sa.snap();
    sa.snap();
    sa.snap();
    assert_eq!(sa.get(0, 2), 15);
}
