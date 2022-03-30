//! 整数转换英文表示

const UNIT: [&str; 20] = ["Zero", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight",
    "Nine", "Ten", "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen", "Sixteen", "Seventeen",
    "Eighteen", "Nineteen"];
const TEN_UNIT: [&str; 8] = ["Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty",
    "Ninety"];

pub fn number_to_words(num: i32) -> String {
    if num == 0 { return String::from("Zero"); }
    let mut ans = vec![];
    fn three(num: i32) -> String {
        if num == 0 { return String::from(""); }
        let mut ans = vec![];
        let percentile = num / 100;
        if percentile > 0 {
            ans.push(UNIT[percentile as usize]);
            ans.push("Hundred");
        }
        let last2 = num % 100;
        if last2 == 0 {} else if last2 < 20 {
            ans.push(UNIT[last2 as usize]);
        } else {
            let ten = last2 / 10;
            let unit = last2 % 10;
            ans.push(TEN_UNIT[ten as usize - 2]);
            if unit != 0 {
                ans.push(UNIT[unit as usize]);
            }
        }
        ans.join(" ")
    }

    let billion = num / (1e9 as i32);
    if billion > 0 {
        ans.push(three(billion));
        ans.push(String::from("Billion"));
    }
    let million = num % (1e9 as i32) / (1e6 as i32);
    if million > 0 {
        ans.push(three(million));
        ans.push(String::from("Million"));
    }
    let thousand = num % (1e6 as i32) / (1e3 as i32);
    if thousand > 0 {
        ans.push(three(thousand));
        ans.push(String::from("Thousand"));
    }
    let left = num % (1e3 as i32);
    if left > 0 {
        ans.push(three(left));
    }

    ans.join(" ")
}

fn main() {
    assert_eq!(number_to_words(123), String::from("One Hundred Twenty Three"));
    assert_eq!(number_to_words(12345), String::from("Twelve Thousand Three Hundred Forty Five"));
    assert_eq!(number_to_words(1234567), String::from("One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"));
}
