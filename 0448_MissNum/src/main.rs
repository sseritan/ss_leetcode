struct Solution;
impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        // One pass to flip signs for numbers we have
        for i in 0..nums.len() {
            let ival = (nums[i].abs() - 1) as usize;
            if nums[ival] > 0 {
                nums[ival] *= -1;
            }
        }

        // One pass to find positive values (i.e. missing)
        let mut missing = vec![];
        for i in 0..nums.len() {
            if nums[i] > 0 {
                missing.push(i as i32 + 1);
            }
        }
        missing
    }
}

fn test(vec: Vec<i32>, expect: Vec<i32>) {
    print!("Testing {:?}... ", vec);

    let missing = Solution::find_disappeared_numbers(vec);

    if missing != expect {
        println!("Expected {:?}, found {:?}", expect, missing);
    } else {
        println!("Passed!");
    }
}

fn main() {
    test(vec![4, 3, 2, 7, 8, 2, 3, 1], vec![5, 6]);
}
