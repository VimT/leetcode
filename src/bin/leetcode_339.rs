//! 嵌套列表权重和

use leetcode::nested_int;
use leetcode::nested_integer::NestedInteger;

pub fn depth_sum(nested_list: Vec<NestedInteger>) -> i32 {
    fn dfs(nested_list: NestedInteger, cur_depth: i32) -> i32 {
        match nested_list {
            NestedInteger::Int(num) => { num * cur_depth }
            NestedInteger::List(nl) => {
                let mut result = 0;
                for item in nl {
                    result += dfs(item, cur_depth + 1);
                }
                result
            }
        }
    }
    nested_list.into_iter().map(|x| dfs(x, 1)).sum()
}

fn main() {
    fn test(func: fn(nested_list: Vec<NestedInteger>) -> i32) {
        assert_eq!(func(nested_int![[1,1],2,[1,1]]), 10);
        assert_eq!(func(nested_int![1,[4,[6]]]), 27);
        assert_eq!(func(nested_int![0]), 0);
    }
    test(depth_sum);
}
