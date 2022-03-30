//! 三等分

/// 1的个数必须是3的整数倍 将1等分成3份 再根据最后一份末尾0的个数推导 前两份的最后一个坐标，最后判断3份去掉前导0 是不是一样
pub fn three_equal_parts(arr: Vec<i32>) -> Vec<i32> {
    let mut one = 0;
    let len = arr.len();
    for &num in &arr {
        if num == 1 { one += 1; }
    }
    if one == 0 { return vec![0, 2]; }
    if one % 3 == 0 {
        let every = one / 3;
        let mut i = 0;
        let mut cnt = every;
        while cnt > 0 {
            if arr[i] == 1 { cnt -= 1; }
            i += 1;
        }
        let mut i1 = i - 1;
        cnt = every;
        while cnt > 0 {
            if arr[i] == 1 { cnt -= 1; }
            i += 1;
        }
        let mut i2 = i;
        let mut tail_zero = 0;
        for i in (0..len).rev() {
            if arr[i] == 0 { tail_zero += 1; } else { break; }
        }
        i1 += tail_zero;
        i2 += tail_zero;
        let mut i1_start = 0;
        let mut i2_start = i1 + 1;
        let mut i3_start = i2;
        while i1_start < len && arr[i1_start] == 0 { i1_start += 1; }
        while i2_start < len && arr[i2_start] == 0 { i2_start += 1; }
        while i3_start < len && arr[i3_start] == 0 { i3_start += 1; }
        if i1_start <= i1 && i1 < i2_start && i2_start < i2 && i2 <= i3_start && i3_start < len && arr[i1_start..=i1] == arr[i2_start..i2] && arr[i1_start..=i1] == arr[i3_start..] {
            return vec![i1 as i32, i2 as i32];
        }
    }
    vec![-1, -1]
}

fn main() {
    assert_eq!(three_equal_parts(vec![1, 1, 1]), vec![0, 2]);
    assert_eq!(three_equal_parts(vec![0, 0, 0]), vec![0, 2]);
    assert_eq!(three_equal_parts(vec![1, 0, 1, 0, 1]), vec![0, 3]);
    assert_eq!(three_equal_parts(vec![1, 1, 0, 1, 1]), vec![-1, -1]);
    assert_eq!(three_equal_parts(vec![1, 1, 0, 0, 1]), vec![0, 2]);
}
