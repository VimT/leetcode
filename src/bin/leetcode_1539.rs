//! 第 k 个缺失的正整数

pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
    let mut expect = 1;
    let mut leak = 0;
    for num in arr {
        if num > expect {
            leak += num - expect;
            if leak >= k {
                return num - (leak - k + 1);
            }
            expect = num + 1;
        } else if num == expect {
            expect += 1;
        } else {
            unreachable!()
        }
    }
    expect + (k - leak - 1)
}

fn main() {
    fn test(func: fn(arr: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![2, 3, 4, 7, 11], 5), 9);
        assert_eq!(func(vec![1, 2, 3, 4], 2), 6);
    }
    test(find_kth_positive);
}
