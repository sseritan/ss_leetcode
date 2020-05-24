struct Solution;
impl Solution {
    // Extra stipulations: Use read-only array, O(1) space, and no worse than O(N^2) time
    // First attempt: Run through array, looking for each number
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        for target in 1..nums.len() {
            let mut count = 0;
            for n in &nums {
                if target as i32 == *n {
                    count += 1;
                }
            }

            if count > 1 {
                return target as i32
            }
        }

        -1 // Dummy return
    }
}

fn main() {
    let input = vec![1,3,4,2,2];
    println!("Testing {:?}...", input);
    println!("Expected {}, found {}", 2, Solution::find_duplicate(input));

    let input = vec![3,1,3,4,2];
    println!("Testing {:?}...", input);
    println!("Expected {}, found {}", 3, Solution::find_duplicate(input));
}
