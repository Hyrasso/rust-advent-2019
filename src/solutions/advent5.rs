use std::fs;
use crate::solutions::Solution;

pub struct Day5 {}

fn get_addr(mode:i32, ip: usize, value: i32) -> usize {
    if mode % 10 == 0 {
        value as usize
    } else {
        ip
    }
}

fn run(text: String, input: i32) {
    let mut program: Vec<_> = text.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
    let mut ip = 0;
    loop {
        let op = program[ip] % 100;
        // eprintln!("D {}: {}", ip, op);
        let mode = program[ip] / 100;
        match op {
            // Add a + b -> c
            1 => {
                let idest = get_addr(mode / 100, ip + 3, program[ip + 3]);
                program[idest] = program[get_addr(mode, ip + 1, program[ip + 1])] + program[get_addr(mode / 10, ip + 2, program[ip + 2])];
                ip += 4;
            },
            // Mul a + b -> c
            2 => {
                let idest = get_addr(mode / 100, ip + 3, program[ip + 3]);
                program[idest] = program[get_addr(mode, ip + 1, program[ip + 1])] * program[get_addr(mode / 10, ip + 2, program[ip + 2])];
                ip += 4;
            },
            // Get input -> a
            3 => {
                let idest = get_addr(mode, ip + 1, program[ip + 1]);
                program[idest] = input;
                ip += 2;
            },
            // set output -> a 
            4 => {
                let idest = get_addr(mode, ip + 1, program[ip + 1]);
                println!("{}", program[idest]);
                ip += 2;
            },
            // JNZ set ip -> b if a is non zero
            5 => {
                if program[get_addr(mode, ip + 1, program[ip + 1])] != 0 {
                    ip = program[get_addr(mode / 10, ip + 2, program[ip + 2])] as usize;
                } else {
                    ip += 3;
                }
            },
            // JZ set ip -> b if a is zero
            6 => {
                if program[get_addr(mode, ip + 1, program[ip + 1])] == 0 {
                    ip = program[get_addr(mode / 10, ip + 2, program[ip + 2])] as usize;
                } else {
                    ip += 3;
                }
            },
            // LE 1 -> c if a < b, otherwise 0 -> c
            7 => {
                let idest = get_addr(mode / 100, ip + 3, program[ip + 3]);
                if program[get_addr(mode, ip + 1, program[ip + 1])] < program[get_addr(mode / 10, ip + 2, program[ip + 2])] {
                    program[idest] = 1;
                } else {
                    program[idest] = 0;
                }
                ip += 4;
            },
            // EQ 1 -> c if a == b, otherwise 0 -> c
            8 => {
                let idest = get_addr(mode / 100, ip + 3, program[ip + 3]);
                if program[get_addr(mode, ip + 1, program[ip + 1])] == program[get_addr(mode / 10, ip + 2, program[ip + 2])] {
                    program[idest] = 1;
                } else {
                    program[idest] = 0;
                }
                ip += 4;
            },
            99 => break,
            _ => {
                println!("Unknown op code {}", op);
                break;
            }
        }
    }
}

impl Solution for Day5 {
    fn solve(&self) {
        let text = fs::read_to_string("inputs/5-1.txt").expect("Error while opening input");
        // run(text, 1);
        run(text, 5);
    }
}