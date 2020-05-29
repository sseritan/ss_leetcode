struct Solution;
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut iter = s.char_indices().rev();
        let mut start = -1;
        let mut end = -1;
        loop{
            match iter.next() {
                Some((i, c)) => {
                    if c != ' ' && start == -1 {
                        start = i as i32;
                    } else if c == ' ' && start != -1 {
                        end = i as i32;
                        break;
                    } else {
                        continue;
                    }
                },
                None => break
            }
        }
        
        if end == s.len() as i32 {
            return 0
        }

        start - end
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