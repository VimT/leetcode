//! 电话目录管理系统

/// 全O(1)，slots存指向下一个可用数字
struct PhoneDirectory {
    slots: Vec<i32>,
    avail: i32,
}


impl PhoneDirectory {
    fn new(max_numbers: i32) -> Self {
        let mut slots: Vec<i32> = (1..=max_numbers).collect();
        *slots.last_mut().unwrap() = -1;
        Self { slots, avail: 0 }
    }

    fn get(&mut self) -> i32 {
        if self.avail == -1 { return -1; }
        let rtn = self.avail;
        self.avail = self.slots[rtn as usize];
        // occupied
        self.slots[rtn as usize] = -2;
        return rtn;
    }

    fn check(&self, number: i32) -> bool {
        self.slots[number as usize] != -2
    }

    fn release(&mut self, number: i32) {
        if self.slots[number as usize] == -2 {
            if self.avail == -1 {
                self.avail = number;
                self.slots[number as usize] = -1;
            } else {
                let old = self.slots[self.avail as usize];
                self.slots[self.avail as usize] = number;
                self.slots[number as usize] = old;
            }
        }
    }
}


fn main() {
    let mut directory = PhoneDirectory::new(3);
    assert_eq!(directory.get(), 0);
    assert_eq!(directory.get(), 1);
    assert_eq!(directory.check(2), true);
    assert_eq!(directory.get(), 2);
    assert_eq!(directory.check(2), false);
    directory.release(2);
    assert_eq!(directory.check(2), true);
}
