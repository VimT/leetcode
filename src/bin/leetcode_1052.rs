//! 爱生气的书店老板

pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
    let len = customers.len();
    let mut cur = 0;
    for i in 0..len {
        if grumpy[i] == 0 {
            cur += customers[i];
        }
    }
    let minutes = minutes as usize;
    for i in 0..minutes {
        if grumpy[i] == 1 {
            cur += customers[i];
        }
    }
    let mut result = cur;
    for i in minutes..len {
        if grumpy[i] == 1 {
            cur += customers[i];
        }
        if grumpy[i - minutes] == 1 {
            cur -= customers[i - minutes];
        }
        result = result.max(cur);
    }
    result
}

fn main() {
    assert_eq!(max_satisfied(vec![1, 0, 1, 2, 1, 1, 7, 5], vec![0, 1, 0, 1, 0, 1, 0, 1], 3), 16);
    assert_eq!(max_satisfied(vec![1], vec![0], 1), 1);
}
