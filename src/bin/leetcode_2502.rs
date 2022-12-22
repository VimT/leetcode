//! 设计内存分配器

struct Allocator {
    mem: Vec<i32>,
}


impl Allocator {
    fn new(n: i32) -> Self {
        Self { mem: vec![0; n as usize] }
    }

    fn allocate(&mut self, size: i32, m_id: i32) -> i32 {
        let len = self.mem.len();
        let mut cur_size = 0;
        let mut start = len;
        for i in 0..len {
            if self.mem[i] == 0 {
                cur_size += 1;
                if start == len {
                    start = i;
                }
                if cur_size >= size {
                    for j in start..start + size as usize {
                        self.mem[j] = m_id;
                    }
                    return start as i32;
                }
            } else {
                cur_size = 0;
                start = len;
            }
        }
        -1
    }

    fn free(&mut self, m_id: i32) -> i32 {
        let mut result = 0;
        for i in 0..self.mem.len() {
            if self.mem[i] == m_id {
                self.mem[i] = 0;
                result += 1;
            }
        }
        result
    }
}

fn main() {
    let mut loc = Allocator::new(10); // 初始化一个大小为 10 的内存数组，所有内存单元都是空闲的。
    assert_eq!(loc.allocate(1, 1), 0); // 最左侧的块的第一个下标是 0 。内存数组变为 [1, , , , , , , , , ]。返回 0 。
    assert_eq!(loc.allocate(1, 2), 1); // 最左侧的块的第一个下标是 1 。内存数组变为 [1,2, , , , , , , , ]。返回 1 。
    assert_eq!(loc.allocate(1, 3), 2); // 最左侧的块的第一个下标是 2 。内存数组变为 [1,2,3, , , , , , , ]。返回 2 。
    assert_eq!(loc.free(2), 1); // 释放 mID 为 2 的所有内存单元。内存数组变为 [1, ,3, , , , , , , ] 。返回 1 ，因为只有 1 个 mID 为 2 的内存单元。
    assert_eq!(loc.allocate(3, 4), 3); // 最左侧的块的第一个下标是 3 。内存数组变为 [1, ,3,4,4,4, , , , ]。返回 3 。
    assert_eq!(loc.allocate(1, 1), 1); // 最左侧的块的第一个下标是 1 。内存数组变为 [1,1,3,4,4,4, , , , ]。返回 1 。
    assert_eq!(loc.allocate(1, 1), 6); // 最左侧的块的第一个下标是 6 。内存数组变为 [1,1,3,4,4,4,1, , , ]。返回 6 。
    assert_eq!(loc.free(1), 3); // 释放 mID 为 1 的所有内存单元。内存数组变为 [ , ,3,4,4,4, , , , ] 。返回 3 ，因为有 3 个 mID 为 1 的内存单元。
    assert_eq!(loc.allocate(10, 2), -1); // 无法找出长度为 10 个连续空闲内存单元的空闲块，所有返回 -1 。
    assert_eq!(loc.free(7), 0); // 释放 mID 为 7 的所有内存单元。内存数组保持原状，因为不存在 mID 为 7 的内存单元。返回 0 。
}
