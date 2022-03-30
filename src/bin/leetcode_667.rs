//! 优美的排列 II

pub fn construct_array(n: i32, k: i32) -> Vec<i32> {
    let mut result = Vec::with_capacity(n as usize);
    let mut last = 1;
    let mut add = true;
    result.push(1);
    for i in (1..=k).rev() {
        if add {
            result.push(last + i);
            last += i;
            add = false;
        } else {
            result.push(last - i);
            last -= i;
            add = true;
        }
    }
    for i in k + 2..=n {
        result.push(i);
    }
    result
}

fn main() {
    for i in 1..10 {
        println!("{:?}", construct_array(10, i));
    }
    assert_eq!(construct_array(3, 1), vec![1, 2, 3]);
    assert_eq!(construct_array(3, 2), vec![1, 3, 2]);
}
