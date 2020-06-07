struct Solution;
impl Solution {
    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // Sort descending in height and then ascending in index
        people.sort_unstable_by(|a, b|
            b[0].cmp(&a[0]).then_with(||
                a[1].cmp(&b[1])
            )
        );

        // Now insert with second index as position in new queue
        // Since later insertions always have less than or equal height,
        // guaranteed to give the properties we want
        let mut sorted = vec![];
        for p in people {
            sorted.insert(p[1] as usize, p);
        }
        sorted
    }
}

fn test() {
    let people = vec![vec![7,0],vec![4,4],vec![7,1],vec![5,0],vec![6,1],vec![5,2]];
    println!("Testing with: {:?}", people);

    let sorted = Solution::reconstruct_queue(people);
    println!("Sorted queue: {:?}", sorted);
}

 pub fn run() {
    println!("Day 5: Reconstruct Queue by Height");

    test();
}