//! 完成所有任务的最少时间


/// 按r排序，每个task贪心选最右边的
/// 和 https://leetcode.cn/problems/t3fKg1/ 一样
pub fn find_minimum_time(mut tasks: Vec<Vec<i32>>) -> i32 {
    tasks.sort_unstable_by_key(|x| x[1]);
    let mut ok = vec![0; 2001];
    for task in tasks {
        let (l, r, mut c) = (task[0] as usize, task[1] as usize, task[2]);
        for i in l..=r {
            c -= ok[i];
        }
        let mut i = r;
        while c > 0 {
            if ok[i as usize] == 0 {
                ok[i as usize] = 1;
                c -= 1;
            }
            i -= 1;
        }
    }
    ok.into_iter().sum()
}

fn main() {
    fn test(func: fn(tasks: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![2, 3, 1], vec![4, 5, 1], vec![1, 5, 2]]), 2);
        assert_eq!(func(vec![vec![1, 3, 2], vec![2, 5, 3], vec![5, 6, 2]]), 4);
    }
    test(find_minimum_time);
}
