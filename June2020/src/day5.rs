use rand::Rng;

struct Solution {
    cummul: Vec<i32>,
    max: i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(weights: Vec<i32>) -> Self {
        let mut cummul = vec![];
        for w in weights {
            let last = *cummul.last().unwrap_or(&0);
            cummul.push(last + w);
        }
        
        Solution {
            max: *cummul.last().unwrap_or(&0),
            cummul: cummul,
        }
    }
    
    fn pick_index(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let pick = rng.gen_range(0, self.max);

        // Search for pick
        let mut i = 0;
        let mut iter = self.cummul.iter();
        loop {
            match iter.next() {
                Some(c) if *c <= pick => {
                    i += 1;
                },
                _ => break,
            } 
        }

        return i as i32
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(w);
 * let ret_1: i32 = obj.pick_index();
 */
fn test(weights: Vec<i32>, n: i32) {
    println!("Testing weights: {:?}", weights);
    let obj = Solution::new(weights);

    for i in 0..n {
        let ret = obj.pick_index();
        print!("{} ", ret);
        if (i + 1) % 10 == 0 {
            println!();
        }
    }
    println!();
}


 pub fn run() {
    println!("Day 5: Random Pick with Weights");

    test(vec![1], 2);
    test(vec![1, 3], 10);
    test(vec![2, 2, 6], 10);
}