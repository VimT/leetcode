//! 水果成篮

pub fn total_fruit(fruits: Vec<i32>) -> i32 {
    let len = fruits.len();
    let mut i = 0;
    let mut result = 0;
    while i < len {
        let pin1 = fruits[i];
        let mut ie = i + 1;
        while ie < len && fruits[ie] == pin1 { ie += 1; }
        if ie == len {
            result = result.max(ie - i);
            break;
        }
        let pin2 = fruits[ie];
        let mut je = ie + 1;
        while je < len && (fruits[je] == pin1 || fruits[je] == pin2) { je += 1; }
        result = result.max(je - i);
        if je == len { break; }
        i = je - 1;
        let pin = fruits[i];
        while fruits[i] == pin {
            i -= 1;
            if i == 0 { break; }
        }
        if fruits[i] != pin { i += 1; }
    }
    result as i32
}

fn main() {
    assert_eq!(total_fruit(vec![1, 2, 1]), 3);
    assert_eq!(total_fruit(vec![0, 1, 2, 2]), 3);
    assert_eq!(total_fruit(vec![1, 2, 3, 2, 2]), 4);
}
