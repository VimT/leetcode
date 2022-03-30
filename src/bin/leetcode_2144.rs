//! 打折购买糖果的最小开销

pub fn minimum_cost(mut cost: Vec<i32>) -> i32 {
    cost.sort_unstable();
    cost.reverse();
    let mut result = 0;
    for chunk in cost.chunks(3) {
        result += chunk[0];
        if chunk.len() > 1 {
            result += chunk[1];
        }
    }
    result
}

fn main() {
    assert_eq!(minimum_cost(vec![1, 2, 3]), 5);
    assert_eq!(minimum_cost(vec![6, 5, 7, 9, 2, 2]), 23);
    assert_eq!(minimum_cost(vec![5, 5]), 10);
}
