struct Solution;
impl Solution {
    // Extra stipulations: Use read-only array, O(1) space, and no worse than O(N^2) time
    // Second attempt: Floyd's Tortoise and Hare
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut slow = nums[nums[0] as usize];
        let mut fast = nums[nums[nums[0] as usize] as usize];

        // Phase 1: Hare moves at double speed until it meets Tortoise
        while slow != fast {
            slow = nums[slow as usize];
            fast = nums[nums[fast as usize] as usize];
        }

        // Phase 2: Tortoise restarts and Hare moves at slow speed
        // They should meet at entrance to cycle, which is duplicate
        slow = nums[0];
        while slow != fast {
            slow = nums[slow as usize];
            fast = nums[fast as usize];
        }

        slow
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
