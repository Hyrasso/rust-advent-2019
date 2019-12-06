use std::fs;
use crate::solutions::Solution;

pub struct Day2 {}

fn run(text: String, noun: i32, verb: i32) -> i32 {
    let mut program: Vec<_> = text.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
    program[1] = noun;
    program[2] = verb;
    let mut ip = 0;
    loop {
        let dest = program[ip + 3] as usize;
        match program[ip] {
            1 => program[dest] = program[program[ip + 1] as usize] + program[program[ip + 2] as usize],
            2 => program[dest] = program[program[ip + 1] as usize] * program[program[ip + 2] as usize],
            99 => break,
            _ => println!("Unknown op code {}", program[ip])
        }
        ip += 4;
    }
    program[0]
}

impl Solution for Day2 {
    fn solve(&self) {
        let text = fs::read_to_string("inputs/2-1.txt").expect("Error while opening input");
        // let res = run(text, 12, 2);
        // println!("{}", res);
        let target = 19690720;
        for noun in 0..100 {
            for verb in 0..100 {
                let res = run(text.clone(), noun, verb);
                if res == target{
                    println!("Noun {}, verb {}, answer {}", noun, verb, noun * 100 + verb);
                    return;
                }
            }
        }
        println!("No solution found for {}", target);
    }
}