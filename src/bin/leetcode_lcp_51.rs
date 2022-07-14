//! 2022春季编程大赛：烹饪料理

/// dfs做法，或者 二进制遍历 所有料理
pub fn perfect_menu(mut materials: Vec<i32>, cookbooks: Vec<Vec<i32>>, attribute: Vec<Vec<i32>>, limit: i32) -> i32 {
    fn dfs(left: &mut Vec<i32>, cookbooks: &Vec<Vec<i32>>, attribute: &Vec<Vec<i32>>, limit: i32, i: usize, result: &mut i32, cur_bao: i32, cur_mei: i32) {
        if i == attribute.len() {
            if cur_bao >= limit {
                *result = (*result).max(cur_mei);
            }
            return
        }
        let mut ok = true;
        for j in 0..left.len() {
            if left[j] < cookbooks[i][j] {
                ok = false;
                break;
            }
        }
        if ok {
            for j in 0..left.len() {
                left[j] -= cookbooks[i][j];
            }
            dfs(left, cookbooks, attribute, limit, i + 1, result, cur_bao + attribute[i][1], cur_mei + attribute[i][0]);
            for j in 0..left.len() {
                left[j] += cookbooks[i][j];
            }
        }
        dfs(left, cookbooks, attribute, limit, i + 1, result, cur_bao, cur_mei)
    }
    let mut result = -1;
    dfs(&mut materials, &cookbooks, &attribute, limit, 0, &mut result, 0, 0);
    result
}

fn main() {
    assert_eq!(perfect_menu(vec![3, 2, 4, 1, 2], vec![vec![1, 1, 0, 1, 2], vec![2, 1, 4, 0, 0], vec![3, 2, 4, 1, 0]], vec![vec![3, 2], vec![2, 4], vec![7, 6]], 5), 7);
    assert_eq!(perfect_menu(vec![10, 10, 10, 10, 10], vec![vec![1, 1, 1, 1, 1], vec![3, 3, 3, 3, 3], vec![10, 10, 10, 10, 10]], vec![vec![5, 5], vec![6, 6], vec![10, 10]], 1), 11);
}
