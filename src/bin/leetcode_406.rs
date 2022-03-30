//! 根据身高重建队列

pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let len = people.len();
    let mut ret: Vec<Vec<i32>> = Vec::with_capacity(len);
    people.sort_unstable_by_key(|x| (-x[0], x[1]));
    for person in people {
        ret.insert(person[1] as usize, person);
    }
    ret
}

fn main() {
    assert_eq!(reconstruct_queue(vec![vec![7, 0], vec![4, 4], vec![7, 1], vec![5, 0], vec![6, 1], vec![5, 2]]), vec![vec![5, 0], vec![7, 0], vec![5, 2], vec![6, 1], vec![4, 4], vec![7, 1]]);
    assert_eq!(reconstruct_queue(vec![vec![6, 0], vec![5, 0], vec![4, 0], vec![3, 2], vec![2, 2], vec![1, 4]]), vec![vec![4, 0], vec![5, 0], vec![2, 2], vec![3, 2], vec![1, 4], vec![6, 0]]);
}
