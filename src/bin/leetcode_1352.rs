//! 最后 K 个数的乘积


struct ProductOfNumbers {
    pre: Vec<i32>,
}

impl ProductOfNumbers {
    fn new() -> Self {
        Self { pre: vec![1] }
    }

    fn add(&mut self, num: i32) {
        if num == 0 {
            unsafe { self.pre.set_len(1); }
            return;
        }
        let last = self.pre.last().cloned().unwrap();
        self.pre.push(num * last);
    }

    fn get_product(&self, k: i32) -> i32 {
        if self.pre.len() <= k as usize {
            return 0;
        }
        let len = self.pre.len();
        self.pre[len - 1] / self.pre[len - 1 - k as usize]
    }
}

fn main() {
    let mut pn = ProductOfNumbers::new();
    pn.add(3);        // [3]
    pn.add(0);        // [3,0]
    pn.add(2);        // [3,0,2]
    pn.add(5);        // [3,0,2,5]
    pn.add(4);        // [3,0,2,5,4]
    assert_eq!(pn.get_product(2), 20); // 返回 20 。最后 2 个数字的乘积是 5 * 4 = 20
    assert_eq!(pn.get_product(3), 40); // 返回 40 。最后 3 个数字的乘积是 2 * 5 * 4 = 40
    assert_eq!(pn.get_product(4), 0); // 返回  0 。最后 4 个数字的乘积是 0 * 2 * 5 * 4 = 0
    pn.add(8);        // [3,0,2,5,4,8]
    assert_eq!(pn.get_product(2), 32); // 返回 32 。最后 2 个数字的乘积是 4 * 8 = 32
}
