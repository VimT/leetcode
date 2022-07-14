//! 加权嵌套序列和 II

use leetcode::nested_int;
use leetcode::nested_integer::NestedInteger;

pub fn depth_sum_inverse(nested_list: Vec<NestedInteger>) -> i32 {
    fn max_depth(nested: &Vec<NestedInteger>) -> i32 {
        let mut result = 0;
        for item in nested {
            match item {
                NestedInteger::Int(_) => {}
                NestedInteger::List(v) => {
                    result = result.max(max_depth(v));
                }
            }
        }
        result + 1
    }
    fn dfs(nested_list: NestedInteger, cur_depth: i32) -> i32 {
        match nested_list {
            NestedInteger::Int(num) => { num * cur_depth }
            NestedInteger::List(nl) => {
                let mut result = 0;
                for item in nl {
                    result += dfs(item, cur_depth - 1);
                }
                result
            }
        }
    }
    let md = max_depth(&nested_list);
    nested_list.into_iter().map(|x| dfs(x, md)).sum()
}

fn main() {
    fn test(func: fn(nested_list: Vec<NestedInteger>) -> i32) {
        assert_eq!(func(nested_int![[1,1],2,[1,1]]), 8);
        assert_eq!(func(nested_int![1,[4,[6]]]), 17);
    }
    test(depth_sum_inverse);
}
