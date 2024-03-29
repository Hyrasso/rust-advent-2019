// #[macro_use] extern crate itertools;
use std::fs;
use crate::solutions::Solution;
use itertools::Itertools;
use std::thread;
use std::sync::mpsc;
pub struct Day7 {}

fn get_addr(mode:i32, ip: usize, value: i32) -> usize {
    if mode % 10 == 0 {
        value as usize
    } else {
        ip
    }
}

fn run(text: &str, input: mpsc::Receiver<i32>, output: mpsc::Sender<i32>) {
    // println!("Starting amp");
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
                program[idest] = input.recv().unwrap();
                ip += 2;
            },
            // set output -> a 
            4 => {
                let idest = get_addr(mode, ip + 1, program[ip + 1]);
                // eprint!("{} ", program[idest]);
                let res = output.send(program[idest]);
                if res.is_err() {
                    println!("{}", program[idest]);
                }
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
    let res = input.try_recv();
    if res.is_ok() {
        println!("{}", res.unwrap());
    }
}

impl Solution for Day7 {
    fn solve(&self) {
        let text = fs::read_to_string("inputs/7-1.txt").expect("Error while opening input");
        for perm in (5..10).permutations(5) {
            let mut senders = vec![];
            let mut receivers = vec![];
            for _ in 0..5 {
                let (tx, rx) = mpsc::channel();
                senders.push(tx);
                receivers.push(rx);
            }
            // phase config
            senders[0].send(perm[0]).unwrap();
            senders[1].send(perm[1]).unwrap();
            senders[2].send(perm[2]).unwrap();
            senders[3].send(perm[3]).unwrap();
            senders[4].send(perm[4]).unwrap();
            // init input
            let itx0 = mpsc::Sender::clone(&senders[0]);
            itx0.send(0).unwrap();
            let mut progs = vec![];
            // Amp A
            for _ in 0..4 {
                let instr = text.clone();
                // found how to take ownership of vec element 
                // finally, loops are great
                let irx = receivers.remove(0);
                let otx = senders.remove(1);
                progs.push(thread::spawn(move || {
                    run(&instr, irx, otx);
                }));
            }
            // Amp E
            let instr = text.clone();
            let irx = receivers.remove(0);
            let otx = senders.remove(0);
            progs.push(thread::spawn(move || {
                run(&instr, irx, otx);
            }));
            progs.into_iter().for_each(|t| t.join().unwrap());
        }
    }
}