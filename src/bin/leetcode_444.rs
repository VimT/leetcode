//! 序列重建

pub fn sequence_reconstruction(nums: Vec<i32>, sequences: Vec<Vec<i32>>) -> bool {
    let len = nums.len();
    let mut exist = false;
    let mut pos = vec![0; len + 1];
    let mut flag = vec![false; len + 1];
    let mut cnt = 1;
    for i in 0..len {
        pos[nums[i] as usize] = i;
    }
    for seq in sequences {
        for i in 0..seq.len() {
            exist = true;
            if seq[i] < 1 || seq[i] > len as i32 { return false; }
            if i == 0 { continue; }
            if pos[seq[i] as usize] <= pos[seq[i - 1] as usize] { return false; }
            // 首次出现，且 seq[I] 在 nums的位置，是前一个数字的后一个
            if flag[seq[i] as usize] == false && pos[seq[i] as usize] == pos[seq[i - 1] as usize] + 1 {
                cnt += 1;
                flag[seq[i] as usize] = true;
            }
        }
    }
    cnt == len && exist
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, sequences: Vec<Vec<i32>>) -> bool) {
        assert_eq!(func(vec![1, 2, 3], vec![vec![1, 2], vec![1, 3]]), false);
        assert_eq!(func(vec![1, 2, 3], vec![vec![1, 2]]), false);
        assert_eq!(func(vec![1, 2, 3], vec![vec![1, 2], vec![1, 3], vec![2, 3]]), true);
        assert_eq!(func(vec![4, 1, 5, 2, 6, 3], vec![vec![5, 2, 6, 3], vec![4, 1, 5, 2]]), true);
    }
    test(sequence_reconstruction);
}
