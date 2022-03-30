//! 你完成的完整对局数

pub fn number_of_rounds(start_time: String, finish_time: String) -> i32 {
    let mut start = start_time[..2].parse::<i32>().unwrap() * 60 + start_time[3..].parse::<i32>().unwrap();
    let mut finish = finish_time[..2].parse::<i32>().unwrap() * 60 + finish_time[3..].parse::<i32>().unwrap();
    if start > finish {
        finish += 24 * 60;
    }
    if start % 15 != 0 {
        start += 15;
    }
    start = start / 15;
    finish = finish / 15;
    (finish - start).max(0)
}

fn main() {
    assert_eq!(number_of_rounds(String::from("09:31"), String::from("10:14")), 1);
    assert_eq!(number_of_rounds(String::from("21:30"), String::from("03:00")), 22);
}
