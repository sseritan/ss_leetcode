struct Solution;
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut count: i32 = 0;
        let mut iter = s.char_indices().rev();
        let mut start = false;
        loop{
            if let Some((i, c)) = iter.next() {
                if !start && c != ' ' {
                    start = true;
                    count += 1;
                } else if !start && c == ' ' {
                    continue;
                } else if start && c != ' ' {
                    count += 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        count
    }
}

fn test(input: String) {
    println!("Input: {}", input);
    let output = Solution::length_of_last_word(input);
    println!("Chars in last word: {}", output);
}

fn main() {
    test(String::from("Hello World"));
    test(String::from("AllOneWord"));
    test(String::from("a"));
    test(String::from("ends in space "));
}