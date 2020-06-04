struct Solution;
impl Solution {
    pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
        // O(N) space, store cost differential
        let mut cost_diff: Vec<(usize,i32)> = vec![];
        for (i, pc) in costs.iter().enumerate() {
            cost_diff.push((i, pc[0] - pc[1]));
        }

        // O(NlogN) sort
        cost_diff.sort_unstable_by_key(|k| k.1);

        let n = costs.len() as i32 / 2;
        let mut total_cost = 0;
        for (i, (person, _)) in cost_diff.iter().enumerate() {
            if (i as i32) < n {
                // Goes to city A
                total_cost += costs[*person][0];
            } else {
                // Goes to city B
                total_cost += costs[*person][1];
            }
        }

        total_cost
    }
}

fn test1() {
    let costs = vec![vec![10,20], vec![30,200], vec![400,50], vec![30,20]];
    let expected = 10 + 30 + 50 + 20;
    let ret = Solution::two_city_sched_cost(costs);

    if ret == expected {
        println!("Passed test 1!");
    } else {
        println!("Expected {}, got {} for test 1", expected, ret);
    }
}

fn test2() {
    let costs = vec![vec![10,50], vec![30,200], vec![40,200], vec![30,40]];
    let expected = 50 + 30 + 40 + 40;
    let ret = Solution::two_city_sched_cost(costs);

    if ret == expected {
        println!("Passed test 2!");
    } else {
        println!("Expected {}, got {} for test 2", expected, ret);
    }
}

fn test3() {
    let costs = vec![vec![10,100], vec![20,100], vec![80,90], vec![70,80]];
    let expected = 10 + 20 + 90 + 80;
    let ret = Solution::two_city_sched_cost(costs);

    if ret == expected {
        println!("Passed test 3!");
    } else {
        println!("Expected {}, got {} for test 3", expected, ret);
    }
}

pub fn run() {
    println!("Day 3: Two City Scheduling");
    
    test1();
    test2();
    test3();
}