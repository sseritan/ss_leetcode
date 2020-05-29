struct NumMatrix {
    row_sums: Vec<Vec<i32>>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {

    // Only store cummulative for rows
    // O(N^2) memory and O(N^2) initialize time
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let mut sums = vec![];
        for row in matrix {
            let mut sum = 0;
            let mut row_sum = vec![0];

            for elem in row {
                sum += elem;
                row_sum.push(sum);
            }

            sums.push(row_sum);
        }

        NumMatrix {
            row_sums: sums
        }
    }
    
    // Calculate interval for each row based on difference in cummulative
    // Is only O(N) per request over O(N^2) brute force
    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let mut sum = 0;
        for r in row1..row2+1 {
            let row_sum = &self.row_sums[r as usize];
            sum += row_sum[(col2 + 1) as usize] - row_sum[col1 as usize];
        }
        sum
    }
}

/**
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);
 */

fn test(obj: &NumMatrix, r1: i32, c1: i32, r2: i32, c2: i32, expect: i32) {
    print!("Testing region ({},{}) to ({}, {})... ", r1, c1, r2, c2);
    let ret = obj.sum_region(r1, c1, r2, c2);
    if ret != expect {
        println!("Failed, expected {} but got {}", expect, ret);
    } else {
        println!("Passed!");
    }
}

fn main() {
    let matrix = vec![
        vec![3, 0, 1, 4, 2],
        vec![5, 6, 3, 2, 1],
        vec![1, 2, 0, 1, 5],
        vec![4, 1, 0, 1, 7],
        vec![1, 0, 3, 0, 5]];

    let obj = NumMatrix::new(matrix);

    test(&obj, 2, 1, 4, 3, 8);
    test(&obj, 1, 1, 2, 2, 11);
    test(&obj, 1, 2, 2, 4, 12);
    test(&obj, 4, 4, 4, 4, 5);

    let obj2 = NumMatrix::new(vec![vec![-1]]);
    test(&obj2, 0, 0, 0, 0, -1);
}
