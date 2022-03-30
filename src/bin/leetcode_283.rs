pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut p = 0;
    for i in 0..nums.len() {
        if nums[i] != 0 {
            if p != i { nums.swap(p, i); }
            p += 1;
        }
    }
}

fn main() {
    let mut s = vec![0, 1, 0, 3, 12];
    move_zeroes(&mut s);
    println!("{:?}", s)
}
