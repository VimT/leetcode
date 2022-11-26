//! 最长上传前缀

use std::collections::BTreeSet;

struct LUPrefix {
    n: i32,
    set: BTreeSet<i32>,
}


impl LUPrefix {
    fn new(n: i32) -> Self {
        Self { n, set: (1..=n).collect() }
    }

    fn upload(&mut self, video: i32) {
        self.set.remove(&video);
    }

    fn longest(&self) -> i32 {
        self.set.range(..).next().map(|x| *x - 1).unwrap_or(self.n)
    }
}


fn main() {
    let mut server = LUPrefix::new(4);   // 初始化 4个视频的上传流
    server.upload(3);                    // 上传视频 3 。
    assert_eq!(server.longest(), 0);                    // 由于视频 1 还没有被上传，最长上传前缀是 0 。
    server.upload(1);                    // 上传视频 1 。
    assert_eq!(server.longest(), 1);                    // 前缀 [1] 是最长上传前缀，所以我们返回 1 。
    server.upload(2);                    // 上传视频 2 。
    assert_eq!(server.longest(), 3);                    // 前缀 [1,2,3] 是最长上传前缀，所以我们返回 3 。
}
