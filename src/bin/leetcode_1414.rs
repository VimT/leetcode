//! 和为 K 的最少斐波那契数字数目

static F: [i32; 45] = [1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987,
    1597, 2584, 4181, 6765, 10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811,
    514229, 832040, 1346269, 2178309, 3524578, 5702887, 9227465, 14930352, 24157817,
    39088169, 63245986, 102334155, 165580141, 267914296, 433494437, 701408733, 1134903170,
    1836311903];

pub fn find_min_fibonacci_numbers(k: i32) -> i32 {
    match F.binary_search(&k) {
        Ok(_) => 1,
        Err(v) => {
            find_min_fibonacci_numbers(k - F[v - 1]) + 1
        }
    }
}

fn main() {
    assert_eq!(find_min_fibonacci_numbers(7), 2);
    assert_eq!(find_min_fibonacci_numbers(10), 2);
    assert_eq!(find_min_fibonacci_numbers(19), 3);
}
