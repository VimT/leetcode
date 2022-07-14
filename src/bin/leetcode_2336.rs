//! 无限集中的最小数字


use std::collections::BTreeSet;

struct SmallestInfiniteSet {
    new_add: BTreeSet<i32>,
    start_num: i32,
}


impl SmallestInfiniteSet {
    fn new() -> Self {
        return SmallestInfiniteSet {
            new_add: BTreeSet::new(),
            start_num: 1,
        };
    }

    fn pop_smallest(&mut self) -> i32 {
        if self.new_add.is_empty() {
            self.start_num += 1;
            self.start_num - 1
        } else {
            let num = *self.new_add.range(..).next().unwrap();
            self.new_add.remove(&num);
            num
        }
    }

    fn add_back(&mut self, num: i32) {
        if num >= self.start_num {
            return;
        }
        self.new_add.insert(num);
    }
}

fn main() {
    let mut sis = SmallestInfiniteSet::new();
    sis.add_back(2);    // 2 已经在集合中，所以不做任何变更。
    assert_eq!(sis.pop_smallest(), 1); // 返回 1 ，因为 1 是最小的整数，并将其从集合中移除。
    assert_eq!(sis.pop_smallest(), 2); // 返回 2 ，并将其从集合中移除。
    assert_eq!(sis.pop_smallest(), 3); // 返回 3 ，并将其从集合中移除。
    sis.add_back(1);    // 将 1 添加到该集合中。
    assert_eq!(sis.pop_smallest(), 1); // 返回 1 ，因为 1 在上一步中被添加到集合中， 且 1 是最小的整数，并将其从集合中移除。
    assert_eq!(sis.pop_smallest(), 4); // 返回 4 ，并将其从集合中移除。
    assert_eq!(sis.pop_smallest(), 5); // 返回 5 ，并将其从集合中移除。
    sis.add_back(2);
    sis.add_back(3);
    assert_eq!(sis.pop_smallest(), 2);
    assert_eq!(sis.pop_smallest(), 3);
}
