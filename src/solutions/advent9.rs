// #[macro_use] extern crate itertools;
use std::fs;
use crate::solutions::Solution;
use std::sync::mpsc;
use std::iter;

pub struct Day9 {
    pub file: String
}

fn get_addrs(mode: i128, ip: usize, base: usize, args: &[i128]) -> Vec<usize> {
    // eprintln!("D {:?}: {:?} {:?} {:?} ", ip, mode, base, args);
    let mut arg_mode = mode;
    let mut res = vec![];
    for (i, arg) in args.iter().enumerate() {
        let dest = match arg_mode % 10 {
            0 => *arg as usize, // position
            1 => ip + i as usize, // immediate
            2 => (base as i128 + *arg) as usize, // relative
            m => panic!("unknown mode {:?}", m)
        };
        res.push(dest);
        arg_mode /= 10;
    }
    res
}

fn run(text: &str, input: mpsc::Receiver<i128>, output: mpsc::Sender<i128>) {
    // println!("Starting amp");
    let mut program: Vec<_> = text.split(",").map(|s| s.parse::<i128>().unwrap()).collect();
    program.extend(iter::repeat(0).take(500));
    let mut ip = 0;
    let mut base = 0;
    loop {
        let op = program[ip] % 100;
        // eprintln!("D {}: {}", ip, op);
        let mode = program[ip] / 100;
        match op {
            // Add a + b -> c
            1 => {
                let idests = get_addrs(mode, ip+1, base, &program[ip+1..=ip+3]);
                program[idests[2]] = program[idests[0]] + program[idests[1]];
                ip += 4;
            },
            // Mul a + b -> c
            2 => {
                let idests = get_addrs(mode, ip+1, base, &program[ip+1..=ip+3]);
                program[idests[2]] = program[idests[0]] * program[idests[1]];
                ip += 4;
            },
            // Get input -> a
            3 => {
                let idests = get_addrs(mode, ip+1, base, &program[ip + 1..ip+2]);
                program[idests[0]] = input.recv().unwrap();
                ip += 2;
            },
            // set output -> a 
            4 => {
                let idests = get_addrs(mode, ip + 1, base, &program[ip + 1..ip+2]);
                // eprint!("{} ", program[idest]);
                let res = output.send(program[idests[0]]);
                if res.is_err() {
                    println!("D out closed, tried to send {}", program[idests[0]]);
                }
                ip += 2;
            },
            // JNZ set ip -> b if a is non zero
            5 => {
                let idests = get_addrs(mode, ip + 1, base, &program[ip+1..=ip+2]);
                if program[idests[0]] != 0 {
                    ip = program[idests[1]] as usize;
                } else {
                    ip += 3;
                }
            },
            // JZ set ip -> b if a is zero
            6 => {
                let idests = get_addrs(mode, ip + 1, base, &program[ip+1..=ip+2]);
                if program[idests[0]] == 0 {
                    ip = program[idests[1]] as usize;
                } else {
                    ip += 3;
                }
            },
            // LE 1 -> c if a < b, otherwise 0 -> c
            7 => {
                let idests = get_addrs(mode, ip + 1, base, &program[ip + 1..=ip + 3]);
                if program[idests[0]] < program[idests[1]] {
                    program[idests[2]] = 1;
                } else {
                    program[idests[2]] = 0;
                }
                ip += 4;
            },
            // EQ 1 -> c if a == b, otherwise 0 -> c
            8 => {
                let idests = get_addrs(mode, ip + 1, base, &program[ip + 1..=ip + 3]);
                if program[idests[0]] == program[idests[1]] {
                    program[idests[2]] = 1;
                } else {
                    program[idests[2]] = 0;
                }
                ip += 4;
            },
            9 => {
                let idests = get_addrs(mode, ip + 1, base, &program[ip + 1..ip + 2]);
                // eprintln!("D SET BASE {:?}", program[idests[0]] + base as i128);
                base = (program[idests[0]] + base as i128) as usize;
                // eprintln!("D SET BASE {:?}", program[idests[0]] + base as i128);
                ip += 2;
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
        println!("D unread input {}", res.unwrap());
    }
}

impl Solution for Day9 {
    fn solve(&self) {
        let text = fs::read_to_string(&self.file).expect("Error while opening input");
        let (itx, irx) = mpsc::channel();
        let (otx, orx) = mpsc::channel();
        itx.send(2).unwrap();
        run(&text, irx, otx);
        while let Ok(res) = orx.recv() {
            println!("{:?}", res);
        }
    }
}