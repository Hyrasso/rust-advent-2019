use crate::solutions::Solution;

pub struct Day4 {}

impl Solution for Day4 {
    fn solve(&self) {
        let mut count = 0;
        for i in 278384..=824795 {
            let n = format!("{}", i);
            let valid = n.chars().zip(n[1..].chars()).fold(true, |acc, (c1, c2)| acc && (c1 <= c2))
                        && n.chars().zip(n[1..].chars()).fold(false, |acc, (c1, c2)| acc || (c1 == c2));
            if valid {
                let mut occ = vec![0, 0, 0, 0, 0];
                n.chars().zip(n[1..].chars()).fold(0, |acc, (c1, c2)| {
                    if c1 == c2 {
                        occ[acc] += 1;
                        return acc + 1;
                    } else {
                        return 0;
                    }
                });
                if occ[0] > occ[1] {
                    count += 1;
                }
            } 
            
        }
        println!("Total: {}", count);
    }
}
