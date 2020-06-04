struct Solution;
impl Solution {
    pub fn generate_abbreviations(word: String) -> Vec<String> {
        let num_abbrevs = 2i32.pow(word.len() as u32); // 2^N choices (char or num)

        let mut abbrevs = vec![];
        for i in 1..num_abbrevs+1 {
            let mut abbr = String::new();
            let mut num = 0; // Number of subsequent replacements
            let mut bitmask = 1;

            for c in word.chars() {
                if i & bitmask > 0 {
                    num += 1;
                } else {
                    if num > 0 {
                        abbr += &num.to_string();
                        num = 0;
                    }
                    abbr.push(c);
                }

                bitmask = 2*bitmask;
            }
            if num > 0 { // trailing num
                abbr += &num.to_string();
            }
            abbrevs.push(abbr);
        }
        abbrevs
    }
}

fn main() {
    let word = String::from("word");
    println!("Finding abbreviations of {}", word);
    let abbrevs = Solution::generate_abbreviations(word);
    println!("Found abbreviations: {:?}", abbrevs);
}
