//! 柠檬水找零

pub fn lemonade_change(bills: Vec<i32>) -> bool {
    let mut c5 = 0;
    let mut c10 = 0;
    for bill in bills {
        match bill {
            5 => c5 += 1,
            10 => {
                c10 += 1;
                if c5 == 0 { return false; }
                c5 -= 1;
            }
            20 => {
                if c10 >= 1 && c5 >= 1 {
                    c10 -= 1;
                    c5 -= 1;
                } else if c5 >= 3 {
                    c5 -= 3;
                } else {
                    return false;
                }
            }
            _ => unreachable!()
        }
    }
    true
}

fn main() {
    assert_eq!(lemonade_change(vec![5, 5, 5, 10, 20]), true);
    assert_eq!(lemonade_change(vec![5, 5, 10, 10, 20]), false);
}
