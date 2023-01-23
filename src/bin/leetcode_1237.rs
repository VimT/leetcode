//! 找出给定方程的正整数解

pub struct CustomFunction {
    def: fn(i32, i32) -> i32,
}

impl CustomFunction {
    pub fn f(&self, x: i32, y: i32) -> i32 {
        (self.def)(x, y)
    }
}

pub fn find_solution(customfunction: &CustomFunction, z: i32) -> Vec<Vec<i32>> {
    let mut result = vec![];
    for x in 1..=1000 {
        for y in 1..=1000 {
            let g = customfunction.f(x, y);
            if g == z { result.push(vec![x, y]); }
            if g >= z { break; }
        }
    }
    result
}

fn main() {
    fn test(func: fn(customfunction: &CustomFunction, z: i32) -> Vec<Vec<i32>>) {
        assert_eq!(func(&CustomFunction { def: |a, b| a + b }, 5), vec![vec![1, 4], vec![2, 3], vec![3, 2], vec![4, 1]]);
        assert_eq!(func(&CustomFunction { def: |a, b| a * b }, 5), vec![vec![1, 5], vec![5, 1]]);
    }
    test(find_solution);
}
