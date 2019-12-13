// #[macro_use] extern crate itertools;
use std::fs;
use crate::solutions::Solution;
use std::iter;
use std::collections::HashMap;

pub struct Day13 {
    pub file: String
}

struct Intcode {
    program: Vec<i128>,
    ip: usize,
    base: usize
}

enum IntcodeStepResult {
    OUTPUT(i128),
    HALT
}

#[derive(Debug, PartialEq)]
enum Tile {
    EMPTY,
    WALL,
    BLOCK,
    PADDLE,
    BALL
}

impl Intcode {
    fn get_addrs(&self, mode: i128, n_args: usize) -> Vec<usize> {
        // eprintln!("D {:?}: {:?} {:?} {:?} ", ip, mode, base, args);
        let mut arg_mode = mode;
        let mut res = vec![];
        let args_start = self.ip + 1;
        for (i, arg) in self.program[args_start..args_start + n_args].iter().enumerate() {
            let dest = match arg_mode % 10 {
                0 => *arg as usize, // position
                1 => self.ip + 1 + i as usize, // immediate
                2 => (self.base as i128 + *arg) as usize, // relative
                m => panic!("unknown mode {:?}", m)
            };
            res.push(dest);
            arg_mode /= 10;
        }
        res
    }

    fn get(&self, addr: usize) -> i128 {
        *self.program.get(addr).unwrap_or(&0)
    }

    fn set(&mut self, addr: usize, value: i128) {
        if addr >= self.program.len() {
            self.program.extend(iter::repeat(0).take(addr - self.program.len() + 1));
        }
        self.program[addr] = value;
    }

    fn get_mode_op(&self) -> (i128, i128) {
        (self.get(self.ip) / 100, self.get(self.ip) % 100)
    }

    fn step(&mut self, input: i128) -> Option<IntcodeStepResult> {
        let (mode, op) = self.get_mode_op();
        // eprintln!("D {}: {}", ip, op);
        match op {
            // Add a + b -> c
            1 => {
                let idests = self.get_addrs(mode, 3);
                self.set(idests[2], self.get(idests[0]) + self.get(idests[1]));
                self.ip += 4;
            },
            // Mul a * b -> c
            2 => {
                let idests = self.get_addrs(mode, 3);
                self.set(idests[2], self.get(idests[0]) * self.get(idests[1]));
                self.ip += 4;
            },
            // Get input -> a
            3 => {
                let idests = self.get_addrs(mode, 1);
                self.set(idests[0], input);
                self.ip += 2;
            },
            // set output -> a 
            4 => {
                let idests = self.get_addrs(mode, 1);
                // eprint!("{} ", program[idest]);
                let output = self.get(idests[0]);
                self.ip += 2;
                return Some(IntcodeStepResult::OUTPUT(output));
            },
            // JNZ set ip -> b if a is non zero
            5 => {
                let idests = self.get_addrs(mode, 2);
                if self.get(idests[0]) != 0 {
                    self.ip = self.get(idests[1]) as usize;
                } else {
                    self.ip += 3;
                }
            },
            // JZ set ip -> b if a is zero
            6 => {
                let idests = self.get_addrs(mode, 2);
                if self.get(idests[0]) == 0 {
                    self.ip = self.get(idests[1]) as usize;
                } else {
                    self.ip += 3;
                }
            },
            // LE 1 -> c if a < b, otherwise 0 -> c
            7 => {
                let idests = self.get_addrs(mode, 3);
                if self.get(idests[0]) < self.get(idests[1]) {
                    self.set(idests[2], 1);
                } else {
                    self.set(idests[2], 0);
                }
                self.ip += 4;
            },
            // EQ 1 -> c if a == b, otherwise 0 -> c
            8 => {
                let idests = self.get_addrs(mode, 3);
                if self.get(idests[0]) == self.get(idests[1]) {
                    self.set(idests[2], 1);
                } else {
                    self.set(idests[2], 0);
                }
                self.ip += 4;
            },
            9 => {
                let idests = self.get_addrs(mode, 1);
                // eprintln!("D SET BASE {:?}", program[idests[0]] + base as i128);
                self.base = (self.get(idests[0]) + self.base as i128) as usize;
                // eprintln!("D SET BASE {:?}", program[idests[0]] + base as i128);
                self.ip += 2;
            },
            99 => return Some(IntcodeStepResult::HALT),
            _ => {
                panic!("Unknown op code {}", op);
            }
        }
        None
    }

    fn new(program: &String) -> Self {
        Intcode {
            program: program.split(",").map(|op| op.parse::<i128>().unwrap()).collect(),
            ip: 0,
            base: 0
        }
    }
}


impl Solution for Day13 {
    fn solve(&self) {
        let text = fs::read_to_string(&self.file).expect("Error while opening input");
        let mut intcode = Intcode::new(&text);
        
        let mut map = HashMap::new();
        let mut output = vec![];
        let mut score = -1;
        let mut paddle = 0;
        let mut ball = 0;
        loop {
            let res = intcode.step(i128::signum(ball - paddle));
            match res {
                Some(IntcodeStepResult::OUTPUT(i)) => output.push(i),
                Some(IntcodeStepResult::HALT) => break,
                None => ()
            }
            if output.len() == 3 {
                let pos = (output[0], output[1]);
                if pos == (-1, 0) {
                    score = output[2];
                } else {
                    let tile = match output[2] {
                        0 => Tile::EMPTY,
                        1 => Tile::WALL,
                        2 => Tile::BLOCK,
                        3 => {
                            paddle = pos.0;
                            Tile::PADDLE
                        },
                        4 => {
                            ball = pos.0;
                            Tile::BALL
                        },
                        i => panic!("Unknown tile {}", i)
                    };
                    map.insert(pos, tile);
                }
                output.clear();
            }
            if !map.iter().any(|(_k, v)| v == &Tile::BLOCK) && map.len() > 2022 {
                println!("Blocks left {}", map.iter().filter(|(_k, v)| *v == &Tile::BLOCK).count());
                break;
            }
        }
        // println!("{:?}", intcode.program);
        println!("Blocks left {}", map.iter().filter(|(_k, v)| *v == &Tile::BLOCK).count());
        println!("{:?}", score);
    }
}