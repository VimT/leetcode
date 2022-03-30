//! 二叉树寻路

pub fn path_in_zig_zag_tree(mut label: i32) -> Vec<i32> {
    let mut result = vec![];
    let mut level = (label as f64).log2() as u32;
    while level > 0 {
        result.push(label);
        if level & 1 == 1 {
            label = (2_i32.pow(level) + (2_i32.pow(level + 1) - 1 - label)) / 2;
        } else {
            label = label / 2;
            label = 2_i32.pow(level - 1) + (2_i32.pow(level) - 1 - label);
        }
        level -= 1;
    }
    result.push(1);
    result.reverse();
    result
}

fn main() {
    fn test(func: fn(label: i32) -> Vec<i32>) {
        assert_eq!(func(14), vec![1, 3, 4, 14]);
        assert_eq!(func(26), vec![1, 2, 6, 10, 26]);
    }
    test(path_in_zig_zag_tree);
}
