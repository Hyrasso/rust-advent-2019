// #[macro_use] extern crate itertools;
use std::fs;
use crate::solutions::Solution;
use std::iter;
use std::collections::HashMap;

pub struct Day11 {
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

#[derive(Debug)]
enum Color {
    BLACK,
    WHITE,
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

    fn step(&mut self, color: &Color) -> Option<IntcodeStepResult> {
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
                let color = match color {
                    Color::WHITE => 1,
                    Color::BLACK => 0
                };
                self.set(idests[0], color);
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


fn modulo(n: i32, m: i32) -> i32 {
    ((n % m) + m) % m
}

impl Solution for Day11 {
    fn solve(&self) {
        let text = fs::read_to_string(&self.file).expect("Error while opening input");
        let mut intcode = Intcode::new(&text);
        
        let mut map = HashMap::new();
        map.insert((0, 0), Color::WHITE);
        let mut pos = (0, 0);
        let mut dir = 0; // start up
        let mut painting = true;
        let rotations = vec![
            (0, 1), // up
            (1, 0), //right
            (0, -1), // down
            (-1, 0), //left
        ];

        loop {
            let color = map.get(&pos).unwrap_or(&Color::BLACK);
            let res = intcode.step(color);
            match res {
                Some(IntcodeStepResult::OUTPUT(out)) => {
                    if painting {
                        match out {
                            0 => map.insert(pos, Color::BLACK),
                            1 => map.insert(pos, Color::WHITE),
                            _ => panic!("Unknown color {}", out)
                        };
                        painting = false;
                    } else {
                        dir = dir + match out {
                            0 => -1,
                            1 => 1,
                            _ => panic!("Unknown rotation {}", out)
                        };
                        dir = modulo(dir, 4);
                        let t = rotations[dir as usize];
                        painting = true;
                        pos = (pos.0 + t.0, pos.1 + t.1);
                    }
                },
                Some(IntcodeStepResult::HALT) => break,
                _ => ()
            }
        }
        // println!("{}", map.len());
        let (width, height) = (50, 6);
        let mut im = vec!["  "; width * height];
        for (k, v) in map.iter() {
            // println!("{:?}: {:?}", k, v);
            im[(k.0 + -k.1 * width as i32) as usize] = match v {
                Color::BLACK => "  ",
                Color::WHITE => "██"
            };
        }
        for line in im.chunks(width) {
            for c in line {
                print!("{}", c);
            }
            println!("");
        }
    }
}