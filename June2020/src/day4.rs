struct Solution;
impl Solution {
    // O(N) time, O(1) space
    pub fn reverse_string(s: &mut Vec<char>) {
        let size = s.len() as i32;

        for i in 0..size/2 {
            let irev = (size - i - 1) as usize;
            let i = i as usize;
            let temp = s[irev];
            s[irev] = s[i];
            s[i] = temp;
        }
    }
} 

pub fn run() {
    println!("Day 4: Reverse String with O(1) Memory");

    let mut odd = vec!['H','e','l','l','o'];
    print!("Reversing {:?}... ", odd);
    Solution::reverse_string(&mut odd);
    println!("Got {:?}", odd);

    let mut even = vec!['W','o','r','l','d', '!'];
    print!("Reversing {:?}... ", even);
    Solution::reverse_string(&mut even);
    println!("Got {:?}", even);
}