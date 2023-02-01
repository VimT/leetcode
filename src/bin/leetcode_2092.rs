//! 找出知晓秘密的所有专家

use leetcode::union_set::UnionSet;
use leetcode::unorder;

pub fn find_all_people(n: i32, mut meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
    meetings.sort_unstable_by_key(|x| x[2]);
    let mut us = UnionSet::new(n as usize);
    let mut ok = vec![false; n as usize];
    ok[0] = true;
    ok[first_person as usize] = true;
    us.union(first_person as usize, 0);
    let mut i = 0;
    let len = meetings.len();
    while i < len {
        let mut j = i + 1;
        while j < len && meetings[j][2] == meetings[i][2] { j += 1; }
        for x in i..j {
            us.union(meetings[x][0] as usize, meetings[x][1] as usize);
        }
        for x in i..j {
            if us.find(meetings[x][0] as usize) != us.find(0) {
                us.isolate(meetings[x][0] as usize);
                us.isolate(meetings[x][1] as usize);
            } else {
                ok[meetings[x][0] as usize] = true;
                ok[meetings[x][1] as usize] = true;
            }
        }
        i = j;
    }
    (0..n).filter(|x| ok[*x as usize]).collect()
}


fn main() {
    assert_eq!(unorder(find_all_people(6, vec![vec![1, 2, 5], vec![2, 3, 8], vec![1, 5, 10]], 1)), unorder(vec![0, 1, 2, 3, 5]));
    assert_eq!(unorder(find_all_people(4, vec![vec![3, 1, 3], vec![1, 2, 2], vec![0, 3, 3]], 3)), unorder(vec![0, 1, 3]));
    assert_eq!(unorder(find_all_people(5, vec![vec![3, 4, 2], vec![1, 2, 1], vec![2, 3, 1]], 1)), unorder(vec![0, 1, 2, 3, 4]));
}
