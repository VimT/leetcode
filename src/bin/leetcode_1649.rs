//! 通过指令创建有序数组


const YU: i32 = 1e9 as i32 + 7;

struct TreeArray {
    t: Vec<i32>,
    n: i32,
}

impl TreeArray {
    fn new(n: i32) -> Self {
        TreeArray { t: vec![0; n as usize], n }
    }
    fn lowbit(x: i32) -> i32 { x & (-x) }

    fn insert(&mut self, mut x: i32) {
        while x <= self.n {
            self.t[x as usize] += 1;
            x += Self::lowbit(x);
        }
    }

    fn get_sum(&self, mut x: i32) -> i32 {
        let mut result = 0;
        while x > 0 {
            result += self.t[x as usize];
            x -= Self::lowbit(x);
        }
        result
    }
}

pub fn create_sorted_array(instructions: Vec<i32>) -> i32 {
    let max = *instructions.iter().max().unwrap();
    let mut ta = TreeArray::new(2 * (max + 1));
    let mut result = 0;
    for i in 0..instructions.len() {
        let low = ta.get_sum(instructions[i] - 1);
        let more = i as i32 - ta.get_sum(instructions[i]);
        result += low.min(more);
        result %= YU;
        ta.insert(instructions[i]);
    }
    result
}

fn main() {
    assert_eq!(create_sorted_array(vec![12, 1, 17, 21, 10, 42, 15, 40, 27, 19, 39, 34, 14, 11, 22, 43, 5, 6, 3, 30, 38, 23, 33, 41, 16, 29, 8, 49, 47, 20, 9, 50, 7, 28, 46, 31, 25, 32, 24, 36, 4, 2, 44, 37, 13, 26, 51, 48, 35, 45, 18]), 3);
    assert_eq!(create_sorted_array(vec![1, 2, 3, 6, 5, 4]), 3);
    assert_eq!(create_sorted_array(vec![1, 5, 6, 2]), 1);
    assert_eq!(create_sorted_array(vec![1, 3, 3, 3, 2, 4, 2, 1, 2]), 4);
}
