//! 掷骰子模拟

pub fn die_simulator(n: i32, roll_max: Vec<i32>) -> i32 {

}

fn main() {
fn test(func: fn(n: i32, roll_max: Vec<i32>) -> i32 ) {
assert_eq!(func(2,vec![1,1,2,2,2,3]), 34);
assert_eq!(func(2,vec![1,1,1,1,1,1]), 30);
assert_eq!(func(3,vec![1,1,1,2,2,3]), 181);
}
test(die_simulator);
}
