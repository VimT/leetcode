//! 优势洗牌

pub fn advantage_count(mut a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let len = a.len();
    let mut b: Vec<(i32, usize)> = b.into_iter().enumerate().map(|x| (x.1, x.0)).collect();
    b.sort_unstable();
    a.sort_unstable();
    let mut result = vec![i32::MIN; len];
    let mut i = 0;
    let mut other = vec![];
    for (b_num, bi) in b {
        while i < len && a[i] <= b_num {
            other.push(a[i]);
            i += 1;
        }
        if i == len { break; }
        result[bi] = a[i];
        i += 1;
    }
    for i in 0..len {
        if result[i] == i32::MIN {
            result[i] = other.pop().unwrap();
        }
    }
    result
}

fn main() {
    assert_eq!(advantage_count(vec![2, 0, 4, 1, 2], vec![1, 3, 0, 0, 2]), vec![2, 0, 1, 2, 4]);
    assert_eq!(advantage_count(vec![2, 7, 11, 15], vec![1, 10, 4, 11]), vec![2, 11, 7, 15]);
    assert_eq!(advantage_count(vec![12, 24, 8, 32], vec![13, 25, 32, 11]), vec![24, 32, 8, 12]);
}
