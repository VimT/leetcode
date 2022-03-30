//! 加油站

pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let len = gas.len();
    let mut need = 0;
    let mut current = 0;
    for i in 0..len {
        current += gas[i];
        current -= cost[i];
        if current < 0 {
            need += -current;
            current = 0;
        }
    }
    if need <= 0 {
        return 0;
    }
    for i in (0..len).rev() {
        need += cost[i];
        need -= gas[i];
        if need <= 0 {
            return i as i32;
        }
    }
    -1
}


fn main() {
    assert_eq!(can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]), 3);
    assert_eq!(can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]), -1);
}
