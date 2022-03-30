//! 验证栈序列

pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
    let mut s = vec![];
    let mut push_idx = 0;
    let mut pop_idx = 0;
    let len = pushed.len();
    while pop_idx < len {
        if !s.is_empty() && *s.last().unwrap() == popped[pop_idx] {
            s.pop();
            pop_idx += 1;
            continue;
        }
        while push_idx < len && pushed[push_idx] != popped[pop_idx] {
            s.push(pushed[push_idx]);
            push_idx += 1;
        }
        if push_idx == len {
            return false;
        }
        push_idx += 1;
        pop_idx += 1;
    }
    s.is_empty()
}

fn main() {
    assert_eq!(validate_stack_sequences(vec![1, 2, 3, 4, 5], vec![4, 5, 3, 2, 1]), true);
    assert_eq!(validate_stack_sequences(vec![1, 2, 3, 4, 5], vec![4, 3, 5, 1, 2]), false);
}
